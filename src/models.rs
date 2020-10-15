
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use slog;
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool,
    pub log: slog::Logger,
}


#[derive(Serialize)]
pub struct Status {
    pub status:String 
}
//PostgresMapper is important otherwise pg_mapper macros attribute doesn't work
#[derive(Debug, Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "my_list")]
pub struct MyList{
    pub id:i32,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table="my_list_details")]
pub struct MyListItem{
    pub id:i32,
    pub title: String,
    pub checked:bool,
    pub list_id:i32
}
