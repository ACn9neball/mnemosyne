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
struct Manga {
    id: i64,
    tp: String,
    year: i32,
    chapter: String,
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
    println!("Type [mg/mh/oh]");
    let type_char: String = input().to_lowercase();
    let tp: String;
    if type_char == "mg" {
        tp = "Manga".to_string();
    } else if type_char == "mh" {
        tp = "Manhwa".to_string();
    } else if type_char == "oh" {
        tp = "Other".to_string();
    } else {
        tp = "".to_string();
    }
    println!("Chapter [V? C?]");
    let chapter = input();
    println!("Completed [y/n]");
    let complete = input().to_lowercase();
    let mut completed: String = "No".to_string();
    if complete == "y" {
        completed = "Yes".to_string();
    }
    let current_date: DateTime<Local> = Local::now();
    let date = current_date.format("%Y/%m/%d/%H/%M/%S").to_string();
    let manga = Manga {
        id: 0,
        tp: tp,
        year: year,
        date: date,
        unique_id: unique_id,
        chapter: chapter,
        completed: completed,
        title: "".to_string(),
    };
    c.execute(
        "INSERT INTO manga (type, year, chapter, completed, unique_id , date) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        (&manga.tp, &manga.year, &manga.chapter, &manga.completed, &manga.unique_id, &manga.date),
    )?;
    println!("Added.");

    Ok(())
}

pub fn view(id: i64) -> Result<()> {
    let c = Connection::open(DB)?;
    let mut manga_table = c.prepare(
        "SELECT manga.*, main.title FROM manga JOIN main ON manga.unique_id = main.unique_id",
    )?;
    let manga_iter = manga_table.query_map([], |row| {
        Ok(Manga {
            id: row.get(0)?,
            tp: row.get(1)?,
            year: row.get(2)?,
            chapter: row.get(3)?,
            completed: row.get(4)?,
            unique_id: row.get(5)?,
            date: row.get(6)?,
            title: row.get(7)?,
        })
    })?;
    for manga in manga_iter {
        let m = manga.unwrap();
        let unique_id: i64 = m.id;
        if id == unique_id {
            println!("Title: {}", m.title);
            println!("Audio: {}", m.tp);
            println!("Year: {}", m.year.to_string());
            println!("Episode: {}", m.chapter);
            println!("Completed: {}", m.completed);
            println!("Date: {}", m.date);
            println!("Unique_ID: {}", m.unique_id.to_string());
        }
    }

    Ok(())
}

pub fn update(id: i64) -> Result<()> {
    let c = Connection::open(DB)?;
    println!("Year");
    let year: i32 = input().parse().unwrap_or(0);
    println!("Type [mg/mh/oh]");
    let type_char: String = input().to_lowercase();
    let tp: String;
    if type_char == "mg" {
        tp = "Manga".to_string();
    } else if type_char == "mh" {
        tp = "Manhwa".to_string();
    } else if type_char == "oh" {
        tp = "Other".to_string();
    } else {
        tp = "".to_string();
    }
    println!("Chapter [V? C?]");
    let chapter = input();
    println!("Completed [y/n]");
    let complete = input().to_lowercase();
    let mut completed: String = "No".to_string();
    if complete == "y" {
        completed = "Yes".to_string();
    }
    let current_date: DateTime<Local> = Local::now();
    let date = current_date.format("%Y/%m/%d/%H/%M/%S").to_string();
    let manga = Manga {
        id: 0,
        tp: tp,
        year: year,
        date: date,
        unique_id: 0,
        chapter: chapter,
        completed: completed,
        title: "".to_string(),
    };
    c.execute(
        "UPDATE manga SET 
        type = COALESCE(NULLIF(?2, ''), type), 
        year = COALESCE(NULLIF(?3, 0), year), 
        chapter = COALESCE(NULLIF(?4, ''), chapter), 
        completed = COALESCE(NULLIF(?5, ''), completed), 
        date = COALESCE(NULLIF(?6, ''), date) 
    WHERE id = ?1",
        (
            id,
            &manga.tp,
            &manga.year,
            &manga.chapter,
            &manga.completed,
            &manga.date,
        ),
    )?;
    println!("Updated!");

    Ok(())
}

pub fn remove(id: i64) -> Result<()> {
    let c = Connection::open(DB)?;
    c.execute("DELETE FROM manga WHERE id = ?1", (id,))?;
    println!("{} removed!", id);

    Ok(())
}

pub fn display() -> Result<(), Box<dyn Error>> {
    let c = Connection::open(DB)?;
    let mut all = c.prepare(
        "SELECT manga.*, main.title FROM manga JOIN main ON manga.unique_id = main.unique_id",
    )?;
    let manga_iter = all.query_map([], |row| {
        Ok(Manga {
            id: row.get(0)?,
            tp: row.get(1)?,
            year: row.get(2)?,
            chapter: row.get(3)?,
            completed: row.get(4)?,
            unique_id: row.get(5)?,
            date: row.get(6)?,
            title: row.get(7)?,
        })
    })?;
    let mut data = Vec::new();
    for manga in manga_iter {
        let m = manga.unwrap();
        data.push(vec![
            m.id.cell(),
            m.title.cell(),
            m.tp.cell(),
            m.year.cell(),
            m.chapter.cell(),
            m.completed.cell(),
        ]);
    }

    let table = data
        .table()
        .title(vec![
            "ID".cell().bold(true),
            "TITLE".cell().bold(true),
            "TYPE".cell().bold(true),
            "Year".cell().bold(true),
            "CHAPTER".cell().bold(true),
            "COMPLETED".cell().bold(true),
        ])
        .bold(true);

    print_stdout(table)?;

    Ok(())
}

pub fn search() -> Result<(), Box<dyn Error>> {
    let c = Connection::open(DB)?;
    println!("Manga Title");
    let value = input();
    let mut all = c.prepare(
        "SELECT manga.*, main.title FROM manga JOIN main ON manga.unique_id = main.unique_id WHERE main.title LIKE ?1",
    )?;

    let sp = format!("%{}%", value);

    let manga_iter = all.query_map([&sp], |row| {
        Ok(Manga {
            id: row.get(0)?,
            tp: row.get(1)?,
            year: row.get(2)?,
            chapter: row.get(3)?,
            completed: row.get(4)?,
            unique_id: row.get(5)?,
            date: row.get(6)?,
            title: row.get(7)?,
        })
    })?;
    let mut data = Vec::new();
    for manga in manga_iter {
        let m = manga.unwrap();
        data.push(vec![
            m.id.cell(),
            m.title.cell(),
            m.tp.cell(),
            m.year.cell(),
            m.chapter.cell(),
            m.completed.cell(),
            m.date.cell(),
        ]);
    }

    let table = data
        .table()
        .title(vec![
            "ID".cell().bold(true),
            "TITLE".cell().bold(true),
            "TYPE".cell().bold(true),
            "Year".cell().bold(true),
            "CHAPTER".cell().bold(true),
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
        "SELECT manga.*, main.title FROM manga JOIN main ON manga.unique_id = main.unique_id WHERE manga.completed = 'No'",
    )?;

    let manga_iter = all.query_map([], |row| {
        Ok(Manga {
            id: row.get(0)?,
            tp: row.get(1)?,
            year: row.get(2)?,
            chapter: row.get(3)?,
            completed: row.get(4)?,
            unique_id: row.get(5)?,
            date: row.get(6)?,
            title: row.get(7)?,
        })
    })?;
    let mut data = Vec::new();
    for manga in manga_iter {
        let m = manga.unwrap();
        data.push(vec![
            m.id.cell(),
            m.title.cell(),
            m.tp.cell(),
            m.year.cell(),
            m.chapter.cell(),
            m.completed.cell(),
            m.date.cell(),
        ]);
    }

    let table = data
        .table()
        .title(vec![
            "ID".cell().bold(true),
            "TITLE".cell().bold(true),
            "TYPE".cell().bold(true),
            "Year".cell().bold(true),
            "CHAPTER".cell().bold(true),
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
