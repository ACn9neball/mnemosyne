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
struct Series {
    id: i64,
    audio: String,
    year: i32,
    episode: String,
    completed: String,
    date: String,
    unique_id: i64,
    title: String,
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
    println!("Episode [S? E?]");
    let episode = input();
    println!("Completed [y/n]");
    let complete = input().to_lowercase();
    let mut completed: String = "No".to_string();
    if complete == "y" {
        completed = "Yes".to_string();
    }
    let current_date: DateTime<Local> = Local::now();
    let date = current_date.format("%Y/%m/%d/%H/%M/%S").to_string();
    let series = Series {
        id: 0,
        audio: audio,
        year: year,
        date: date,
        unique_id: unique_id,
        episode: episode,
        completed: completed,
        title: "".to_string(),
    };
    c.execute(
        "INSERT INTO series (audio, year, episode, completed, unique_id , date) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        (&series.audio, &series.year, &series.episode, &series.completed, &series.unique_id, &series.date),
    )?;
    println!("Added.");

    Ok(())
}

pub fn view(id: i64) -> Result<()> {
    let c = Connection::open(DB)?;
    let mut series_table = c.prepare(
        "SELECT series.*, main.title FROM series JOIN main ON series.unique_id = main.unique_id",
    )?;
    let series_iter = series_table.query_map([], |row| {
        Ok(Series {
            id: row.get(0)?,
            audio: row.get(1)?,
            year: row.get(2)?,
            episode: row.get(3)?,
            completed: row.get(4)?,
            unique_id: row.get(5)?,
            date: row.get(6)?,
            title: row.get(7)?,
        })
    })?;
    for series in series_iter {
        let s = series.unwrap();
        let unique_id: i64 = s.id;
        if id == unique_id {
            println!("Title: {}", s.title);
            println!("Audio: {}", s.audio);
            println!("Year: {}", s.year.to_string());
            println!("Episode: {}", s.episode);
            println!("Completed: {}", s.completed);
            println!("Date: {}", s.date);
            println!("Unique_ID: {}", s.unique_id.to_string());
        }
    }

    Ok(())
}

pub fn update(id: i64) -> Result<()> {
    let c = Connection::open(DB)?;
    println!("Year");
    let year: i32 = input().parse().unwrap_or(0);
    println!("Audio [j/e/b/o]");
    let audio_char: String = input().to_lowercase();
    let audio: String;
    if audio_char == "j" {
        audio = "Japanese".to_string();
    } else if audio_char == "e" {
        audio = "English".to_string();
    } else if audio_char == "b" {
        audio = "Both".to_string();
    } else {
        audio = "Other".to_string();
    }
    println!("Episode [S? E?]");
    let episode = input();
    println!("Completed [y/n]");
    let complete = input().to_lowercase();
    let mut completed: String = "No".to_string();
    if complete == "y" {
        completed = "Yes".to_string();
    }
    let current_date: DateTime<Local> = Local::now();
    let date = current_date.format("%Y/%m/%d/%H/%M/%S").to_string();
    let series = Series {
        id: 0,
        audio: audio,
        year: year,
        date: date,
        unique_id: 0,
        episode: episode,
        completed: completed,
        title: "".to_string(),
    };
    c.execute(
        "UPDATE series SET
        audio = COALESCE(NULLIF(?2, ''), audio), 
        year = COALESCE(NULLIF(?3, 0), year), 
        episode = COALESCE(NULLIF(?4, ''), episode),
        completed = COALESCE(NULLIF(?5, ''), completed),
        date = COALESCE(NULLIF(?6, ''), date) 
    WHERE id = ?1",
        (
            id,
            &series.audio,
            &series.year,
            &series.episode,
            &series.completed,
            &series.date,
        ),
    )?;
    println!("Updated!");

    Ok(())
}

pub fn remove(id: i64) -> Result<()> {
    let c = Connection::open(DB)?;
    c.execute("DELETE FROM series WHERE id = ?1", (id,))?;
    println!("{} removed!", id);

    Ok(())
}

pub fn display() -> Result<(), Box<dyn Error>> {
    let c = Connection::open(DB)?;
    let mut all = c.prepare(
        "SELECT series.*, main.title FROM series JOIN main ON series.unique_id = main.unique_id",
    )?;
    let series_iter = all.query_map([], |row| {
        Ok(Series {
            id: row.get(0)?,
            audio: row.get(1)?,
            year: row.get(2)?,
            episode: row.get(3)?,
            completed: row.get(4)?,
            unique_id: row.get(5)?,
            date: row.get(6)?,
            title: row.get(7)?,
        })
    })?;
    let mut data = Vec::new();
    for series in series_iter {
        let s = series.unwrap();
        data.push(vec![
            s.id.cell(),
            s.title.cell(),
            s.audio.cell(),
            s.year.cell(),
            s.episode.cell(),
            s.completed.cell(),
            s.date.cell(),
        ]);
    }

    let table = data
        .table()
        .title(vec![
            "ID".cell().bold(true),
            "TITLE".cell().bold(true),
            "AUDIO".cell().bold(true),
            "Year".cell().bold(true),
            "EPISODE".cell().bold(true),
            "COMPLETED".cell().bold(true),
            "DATE".cell().bold(true),
        ])
        .bold(true);

    print_stdout(table)?;

    Ok(())
}

pub fn search() -> Result<(), Box<dyn Error>> {
    let c = Connection::open(DB)?;
    println!("Enter Series");
    let value = input();
    let mut all = c.prepare(
        "SELECT series.*, main.title FROM series JOIN main ON series.unique_id = main.unique_id WHERE main.title LIKE ?1",
    )?;

    let sp = format!("%{}%", value);
    let series_iter = all.query_map([&sp], |row| {
        Ok(Series {
            id: row.get(0)?,
            audio: row.get(1)?,
            year: row.get(2)?,
            episode: row.get(3)?,
            completed: row.get(4)?,
            unique_id: row.get(5)?,
            date: row.get(6)?,
            title: row.get(7)?,
        })
    })?;
    let mut data = Vec::new();
    for series in series_iter {
        let s = series.unwrap();
        data.push(vec![
            s.id.cell(),
            s.title.cell(),
            s.audio.cell(),
            s.year.cell(),
            s.episode.cell(),
            s.completed.cell(),
            s.date.cell(),
        ]);
    }

    let table = data
        .table()
        .title(vec![
            "ID".cell().bold(true),
            "TITLE".cell().bold(true),
            "AUDIO".cell().bold(true),
            "Year".cell().bold(true),
            "EPISODE".cell().bold(true),
            "COMPLETED".cell().bold(true),
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
