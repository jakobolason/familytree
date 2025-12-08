use std::path::Path;

use async_trait::async_trait;
use loco_rs::{
    app::{AppContext, Hooks, Initializer},
    bgworker::Queue,
    boot::{create_app, BootResult, StartMode},
    config::Config,
    controller::AppRoutes,
    environment::Environment,
    task::Tasks,
    Result,
};
use migration::Migrator;

use crate::{controllers, models::family_tree::FamilyTree, tasks};

pub struct App;
#[async_trait]
impl Hooks for App {
    fn app_name() -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    fn app_version() -> String {
        format!(
            "{} ({})",
            env!("CARGO_PKG_VERSION"),
            option_env!("BUILD_SHA")
                .or(option_env!("GITHUB_SHA"))
                .unwrap_or("dev")
        )
    }

    async fn boot(
        mode: StartMode,
        environment: &Environment,
        config: Config,
    ) -> Result<BootResult> {
        create_app::<Self, Migrator>(mode, environment, config).await
    }

    // async fn initializers(
    //     router: axum::Router,
    //     _ctx: &AppContext,
    // ) -> Result<axum::Router> {
    //     println!("Loading family tree cache from database...");
    //     if let Err(e) = FamilyTree::load_cache_from_db(&_ctx.db).await {
    //         eprintln!(
    //             "Warning: Failed to load family tree cache from database: {}",
    //             e
    //         );
    //     } else {
    //         println!("Family tree cache loaded from database.");
    //     }
    //     Ok(router)
    // }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        // Register all routes
        AppRoutes::with_default_routes()
            .prefix("/api")
            .add_route(controllers::auth::routes())
            .add_route(controllers::user::routes())
            .add_route(controllers::admin::routes())
    }

    async fn after_routes(router: axum::Router, _ctx: &AppContext) -> Result<axum::Router> {
        println!("Retriving latest family graph...");
        if let Err(e) = FamilyTree::load_cache_from_db(&_ctx.db).await {
            eprintln!(
                "Warning: Failed to load family tree cache from database: {}",
                e
            );
        } else {
            println!("Family tree cache loaded from database.");
        }
        Ok(router)
    }

    async fn connect_workers(_ctx: &AppContext, _queue: &Queue) -> Result<()> {
        Ok(())
    }

    fn register_tasks(tasks: &mut Tasks) {
        // Register all tasks
        tasks.register(tasks::seed::SeedData);
        tasks.register(tasks::seed_graph::SeedTree);
        tasks.register(tasks::seed_user::SeedUser);
    }

    async fn truncate(_ctx: &AppContext) -> Result<()> {
        Ok(())
    }

    async fn seed(_ctx: &AppContext, _base: &Path) -> Result<()> {
        Ok(())
    }
}
