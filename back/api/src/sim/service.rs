use sea_orm::sqlx::types::chrono::{DateTime, Utc};
use sea_orm::*;

// use crate::sim::entity::{ Sim, Stat };
use crate::entities::{ sim, sim::Entity as Sim };
use crate::entities::{ sim_stat, sim_stat::Entity as Stat};

#[derive(Clone)]
pub struct SimService {
    db: DatabaseConnection
}

impl SimService {
    pub(crate) fn new(db: DatabaseConnection) -> SimService {
        SimService { 
            db,
        }
    }

    pub async fn setup(&self) -> Result<(), sea_orm::DbErr> {
        // No background decay process needed as we're applying decay on-the-fly
        Ok(())
    }

    // Helper method to apply decay to a stat based on elapsed time
    fn apply_decay(&self, stat: &sim_stat::Model) -> crate::dto::sim_stat::Model {
        let now = Utc::now().naive_utc();
        let elapsed_seconds = (now - stat.timestamp).num_seconds();
        
        let mut value = stat.value;
        
        // Apply decay if decay_rate is set and elapsed time is positive
        if let Some(decay_rate) = stat.decay_rate {
            if decay_rate > 0.0 && elapsed_seconds > 0 {
                let decay_amount = decay_rate * elapsed_seconds as f32;
                // Ensure value doesn't go below 0
                value = (value - decay_amount).max(0.0);
            }
        }
        
        crate::dto::sim_stat::Model {
            name: stat.name.to_owned(),
            value,
            decay_rate: stat.decay_rate,
        }
    }

    pub async fn get_sim(&self, id: u64) -> Result<crate::dto::sim::Model, sea_orm::DbErr> {
        //Sim::find().find_with_related(Stat).where_column(Sim::Id, id).one(&self.db).unwrap()
        let mut sim = Sim::find()
            .find_with_related(Stat)
            .filter(sim::Column::Id.eq(id as i32))
            .all(&self.db)
            .await?;

        if sim.len() == 0 {
            return Err(sea_orm::DbErr::RecordNotFound("Sim".to_owned()))
        } else {
            let sim = sim.remove(0);
            Ok(crate::dto::sim::Model::new(
                sim.0.id,
                sim.0.name,
                sim.0.user_id,
                sim.1.iter().map(|stat| self.apply_decay(stat)).collect()
            ))
        }
    }

    pub async fn get_user_sims(&self, user_id: i32) -> Result<Vec<crate::dto::sim::Model>, sea_orm::DbErr> {
        let sims = Sim::find()
            .find_with_related(Stat)
            .filter(sim::Column::UserId.eq(user_id))
            .all(&self.db)
            .await?;

        let result = sims.into_iter().map(|(sim, stats)| {
            crate::dto::sim::Model::new(
                sim.id,
                sim.name,
                sim.user_id,
                stats.iter().map(|stat| self.apply_decay(stat)).collect()
            )
        }).collect();

        Ok(result)
    }

    pub async fn post_sim(&self, sim: sim::Model) -> Result<sim::Model, sea_orm::DbErr>{
        // Lookup the sim by id
        let lookup_sim = Sim::find_by_id(sim.id).one(&self.db).await?;

        let id = match lookup_sim {
            Some(sim) => Set(sim.id),
            None => NotSet,
        };

        sim::ActiveModel {
            id,
            name: Set(sim.name.to_owned()),
            user_id: Set(sim.user_id),
        }.save(&self.db).await.unwrap().try_into_model()
    }

    pub async fn post_stat(&self, sim_id: u64, stat: crate::dto::sim_stat::Model) -> Result<crate::dto::sim_stat::Model, sea_orm::DbErr> {
        let sim = sim_stat::ActiveModel {
            sim_id: Set(sim_id as i32),
            name: Set(stat.name.to_owned()),
            value: Set(stat.value.to_owned()),
            decay_rate: Set(stat.decay_rate),
            timestamp: Set(Utc::now().naive_utc()),
        };

        sim_stat::Entity::insert(sim)
        .on_conflict(
            sea_query::OnConflict::columns(vec![sim_stat::Column::Name, sim_stat::Column::SimId])
            .update_columns(vec![sim_stat::Column::Name, sim_stat::Column::SimId])
            .value(sim_stat::Column::Value, stat.value.to_owned())
            .value(sim_stat::Column::DecayRate, stat.decay_rate)
            .value(sim_stat::Column::Timestamp, Utc::now().naive_utc())
            .to_owned())
        .exec(&self.db)
        .await?;

        Ok(stat)
    }

    pub async fn update_stat(&self, sim_id: u64, stat: crate::dto::sim_stat::Model) -> Result<crate::dto::sim_stat::Model, sea_orm::DbErr> {
        // Find the existing stat
        let existing_stat = sim_stat::Entity::find()
            .filter(sim_stat::Column::SimId.eq(sim_id as i32))
            .filter(sim_stat::Column::Name.eq(stat.name.to_owned()))
            .one(&self.db)
            .await?;
        
        // Check if the stat exists
        if existing_stat.is_none() {
            return Err(sea_orm::DbErr::RecordNotFound(format!("Stat {} not found for sim {}", stat.name, sim_id)));
        }
        
        // Update the stat
        let mut stat_model: sim_stat::ActiveModel = existing_stat.unwrap().into();
        
        stat_model.value = Set(stat.value.to_owned());
        stat_model.decay_rate = Set(stat.decay_rate);
        stat_model.timestamp = Set(Utc::now().naive_utc());
        
        let updated_stat = stat_model.update(&self.db).await?;
        
        Ok(crate::dto::sim_stat::Model {
            name: updated_stat.name,
            value: updated_stat.value,
            decay_rate: updated_stat.decay_rate,
        })
    }

    pub async fn delete_stat(&self, sim_id: u64, name: String) -> Result<(), sea_orm::DbErr> {
        sim_stat::Entity::delete_many()
            .filter(sim_stat::Column::SimId.eq(sim_id as i32))
            .filter(sim_stat::Column::Name.eq(name))
            .exec(&self.db)
            .await?;

        Ok(())
    }

    pub async fn delete_sim(&self, sim_id: u64, user_id: i32) -> Result<(), sea_orm::DbErr> {
        // First, delete all stats associated with this sim
        sim_stat::Entity::delete_many()
            .filter(sim_stat::Column::SimId.eq(sim_id as i32))
            .exec(&self.db)
            .await?;
        
        // Then delete the sim itself, ensuring it belongs to the correct user
        let result = sim::Entity::delete_many()
            .filter(sim::Column::Id.eq(sim_id as i32))
            .filter(sim::Column::UserId.eq(user_id))
            .exec(&self.db)
            .await?;
        
        if result.rows_affected == 0 {
            return Err(sea_orm::DbErr::RecordNotFound("Sim not found or does not belong to user".to_owned()));
        }
        
        Ok(())
    }

}

