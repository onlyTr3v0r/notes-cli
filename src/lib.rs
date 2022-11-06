use strum::{EnumString, EnumVariantNames}; // VariantNames is needed to get [enum]::VARIANTS
use sqlx::{query, query_as, SqliteConnection};
use clap::Subcommand;

// The action type is split accross files, unclean but oh well :P
#[derive(Subcommand, EnumVariantNames, EnumString, Debug)]
#[strum(serialize_all = "title_case")]
pub enum Action {
    Add { name: String },
    Remove { name: String },
    Toggle { name: String },
    List,
}

#[derive(Debug)]
pub struct Todo {
    pub content: Option<String>,
    pub completed: Option<i64>,
}

pub async fn new(conn: &mut SqliteConnection, name: String) {
    query!(
        "INSERT INTO Todos (content, completed) VALUES (?, ?)",
        name,
        0
    )
    .execute(conn)
    .await
    .unwrap();
}

pub async fn list(conn: &mut SqliteConnection) -> Vec<Todo> {
    query_as!(Todo, "SELECT * from Todos",)
        .fetch_all(conn)
        .await
        .unwrap()
}

pub async fn remove(conn: &mut SqliteConnection, name: String) {
    query!("DELETE FROM Todos WHERE content = ?", name)
        .execute(conn)
        .await
        .unwrap();
}

pub async fn toggle(conn: &mut SqliteConnection, name: String) {
    query!(
        "UPDATE Todos SET completed = NOT completed WHERE content = ?",
        name
    )
    .execute(conn)
    .await
    .unwrap();
}
