mod lib;

use axum::server::Server;
use axum::Router;
use infrastructure::database::establish_connection;
use infrastructure::web::handlers::AppState;
use infrastructure::web::routes::create_routes;
use std::net::SocketAddr;
fn main() {
    let pool = establish_connection();
    let state = AppState { pool };
    let app = create_routes().with_state(state);

    let (port, host, database_url) = config::get_config();

    println!("Servidor rodando em {}:{}", host, port);

    Server::bind(&SocketAddr::from(([host, port])))
        .serve(app.into_make_service())
        .unwrap();

    // println!("Hello, world!");
    // let (port, host, database_url) = config::get_config();

    // println!("Conectado ao banco de dados: {}", database_url);
}
