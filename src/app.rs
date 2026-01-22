use async_trait::async_trait;
use loco_rs::{
    app::{AppContext, Hooks, Initializer},
    boot::{create_app, BootResult, StartMode},
    controller::AppRoutes,
    db::{self, truncate_table},
    environment::Environment,
    task::Tasks,
    Result,
};
use migration::Migrator;
use sea_orm::DatabaseConnection;

use crate::controllers;

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

    async fn boot(mode: StartMode, environment: &Environment) -> Result<BootResult> {
        create_app::<Self, Migrator>(mode, environment).await
    }

    async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
        Ok(vec![])
    }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        AppRoutes::with_default_routes()
            .add_route(controllers::home::routes())
            .add_route(controllers::learn::routes())
            .add_route(controllers::play::routes())
            .add_route(controllers::profile::routes())
            .add_route(controllers::api::routes())
    }

    async fn connect_workers(_ctx: &AppContext, _queue: &loco_rs::bgworker::Queue) -> Result<()> {
        Ok(())
    }

    fn register_tasks(tasks: &mut Tasks) {
        // Register seed task
        tasks.register(crate::controllers::seed_task());
    }

    async fn truncate(db: &DatabaseConnection) -> Result<()> {
        truncate_table(db, "users").await?;
        truncate_table(db, "topics").await?;
        truncate_table(db, "progress").await?;
        truncate_table(db, "game_sessions").await?;
        truncate_table(db, "achievements").await?;
        truncate_table(db, "user_achievements").await?;
        Ok(())
    }

    async fn seed(db: &DatabaseConnection, base: &std::path::Path) -> Result<()> {
        db::seed::<crate::models::users::ActiveModel>(db, &base.join("users.yaml")).await?;
        db::seed::<crate::models::topics::ActiveModel>(db, &base.join("topics.yaml")).await?;
        Ok(())
    }
}
