use std::net::TcpListener;
use sqlx::PgPool;
use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main()->std::io::Result<()>{

    let configuration = get_configuration().expect("Failed to read connection.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
    .await.expect("Failed to connect to Postgres.");
    
    let address = format!("127.0.0.1:{}",configuration.application_port);
    
    let listener = TcpListener::bind(address)?;
    println!("{}",listener.local_addr().unwrap());
    run(listener,connection_pool)?.await?;
    Ok(())
}