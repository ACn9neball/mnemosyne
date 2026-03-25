const DB: &str = "database.db";

use chrono::prelude::*;
use cli_table::*;
use rusqlite::{Connection, Result};
use std::{error::Error, io};

#[derive(Debug)]
struct Main {
    title: String,
}

#[derive(Debug)]
struct Game {
    id: i64,
    genre: String,
    year: i32,
    original: String,
    date: String,
    unique_id: i64,
}

pub fn add() -> Result<()> {
    let c = Connection::open(DB)?;
    println!("New [Y/n]");
    let new: String = input();
    let unique_id: i64;
    if new.to_lowercase() == "y" {
        println!("Title");
        let title: String = input();
        let main = Main {
            title: title.clone(),
        };
        c.execute("INSERT INTO main (title) VALUES (?1)", (&main.title,))?;
        unique_id = c.last_insert_rowid();
    } else {
        println!("Unique_ID");
        unique_id = input().parse().expect("!Integer");
    }
    println!("Game Title");
    let original = input();
    println!("Year");
    let year: i32 = input().parse().expect("!Integer");
    println!("Main Genre");
    let genre: String = input();
    let current_date: DateTime<Local> = Local::now();
    let date = current_date.format("%Y/%m/%d/%H/%M/%S").to_string();
    let game = Game {
        id: 0,
        genre: genre,
        year: year,
        date: date,
        original: original,
        unique_id: unique_id,
    };
    c.execute(
        "INSERT INTO game (year, original, unique_id, genre, date) VALUES (?1, ?2, ?3, ?4, ?5)",
        (
            &game.year,
            &game.original,
            &game.unique_id,
            &game.genre,
            &game.date,
        ),
    )?;
    println!("Added.");

    Ok(())
}

pub fn view(id: i64) -> Result<()> {
    let c = Connection::open(DB)?;
    let mut game_table = c.prepare(
        "SELECT game.*, main.title FROM game JOIN main ON game.unique_id = main.unique_id",
    )?;
    let game_iter = game_table.query_map([], |row| {
        Ok(Game {
            id: row.get(0)?,
            year: row.get(1)?,
            original: row.get(2)?,
            unique_id: row.get(3)?,
            genre: row.get(4)?,
            date: row.get(5)?,
        })
    })?;
    for game in game_iter {
        let g = game.unwrap();
        let unique_id: i64 = g.id;
        if id == unique_id {
            println!("Title: {}", g.original);
            println!("Year: {}", g.year.to_string());
            println!("Genre: {}", g.genre);
            println!("Date: {}", g.date);
            println!("Unique_ID: {}", g.unique_id.to_string());
        }
    }

    Ok(())
}

pub fn update(id: i64) -> Result<()> {
    let c = Connection::open(DB)?;
    println!("Year");
    let year: i32 = input().parse().unwrap_or(0);
    println!("Genre");
    let genre: String = input();
    println!("Game Title");
    let original = input();
    let current_date: DateTime<Local> = Local::now();
    let date = current_date.format("%Y/%m/%d/%H/%M/%S").to_string();
    let game = Game {
        id: 0,
        genre: genre,
        year: year,
        date: date,
        unique_id: 0,
        original: original,
    };
    c.execute(
        "UPDATE game SET 
        genre = COALESCE(NULLIF(?2, ''), genre), 
        year = COALESCE(NULLIF(?3, 0), year), 
        original = COALESCE(NULLIF(?4, ''), original), 
        date = COALESCE(NULLIF(?5, ''), date) 
    WHERE id = ?1",
        (id, &game.genre, &game.year, &game.original, &game.date),
    )?;
    println!("Updated!");

    Ok(())
}

pub fn remove(id: i64) -> Result<()> {
    let c = Connection::open(DB)?;
    c.execute("DELETE FROM game WHERE id = ?1", (id,))?;
    println!("{} removed!", id);

    Ok(())
}

pub fn display() -> Result<(), Box<dyn Error>> {
    let c = Connection::open(DB)?;
    let mut all = c.prepare(
        "SELECT game.*, main.title FROM game JOIN main ON game.unique_id = main.unique_id",
    )?;
    let game_iter = all.query_map([], |row| {
        Ok(Game {
            id: row.get(0)?,
            year: row.get(1)?,
            original: row.get(2)?,
            unique_id: row.get(3)?,
            genre: row.get(4)?,
            date: row.get(5)?,
        })
    })?;
    let mut data = Vec::new();
    for game in game_iter {
        let g = game.unwrap();
        data.push(vec![
            g.id.cell(),
            g.original.cell(),
            g.genre.cell(),
            g.year.cell(),
            g.date.cell(),
        ]);
    }

    let table = data
        .table()
        .title(vec![
            "ID".cell().bold(true),
            "TITLE".cell().bold(true),
            "GENRE".cell().bold(true),
            "Year".cell().bold(true),
            "DATE".cell().bold(true),
        ])
        .bold(true);

    print_stdout(table)?;

    Ok(())
}

pub fn search() -> Result<(), Box<dyn Error>> {
    let c = Connection::open(DB)?;
    println!("Game Title");
    let value = input();
    let mut all = c.prepare(
        "SELECT game.*, main.title FROM game JOIN main ON game.unique_id = main.unique_id WHERE main.title LIKE ?1",
    )?;

    let sp = format!("%{}%", value);
    let game_iter = all.query_map([&sp], |row| {
        Ok(Game {
            id: row.get(0)?,
            year: row.get(1)?,
            original: row.get(2)?,
            unique_id: row.get(3)?,
            genre: row.get(4)?,
            date: row.get(5)?,
        })
    })?;
    let mut data = Vec::new();
    for game in game_iter {
        let g = game.unwrap();
        data.push(vec![
            g.id.cell(),
            g.original.cell(),
            g.genre.cell(),
            g.year.cell(),
            g.date.cell(),
        ]);
    }

    let table = data
        .table()
        .title(vec![
            "ID".cell().bold(true),
            "TITLE".cell().bold(true),
            "GENRE".cell().bold(true),
            "Year".cell().bold(true),
            "DATE".cell().bold(true),
        ])
        .bold(true);

    print_stdout(table)?;

    Ok(())
}

fn input() -> String {
    let mut value: String = String::new();
    io::stdin().read_line(&mut value).expect("Failed");

    return value.trim().parse().unwrap();
}
