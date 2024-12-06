use diesel::r2d2::{self, ConnectionManager};
use diesel::pg::PgConnection;
use crate::config;


pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> DbPool {
    let (_, _, database_url) = config::get_config();
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    
    r2d2::Pool::builder()
        .build(manager)
        .expect("Erro ao criar pool de conexões")
}