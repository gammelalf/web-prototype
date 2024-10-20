use framework_core::module::{InitError, Module, PostInitError, PreInitError};
use rorm::{Database, DatabaseConfiguration, DatabaseDriver};

pub struct Db(pub Database);

impl Module for Db {
    type PreInit = DatabaseConfiguration;

    async fn pre_init() -> Result<Self::PreInit, PreInitError> {
        Ok(DatabaseConfiguration {
            driver: DatabaseDriver::SQLite {
                filename: ":memory:".to_string(),
            },
            min_connections: 1,
            max_connections: 1,
            disable_logging: None,
            statement_log_level: None,
            slow_statement_log_level: None,
        })
    }

    type Dependencies = ();

    async fn init(config: Self::PreInit, (): &mut Self::Dependencies) -> Result<Self, InitError> {
        Ok(Self(Database::connect(config).await?))
    }

    async fn post_init(&'static self) -> Result<(), PostInitError> {
        Ok(())
    }
}
