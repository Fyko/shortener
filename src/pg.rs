use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use std::env;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool() -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url());
    return DbPool::new(manager).expect("Database Pool.");
}

fn database_url() -> String {
    return env::var("DATABASE_URL").expect("DB URL must be set.");
}

pub struct DbConnection(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);
