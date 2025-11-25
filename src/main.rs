mod anime;
mod cartoon;
mod center;
mod comic;
mod databse;
mod game;
mod manga;
mod movie;
mod series;

use clap::{Parser, ValueEnum};
use std::io;

#[derive(Parser)]
struct Cli {
    #[arg(short, long, value_enum)]
    add: Option<Commands>,

    #[arg(value_enum, short, long)]
    remove: Option<Commands>,

    #[arg(short, long, value_enum)]
    update: Option<Commands>,

    #[arg(short, long, value_enum)]
    view: Option<Commands>,

    #[arg(short, long, value_enum)]
    display: Option<Commands>,
}

#[derive(ValueEnum, Clone)]
enum Commands {
    All,
    Anime,
    Manga,
    Series,
    Cartoon,
    Comic,
    Movie,
    Game,
}

fn main() {
    let path = "database.db";
    let unique_id: i64;
    databse::db_setup(path).expect("!Create");
    let cli = Cli::parse();
    if let Some(op) = cli.add {
        match op {
            Commands::All => println!("!Applicable"),
            Commands::Anime => anime::add().expect("Anime !Added"),
            Commands::Manga => manga::add().expect("Manga !Added"),
            Commands::Series => series::add().expect("Series !Added"),
            Commands::Cartoon => cartoon::add().expect("Cartoon !Added"),
            Commands::Comic => comic::add().expect("Comic !Added"),
            Commands::Movie => movie::add().expect("Movie !Added"),
            Commands::Game => game::add().expect("Game !Added"),
        }
    } else if let Some(op) = cli.view {
        match op {
            Commands::All => println!("!Applicable"),
            Commands::Anime => {
                println!("Unique_ID");
                unique_id = input().parse().expect("!Integer");
                anime::view(unique_id).expect("Anime !Viewed");
            }
            Commands::Manga => {
                println!("Unique_ID");
                unique_id = input().parse().expect("!Integer");
                manga::view(unique_id).expect("Manga !Viewed");
            }
            Commands::Series => {
                println!("Unique_ID");
                unique_id = input().parse().expect("!Integer");
                series::view(unique_id).expect("Series !Viewed");
            }
            Commands::Cartoon => {
                println!("Unique_ID");
                unique_id = input().parse().expect("!Integer");
                cartoon::view(unique_id).expect("Cartoon !Viewed");
            }
            Commands::Comic => {
                println!("Unique_ID");
                unique_id = input().parse().expect("!Integer");
                comic::view(unique_id).expect("Comic !Viewed");
            }
            Commands::Movie => {
                println!("Unique_ID");
                unique_id = input().parse().expect("!Integer");
                movie::view(unique_id).expect("Movie !Viewed");
            }
            Commands::Game => {
                println!("Unique_ID");
                unique_id = input().parse().expect("!Integer");
                game::view(unique_id).expect("Game !Viewed");
            }
        }
    } else if let Some(op) = cli.update {
        match op {
            Commands::All => {
                println!("Unique_ID");
                unique_id = input().parse().expect("!Integer");
                center::update(unique_id).expect("!Updated");
            }
            Commands::Anime => {
                println!("Unique_ID");
                unique_id = input().parse().expect("!Integer");
                anime::update(unique_id).expect("Anime !Updated");
            }
            Commands::Manga => {
                println!("Unique_ID");
                unique_id = input().parse().expect("!Integer");
                manga::update(unique_id).expect("Manga !Updated");
            }
            Commands::Series => {
                println!("Unique_ID");
                unique_id = input().parse().expect("!Integer");
                series::update(unique_id).expect("Series !Updated");
            }
            Commands::Cartoon => {
                println!("Unique_ID");
                unique_id = input().parse().expect("!Integer");
                cartoon::update(unique_id).expect("Cartoon !Updated");
            }
            Commands::Comic => {
                println!("Unique_ID");
                unique_id = input().parse().expect("!Integer");
                comic::update(unique_id).expect("Comic !Updated");
            }
            Commands::Movie => {
                println!("Unique_ID");
                unique_id = input().parse().expect("!Integer");
                movie::update(unique_id).expect("Movie !Updated");
            }
            Commands::Game => {
                println!("Unique_ID");
                unique_id = input().parse().expect("!Integer");
                game::update(unique_id).expect("Game !Updated");
            }
        }
    } else if let Some(op) = cli.remove {
        match op {
            Commands::All => println!("!Applicable"),
            Commands::Anime => {
                println!("Unique_ID");
                unique_id = input().parse().expect("!Integer");
                anime::remove(unique_id).expect("Anime !Removes");
            }
            Commands::Manga => {
                println!("Unique_ID");
                unique_id = input().parse().expect("!Integer");
                manga::remove(unique_id).expect("Manga !Removed");
            }
            Commands::Series => {
                println!("Unique_ID");
                unique_id = input().parse().expect("!Integer");
                series::remove(unique_id).expect("Series !Removed");
            }
            Commands::Cartoon => {
                println!("Unique_ID");
                unique_id = input().parse().expect("!Integer");
                cartoon::remove(unique_id).expect("Cartoon !Removed");
            }
            Commands::Comic => {
                println!("Unique_ID");
                unique_id = input().parse().expect("!Integer");
                comic::remove(unique_id).expect("Comic !Removed");
            }
            Commands::Movie => {
                println!("Unique_ID");
                unique_id = input().parse().expect("!Integer");
                movie::remove(unique_id).expect("Movie !Removed");
            }
            Commands::Game => {
                println!("Unique_ID");
                unique_id = input().parse().expect("!Integer");
                game::remove(unique_id).expect("Game !Removed");
            }
        }
    } else if let Some(op) = cli.display {
        match op {
            Commands::All => center::display().expect("!Displayed"),
            Commands::Anime => anime::display().expect("Anime !Displayed"),
            Commands::Manga => manga::display().expect("Manga !Displayed"),
            Commands::Series => series::display().expect("Series !Displayed"),
            Commands::Cartoon => cartoon::display().expect("Cartoon !Displayed"),
            Commands::Comic => comic::display().expect("Comic !Displayed"),
            Commands::Movie => movie::display().expect("Movie !Displayed"),
            Commands::Game => game::display().expect("Game !Displayed"),
        }
    }
}

pub fn input() -> String {
    let mut value: String = String::new();
    io::stdin().read_line(&mut value).expect("Failed");

    return value.trim().parse().unwrap();
}
