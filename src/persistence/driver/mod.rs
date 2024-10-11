use crate::config::PersistenceDriverSelection;
use crate::persistence::driver::in_memory::InMemoryPersistenceDriver;
use crate::persistence::driver::postgres::PostgresPersistenceDriver;
use crate::persistence::driver_trait::PersistenceDriver;
use std::hash::Hash;

pub mod in_memory;
pub mod postgres;

pub async fn load_driver_for_config<'a, I, P>(
    config: PersistenceDriverSelection,
) -> Box<dyn PersistenceDriver<Item = I, PrimaryKey = P> + Send + Sync + 'a>
where
    I: Send + Clone + 'a,
    P: Hash + Eq + Clone + Send + 'a,
{
    match config {
        PersistenceDriverSelection::InMemory => Box::new(InMemoryPersistenceDriver::<P, I>::new()),
        PersistenceDriverSelection::Postgres(postgres_config) => Box::new(
            PostgresPersistenceDriver::<P, I>::new_for_config(postgres_config).await,
        ),
    }
}
