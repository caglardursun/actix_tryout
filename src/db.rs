use crate::errors::{AppError, AppErrorType::*};
use crate::models::{MyListItem, MyList};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn get_todos(client: &Client) -> Result<Vec<MyList>, AppError> {
    let statement = client
        .prepare("select * from my_list order by id desc")
        .await?;

    let todos = client
        .query(&statement, &[])
        .await?
        .iter()
        .map(|row| MyList::from_row_ref(row).unwrap())
        .collect::<Vec<MyList>>();

    Ok(todos)
}