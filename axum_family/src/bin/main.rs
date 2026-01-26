use familyserver_backend::app::App;
use loco_rs::cli;
use migration::Migrator;

#[tokio::main]
async fn main() -> loco_rs::Result<()> {
    dotenv::dotenv().ok();
    cli::main::<App, Migrator>().await
}
