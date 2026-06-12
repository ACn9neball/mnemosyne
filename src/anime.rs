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
struct Anime {
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
    let completed: String;
    if complete == "y" {
        completed = "Yes".to_string();
    } else if complete == "n" {
        completed = "No".to_string();
    } else {
        completed = "".to_string();
    }
    let current_date: DateTime<Local> = Local::now();
    let date = current_date.format("%Y/%m/%d/%H/%M/%S").to_string();
    let anime = Anime {
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
        "INSERT INTO anime (audio, year, episode, completed, unique_id , date) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        (&anime.audio, &anime.year, &anime.episode, &anime.completed, &anime.unique_id, &anime.date),
    )?;
    println!("Added.");

    Ok(())
}

pub fn view(id: i64) -> Result<()> {
    let c = Connection::open(DB)?;
    let mut anime_table = c.prepare(
        "SELECT anime.*, main.title FROM anime JOIN main ON anime.unique_id = main.unique_id",
    )?;
    let anime_iter = anime_table.query_map([], |row| {
        Ok(Anime {
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
    for anime in anime_iter {
        let a = anime.unwrap();
        let unique_id: i64 = a.id;
        if id == unique_id {
            println!("Title: {}", a.title);
            println!("Audio: {}", a.audio);
            println!("Year: {}", a.year.to_string());
            println!("Episode: {}", a.episode);
            println!("Completed: {}", a.completed);
            println!("Date: {}", a.date);
            println!("Unique_ID: {}", a.unique_id.to_string());
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
    } else if audio_char == "o" {
        audio = "Other".to_string();
    } else {
        audio = "".to_string();
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
    let anime = Anime {
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
        "UPDATE anime SET 
        audio = COALESCE(NULLIF(?2, ''), audio), 
        year = COALESCE(NULLIF(?3, 0), year), 
        episode = COALESCE(NULLIF(?4, ''), episode), 
        completed = COALESCE(NULLIF(?5, ''), completed), 
        date = COALESCE(NULLIF(?6, ''), date) 
    WHERE id = ?1",
        (
            id,
            &anime.audio,
            &anime.year,
            &anime.episode,
            &anime.completed,
            &anime.date,
        ),
    )?;
    println!("Updated!");

    Ok(())
}

pub fn remove(id: i64) -> Result<()> {
    let c = Connection::open(DB)?;
    c.execute("DELETE FROM anime WHERE id = ?1", (id,))?;
    println!("{} removed!", id);

    Ok(())
}

pub fn display() -> Result<(), Box<dyn Error>> {
    let c = Connection::open(DB)?;
    let mut all = c.prepare(
        "SELECT anime.*, main.title FROM anime JOIN main ON anime.unique_id = main.unique_id",
    )?;
    let anime_iter = all.query_map([], |row| {
        Ok(Anime {
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
    for anime in anime_iter {
        let a = anime.unwrap();

        data.push(vec![
            a.id.cell(),
            a.title.cell(),
            a.audio.cell(),
            a.year.cell(),
            a.episode.cell(),
            a.completed.cell(),
            a.date.cell(),
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
    println!("Anime Title");
    let value = input();

    let mut all = c.prepare(
        "SELECT anime.*, main.title FROM anime 
         JOIN main ON anime.unique_id = main.unique_id 
         WHERE main.title LIKE ?1",
    )?;

    let sp = format!("%{}%", value);

    let anime_iter = all.query_map([&sp], |row| {
        Ok(Anime {
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
    for anime in anime_iter {
        let a = anime.unwrap();
        data.push(vec![
            a.id.cell(),
            a.title.cell(),
            a.audio.cell(),
            a.year.cell(),
            a.episode.cell(),
            a.completed.cell(),
            a.date.cell(),
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

pub fn incomplete() -> Result<(), Box<dyn Error>> {
    let c = Connection::open(DB)?;

    let mut all = c.prepare(
        "SELECT anime.*, main.title FROM anime 
         JOIN main ON anime.unique_id = main.unique_id 
         WHERE anime.completed = 'No'",
    )?;

    let anime_iter = all.query_map([], |row| {
        Ok(Anime {
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
    for anime in anime_iter {
        let a = anime.unwrap();
        data.push(vec![
            a.id.cell(),
            a.title.cell(),
            a.audio.cell(),
            a.year.cell(),
            a.episode.cell(),
            a.completed.cell(),
            a.date.cell(),
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
