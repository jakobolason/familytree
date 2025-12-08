use loco_rs::{environment::Environment, prelude::*};
use migration::{IntoColumnRef, IntoIden};
use sea_orm::{
    sea_query::{Alias, Expr, Func},
    DatabaseBackend, DbConn, DeriveColumn, EnumIter, FromQueryResult,
};
use sea_orm_pro::{ConfigParser, JsonCfg};
use seaography::lazy_static;
use serde::{Deserialize, Serialize};

const CONFIG_ROOT: &str = "pro_admin";

lazy_static::lazy_static! {
    static ref CONFIG: JsonCfg = ConfigParser::new().load_config(CONFIG_ROOT).unwrap();
}

pub async fn config(State(ctx): State<AppContext>) -> Result<Response> {
    if ctx.environment == Environment::Production {
        // Release: load config from the disk once and then return the cached config afterwards
        format::json(&*CONFIG)
    } else {
        // Debug: load config from disk on every request
        let config = ConfigParser::new()
            .load_config(CONFIG_ROOT)
            .map_err(Into::<Box<dyn std::error::Error + Send + Sync>>::into)?;
        format::json(config)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DashboardBody {
    pub graph: String,
    pub from: Option<DateTime>,
    pub to: Option<DateTime>,
}

#[derive(Debug, Deserialize, Serialize, FromQueryResult, PartialEq)]
pub struct Datum {
    pub key: String,
    pub val: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum DatumColumn {
    Key,
    Val,
}

fn cast_as_year_month(db: &DbConn, col: impl IntoColumnRef) -> Expr {
    let func = match db.get_database_backend() {
        DatabaseBackend::MySql => Func::cust(Alias::new("DATE_FORMAT"))
            .arg(Expr::col(col.into_column_ref()))
            .arg("%Y-%m"),
        DatabaseBackend::Postgres => Func::cust(Alias::new("TO_CHAR"))
            .arg(Expr::col(col.into_column_ref()))
            .arg("YYYY-mm"),
        DatabaseBackend::Sqlite => Func::cust(Alias::new("STRFTIME"))
            .arg("%Y-%m")
            .arg(Expr::col(col.into_column_ref())),
        _ => unimplemented!(),
    };
    Expr::expr(func)
}

fn cast_as_day(db: &DbConn, col: impl IntoColumnRef) -> Expr {
    let func = match db.get_database_backend() {
        DatabaseBackend::MySql => Func::cust(Alias::new("DATE_FORMAT"))
            .arg(Expr::col(col.into_column_ref()))
            .arg("%Y-%m-%d"),
        DatabaseBackend::Postgres => Func::cust(Alias::new("TO_CHAR"))
            .arg(Expr::col(col.into_column_ref()))
            .arg("YYYY-mm-dd"),
        DatabaseBackend::Sqlite => Func::cust(Alias::new("STRFTIME"))
            .arg("%Y-%m-%d")
            .arg(Expr::col(col.into_column_ref())),
        _ => unimplemented!(),
    };
    Expr::expr(func)
}

fn int_keyword(db: &DbConn) -> impl IntoIden {
    match db.get_database_backend() {
        DatabaseBackend::MySql => Alias::new("SIGNED INTEGER"),
        DatabaseBackend::Postgres => Alias::new("INT4"),
        DatabaseBackend::Sqlite => Alias::new("INT"),
        _ => unimplemented!(),
    }
}

pub fn routes() -> Routes {
    Routes::new()
        // Admin route prefix
        .prefix("admin")
        // Fetch web config
        .add("/config", get(config))
    // Fetch dashboard graph data
}
