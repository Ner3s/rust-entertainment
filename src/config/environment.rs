use dotenv::dotenv;
use std::env;

pub fn get_config() -> (String, String, String) {
  dotenv().ok();

  let port = env::var("PORT").unwrap_or("3000".to_string());
  let host = env::var("HOST").unwrap_or("localhost".to_string());
  let database_url = env::var("DATABASE_URL").unwrap_or("".to_string());

  (port, host, database_url)
}