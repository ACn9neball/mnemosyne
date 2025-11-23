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
struct Movie {
    id: i64,
    audio: String,
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
    println!("Movie Title");
    let original = input();
    println!("Year");
    let year: i32 = input().parse().expect("!Integer");
    println!("Audio [j/e/b/o]");
    let audio_char: String = input().to_lowercase();
    let mut audio: String = "Other".to_string();
    if audio_char == "j" {
        audio = "Japanese".to_string();
    } else if audio_char == "e" {
        audio = "English".to_string();
    } else if audio_char == "b" {
        audio = "Both".to_string();
    }
    let current_date: DateTime<Local> = Local::now();
    let date = current_date.format("%Y/%m/%d/%H/%M/%S").to_string();
    let movie = Movie {
        id: 0,
        audio: audio,
        year: year,
        date: date,
        original: original,
        unique_id: unique_id,
    };
    c.execute(
        "INSERT INTO movie (audio, year, original, unique_id , date) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        (&movie.audio, &movie.year, &movie.original, &movie.unique_id, &movie.date),
    )?;
    println!("Added.");

    Ok(())
}

pub fn view(id: i64) -> Result<()> {
    let c = Connection::open(DB)?;
    let mut movie_table = c.prepare(
        "SELECT movie.*, main.title FROM movie JOIN main ON movie.unique_id = main.unique_id",
    )?;
    let movie_iter = movie_table.query_map([], |row| {
        Ok(Movie {
            id: row.get(0)?,
            audio: row.get(1)?,
            year: row.get(2)?,
            original: row.get(3)?,
            unique_id: row.get(4)?,
            date: row.get(5)?,
        })
    })?;
    for movie in movie_iter {
        let m = movie.unwrap();
        let unique_id: i64 = m.id;
        if id == unique_id {
            println!("Title: {}", m.original);
            println!("Audio: {}", m.audio);
            println!("Year: {}", m.year.to_string());
            println!("Date: {}", m.date);
            println!("Unique_ID: {}", m.id.to_string());
        }
    }

    Ok(())
}

pub fn update(id: i64) -> Result<()> {
    let c = Connection::open(DB)?;
    println!("Year");
    let year: i32 = input().parse().expect("!Integer");
    println!("Audio [j/e/b/o]");
    let audio_char: String = input().to_lowercase();
    let mut audio: String = "Other".to_string();
    if audio_char == "j" {
        audio = "Japanese".to_string();
    } else if audio_char == "e" {
        audio = "English".to_string();
    } else if audio_char == "b" {
        audio = "Both".to_string();
    }
    println!("Movie Title");
    let original = input();
    let current_date: DateTime<Local> = Local::now();
    let date = current_date.format("%Y/%m/%d/%H/%M/%S").to_string();
    let movie = Movie {
        id: 0,
        audio: audio,
        year: year,
        date: date,
        unique_id: 0,
        original: original,
    };
    c.execute(
        "UPDATE movie SET audio = ?2, year = ?3, original = ?4, date = ?5 WHERE id = ?1",
        (id, &movie.audio, &movie.year, &movie.original, &movie.date),
    )?;
    println!("Updated!");

    Ok(())
}

pub fn remove(id: i64) -> Result<()> {
    let c = Connection::open(DB)?;
    c.execute("DELETE FROM movie WHERE id = ?1", (id,))?;
    println!("{} removed!", id);

    Ok(())
}

pub fn display() -> Result<(), Box<dyn Error>> {
    let c = Connection::open(DB)?;
    let mut all = c.prepare(
        "SELECT movie.*, main.title FROM movie JOIN main ON movie.unique_id = main.unique_id",
    )?;
    let movie_iter = all.query_map([], |row| {
        Ok(Movie {
            id: row.get(0)?,
            audio: row.get(1)?,
            year: row.get(2)?,
            original: row.get(3)?,
            unique_id: row.get(5)?,
            date: row.get(6)?,
        })
    })?;
    let mut data = Vec::new();
    for movie in movie_iter {
        let m = movie.unwrap();
        data.push(vec![
            m.id.cell(),
            m.original.cell(),
            m.audio.cell(),
            m.year.cell(),
            m.date.cell(),
        ]);
    }

    let table = data
        .table()
        .title(vec![
            "ID".cell().bold(true),
            "TITLE".cell().bold(true),
            "AUDIO".cell().bold(true),
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
