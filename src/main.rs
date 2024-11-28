mod config;

fn main() {
    println!("Hello, world!");
    let (port, host, database_url) = config::get_config();

    println!("Servidor rodando em {}:{}", host, port);
    println!("Conectado ao banco de dados: {}", database_url);
}
