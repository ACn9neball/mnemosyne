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
struct Comic {
    id: i64,
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
    let comic = Comic {
        id: 0,
        year: year,
        date: date,
        unique_id: unique_id,
        chapter: chapter,
        completed: completed,
        title: "".to_string(),
    };
    c.execute(
        "INSERT INTO manga (year, chapter, completed, unique_id , date) VALUES (?1, ?2, ?3, ?4, ?5)",
        (&comic.year, &comic.chapter, &comic.completed, &comic.unique_id, &comic.date),
    )?;
    println!("Added.");

    Ok(())
}

pub fn view(id: i64) -> Result<()> {
    let c = Connection::open(DB)?;
    let mut comic_table = c.prepare(
        "SELECT comic.*, main.title FROM comic JOIN main ON comic.unique_id = main.unique_id",
    )?;
    let comic_iter = comic_table.query_map([], |row| {
        Ok(Comic {
            id: row.get(0)?,
            year: row.get(2)?,
            chapter: row.get(3)?,
            completed: row.get(4)?,
            unique_id: row.get(5)?,
            date: row.get(6)?,
            title: row.get(7)?,
        })
    })?;
    for comic in comic_iter {
        let c = comic.unwrap();
        let unique_id: i64 = c.id;
        if id == unique_id {
            println!("Title: {}", c.title);
            println!("Year: {}", c.year.to_string());
            println!("Episode: {}", c.chapter);
            println!("Completed: {}", c.completed);
            println!("Date: {}", c.date);
            println!("Unique_ID: {}", c.unique_id.to_string());
        }
    }

    Ok(())
}

pub fn update(id: i64) -> Result<()> {
    let c = Connection::open(DB)?;
    println!("Year");
    let year: i32 = input().parse().unwrap_or(0);
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
    let comic = Comic {
        id: 0,
        year: year,
        date: date,
        unique_id: 0,
        chapter: chapter,
        completed: completed,
        title: "".to_string(),
    };
    c.execute(
        "UPDATE comic SET
        year = COALESCE(NULLIF(?3, 0), year), 
        chapter = COALESCE(NULLIF(?4, ''), chapter),
        completed = COALESCE(NULLIF(?5, ''), completed),
        date = COALESCE(NULLIF(?6, ''), date) 
    WHERE id = ?1",
        (
            id,
            &comic.year,
            &comic.chapter,
            &comic.completed,
            &comic.date,
        ),
    )?;
    println!("Updated!");

    Ok(())
}

pub fn remove(id: i64) -> Result<()> {
    let c = Connection::open(DB)?;
    c.execute("DELETE FROM comic WHERE id = ?1", (id,))?;
    println!("{} removed!", id);

    Ok(())
}

pub fn display() -> Result<(), Box<dyn Error>> {
    let c = Connection::open(DB)?;
    let mut all = c.prepare(
        "SELECT comic.*, main.title FROM comic JOIN main ON comic.unique_id = main.unique_id",
    )?;
    let comic_iter = all.query_map([], |row| {
        Ok(Comic {
            id: row.get(0)?,
            year: row.get(2)?,
            chapter: row.get(3)?,
            completed: row.get(4)?,
            unique_id: row.get(5)?,
            date: row.get(6)?,
            title: row.get(7)?,
        })
    })?;
    let mut data = Vec::new();
    for comic in comic_iter {
        let c = comic.unwrap();
        data.push(vec![
            c.id.cell(),
            c.title.cell(),
            c.year.cell(),
            c.chapter.cell(),
            c.completed.cell(),
            c.date.cell(),
        ]);
    }

    let table = data
        .table()
        .title(vec![
            "ID".cell().bold(true),
            "TITLE".cell().bold(true),
            "Year".cell().bold(true),
            "CHAPTER".cell().bold(true),
            "COMPLETED".cell().bold(true),
            "DATE".cell().bold(true),
        ])
        .bold(true);

    print_stdout(table)?;

    Ok(())
}

pub fn search() -> Result<(), Box<dyn Error>> {
    let c = Connection::open(DB)?;
    println!("Comic Title");
    let value = input();
    let mut all = c.prepare(
        "SELECT comic.*, main.title FROM comic JOIN main ON comic.unique_id = main.unique_id WHERE main.title LIKE ?1",
    )?;

    let sp = format!("%{}%", value);
    let comic_iter = all.query_map([&sp], |row| {
        Ok(Comic {
            id: row.get(0)?,
            year: row.get(2)?,
            chapter: row.get(3)?,
            completed: row.get(4)?,
            unique_id: row.get(5)?,
            date: row.get(6)?,
            title: row.get(7)?,
        })
    })?;
    let mut data = Vec::new();
    for comic in comic_iter {
        let c = comic.unwrap();
        data.push(vec![
            c.id.cell(),
            c.title.cell(),
            c.year.cell(),
            c.chapter.cell(),
            c.completed.cell(),
            c.date.cell(),
        ]);
    }

    let table = data
        .table()
        .title(vec![
            "ID".cell().bold(true),
            "TITLE".cell().bold(true),
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
        "SELECT comic.*, main.title FROM comic JOIN main ON comic.unique_id = main.unique_id WHERE comic.complete = 'No'",
    )?;

    let comic_iter = all.query_map([], |row| {
        Ok(Comic {
            id: row.get(0)?,
            year: row.get(2)?,
            chapter: row.get(3)?,
            completed: row.get(4)?,
            unique_id: row.get(5)?,
            date: row.get(6)?,
            title: row.get(7)?,
        })
    })?;
    let mut data = Vec::new();
    for comic in comic_iter {
        let c = comic.unwrap();
        data.push(vec![
            c.id.cell(),
            c.title.cell(),
            c.year.cell(),
            c.chapter.cell(),
            c.completed.cell(),
            c.date.cell(),
        ]);
    }

    let table = data
        .table()
        .title(vec![
            "ID".cell().bold(true),
            "TITLE".cell().bold(true),
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
