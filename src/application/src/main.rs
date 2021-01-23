use std::{net::SocketAddr, path::PathBuf};

use application::grpc::{generated::user_proto::user_server::UserServer, handlers::user_handler::UserHandler};
use cross_cutting::EnvironmentVariables;
use db::{connection::Settings, mongo::user_datasource_mongo::{UserDataSourceMongo}};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::new(PathBuf::default()).expect("Failed to load configuration");

    // // set EnvironmentVariables
    EnvironmentVariables::set_mongo_connection_string(&settings.mongodb.connection_string());
    EnvironmentVariables::set_mongo_db_name(&settings.mongodb.db_name);

    // Repositories
    let user_repository = db::repositories::UserRepository::new(UserDataSourceMongo{});

    let addr: SocketAddr = "127.0.0.1:5002".parse().unwrap();

    println!("Starting ther server at : {}", addr);

    Server::builder()
        .add_service(UserServer::new(UserHandler::new(
            user_repository,
        )))
        .serve(addr)
        .await?;

    Ok(())
}