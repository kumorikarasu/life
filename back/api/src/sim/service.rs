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

    pub async fn get_sim(&self, id: u64) -> Result<Option<(sim::Model, Vec<sim_stat::Model>)>, sea_orm::DbErr> {
        //Sim::find().find_with_related(Stat).where_column(Sim::Id, id).one(&self.db).unwrap()
        let mut sim = Sim::find()
            .find_with_related(Stat)
            .filter(sim::Column::Id.eq(id as i32))
            .all(&self.db)
            .await?;

        if sim.len() == 0 {
            return Ok(None)
        } else {
            let sim = sim.remove(0);
            Ok(Some(sim))
        }
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
        }.save(&self.db).await.unwrap().try_into_model()
    }

    pub async fn post_stat(&self, sim_id: u64, stat: sim_stat::Model) -> Result<sim_stat::Model, sea_orm::DbErr> {
        sim_stat::ActiveModel {
            id: Set(stat.id.to_owned()),
            sim_id: Set(sim_id as i32),
            name: Set(stat.name.to_owned()),
            value: Set(stat.value.to_owned()),
        }.save(&self.db).await.unwrap().try_into_model()
    }

}

