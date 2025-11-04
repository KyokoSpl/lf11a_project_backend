// Handeling DB Connection

use mysql::*;

pub type DbPool = Pool;

pub fn create_pool(database_url: &str) -> Result<DbPool, mysql::Error> {
    let opts = Opts::from_url(database_url)?;
    Pool::new(opts)
}
