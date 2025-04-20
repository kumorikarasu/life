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
        // Setup a tokio task that will run the decay function every minute or so
        /*
        actix_web::rt::spawn(async move {
            loop {
                actix_web::rt::time::sleep(std::time::Duration::from_secs(60)).await;
                SimService::run_decay().await.unwrap();
            }
        });
        */
        Ok(())
    }

    /*
    pub async fn run_decay() -> Result<(), sea_orm::DbErr> {
        let stats = sim_stat::Entity::find()
            .all(&self.db)
            .await?;


        Ok(())
    }
    */

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
                sim.1.iter().map(|stat| crate::dto::sim_stat::Model {
                    name: stat.name.to_owned(),
                    value: stat.value.to_owned(),
                    decay_rate: stat.decay_rate,
                }).collect()
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
                stats.iter().map(|stat| crate::dto::sim_stat::Model {
                    name: stat.name.to_owned(),
                    value: stat.value.to_owned(),
                    decay_rate: stat.decay_rate,
                }).collect()
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
            decay_rate: Set(Some(0.0)),
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

    pub async fn delete_stat(&self, sim_id: u64, name: String) -> Result<(), sea_orm::DbErr> {
        sim_stat::Entity::delete_many()
            .filter(sim_stat::Column::SimId.eq(sim_id as i32))
            .filter(sim_stat::Column::Name.eq(name))
            .exec(&self.db)
            .await?;

        Ok(())
    }

}

