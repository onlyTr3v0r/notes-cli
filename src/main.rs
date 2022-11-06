mod lib;

use lib::*;

use clap::Parser;
use crossterm::{execute, terminal};
use inquire::{MultiSelect, Select, Text};
use sqlx::{migrate, Connection, SqliteConnection};
use std::io::stdout;
use std::path::Path;
use std::str::FromStr;
use strum::VariantNames;
use viuer;
use std::env;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    /// The action to perform
    action: Option<Action>,

    #[arg(default_value_t = String::from("ferris.png"))]
    /// The path to the image to display on greet
    greeting_image: String,

    #[arg(default_value_t = String::from("db.sqlite3"))]
    /// The path to the sqlite database
    db_path: String,
}

#[async_std::main]
async fn main() {
    let mut args = Args::parse();
    env::set_var("DATABASE_URL", &args.db_path);

    if !Path::new(&args.greeting_image).exists() {
        eprintln!(
            "Please provide a valid path to the greeting image or place an image at {}",
            args.greeting_image
        );
        return;
    }

    if !Path::new(&args.db_path).exists() {
        eprintln!(
            "Please provide a valid path to the database or create a database at {}",
            args.db_path
        );
        return;
    }

    let mut conn = SqliteConnection::connect(&args.db_path).await.unwrap();
    migrate!("./migrations").run(&mut conn).await.unwrap();

    execute!(stdout(), terminal::Clear(terminal::ClearType::All),).unwrap();

    viuer::print_from_file(
        args.greeting_image,
        &viuer::Config {
            x: 3,
            y: 3,
            ..Default::default()
        },
    )
    .unwrap();

    print!("\n\n\n");

    if let None = args.action {
        args.action = Some(
            Action::from_str(
                Select::new("Select an option:", Action::VARIANTS.to_vec())
                    .prompt()
                    .unwrap(),
            )
            .unwrap(),
        );
    }

    match args.action.unwrap() {
        Action::Add { name } => {
            let name = Text::new("Enter the name of the todo:").prompt().unwrap();
            new(&mut conn, name).await;
        }
        Action::List => {
            for todo in list(&mut conn).await {
                println!(
                    "[{}] {}",
                    if todo.completed.unwrap() != 0 {
                        "X"
                    } else {
                        " "
                    },
                    todo.content.unwrap()
                );
            }
        }
        Action::Remove { name } => {
            let all = list(&mut conn).await;
            let contents: Vec<&String> = all
                .iter()
                .map(|todo| todo.content.as_ref().unwrap())
                .collect();

            let names = MultiSelect::new("Select the todos to remove:", contents)
                .prompt()
                .unwrap();

            for name in names {
                remove(&mut conn, name.to_string()).await;
            }
        }
        Action::Toggle { name } => {
            let all = list(&mut conn).await;
            let contents: Vec<&String> = all
                .iter()
                .map(|todo| todo.content.as_ref().unwrap())
                .collect();

            let names = MultiSelect::new("Select the todos to remove:", contents)
                .prompt()
                .unwrap();

            for name in names {
                toggle(&mut conn, name.to_string()).await;
            }
        }
    }
}
