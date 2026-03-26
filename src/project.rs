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
struct Project {
    id: i64,
    language: String,
    year: i32,
    completed: String,
    description: String,
    commited: String,
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
    println!("Language");
    let language: String = input().to_lowercase();
    println!("Completed [y/n]");
    let complete = input().to_lowercase();
    let mut completed: String = "No".to_string();
    if complete == "y" {
        completed = "Yes".to_string();
    }
    println!("Commited [y/n]");
    let commit = input().to_lowercase();
    let mut commited: String = "No".to_string();
    if commit == "y" {
        commited = "Yes".to_string();
    }
    println!("Description");
    let description = input();
    let current_date: DateTime<Local> = Local::now();
    let date = current_date.format("%Y/%m/%d/%H/%M/%S").to_string();
    let project = Project {
        id: 0,
        language: language,
        year: year,
        description: description,
        commited: commited,
        date: date,
        unique_id: unique_id,
        completed: completed,
        title: "".to_string(),
    };
    c.execute(
        "INSERT INTO project (language, year, completed, description, commited, unique_id , date) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        (&project.language, &project.year, &project.completed, &project.description, &project.commited, &project.unique_id, &project.date),
    )?;
    println!("Added.");

    Ok(())
}

pub fn view(id: i64) -> Result<()> {
    let c = Connection::open(DB)?;
    let mut project_table = c.prepare(
        "SELECT project.*, main.title FROM project JOIN main ON project.unique_id = main.unique_id",
    )?;
    let project_iter = project_table.query_map([], |row| {
        Ok(Project {
            id: row.get(0)?,
            language: row.get(1)?,
            year: row.get(2)?,
            completed: row.get(3)?,
            description: row.get(4)?,
            commited: row.get(5)?,
            unique_id: row.get(6)?,
            date: row.get(7)?,
            title: row.get(8)?,
        })
    })?;
    for project in project_iter {
        let a = project.unwrap();
        let unique_id: i64 = a.id;
        if id == unique_id {
            println!("Title: {}", a.title);
            println!("language: {}", a.language);
            println!("Year: {}", a.year.to_string());
            println!("Completed: {}", a.completed);
            println!("Commited: {}", a.commited);
            println!("Description: {}", a.description);
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
    println!("language");
    let language: String = input().to_lowercase();
    println!("Completed [y/n]");
    let complete = input().to_lowercase();
    let mut completed: String = "No".to_string();
    if complete == "y" {
        completed = "Yes".to_string();
    }
    println!("Commited [y/n]");
    let commit = input().to_lowercase();
    let mut commited: String = "No".to_string();
    if commit == "y" {
        commited = "Yes".to_string();
    }
    println!("Description");
    let description = input();
    let current_date: DateTime<Local> = Local::now();
    let date = current_date.format("%Y/%m/%d/%H/%M/%S").to_string();
    let project = Project {
        id: 0,
        language: language,
        year: year,
        date: date,
        unique_id: 0,
        completed: completed,
        description: description,
        commited: commited,
        title: "".to_string(),
    };
    c.execute(
        "UPDATE project SET 
        language = COALESCE(NULLIF(?2, ''), language), 
        year = COALESCE(NULLIF(?3, 0), year), 
        completed = COALESCE(NULLIF(?4, ''), completed),
        description = COALESCE(NULLIF(?5, ''), description),
        commited = COALESCE(NULLIF(?6, ''), commited),
        date = COALESCE(NULLIF(?7, ''), date) 
    WHERE id = ?1",
        (
            id,
            &project.language,
            &project.year,
            &project.completed,
            &project.description,
            &project.commited,
            &project.date,
        ),
    )?;
    println!("Updated!");

    Ok(())
}

pub fn remove(id: i64) -> Result<()> {
    let c = Connection::open(DB)?;
    c.execute("DELETE FROM project WHERE id = ?1", (id,))?;
    println!("{} removed!", id);

    Ok(())
}

pub fn display() -> Result<(), Box<dyn Error>> {
    let c = Connection::open(DB)?;
    let mut all = c.prepare(
        "SELECT project.*, main.title FROM project JOIN main ON project.unique_id = main.unique_id",
    )?;
    let project_iter = all.query_map([], |row| {
        Ok(Project {
            id: row.get(0)?,
            language: row.get(1)?,
            year: row.get(2)?,
            completed: row.get(3)?,
            description: row.get(4)?,
            commited: row.get(5)?,
            unique_id: row.get(6)?,
            date: row.get(7)?,
            title: row.get(8)?,
        })
    })?;
    let mut data = Vec::new();
    for project in project_iter {
        let a = project.unwrap();

        data.push(vec![
            a.id.cell(),
            a.title.cell(),
            a.language.cell(),
            a.year.cell(),
            a.completed.cell(),
            a.commited.cell(),
            a.description.cell(),
            a.date.cell(),
        ]);
    }

    let table = data
        .table()
        .title(vec![
            "ID".cell().bold(true),
            "TITLE".cell().bold(true),
            "LANGUAGE".cell().bold(true),
            "Year".cell().bold(true),
            "COMPLETED".cell().bold(true),
            "COMMITED".cell().bold(true),
            "DESCRIPTION".cell().bold(true),
            "DATE".cell().bold(true),
        ])
        .bold(true);

    print_stdout(table)?;

    Ok(())
}

pub fn search() -> Result<(), Box<dyn Error>> {
    let c = Connection::open(DB)?;
    println!("project Title");
    let value = input();

    let mut all = c.prepare(
        "SELECT project.*, main.title FROM project 
         JOIN main ON project.unique_id = main.unique_id 
         WHERE main.title LIKE ?1",
    )?;

    let sp = format!("%{}%", value);

    let project_iter = all.query_map([&sp], |row| {
        Ok(Project {
            id: row.get(0)?,
            language: row.get(1)?,
            year: row.get(2)?,
            completed: row.get(3)?,
            description: row.get(4)?,
            commited: row.get(5)?,
            unique_id: row.get(6)?,
            date: row.get(7)?,
            title: row.get(8)?,
        })
    })?;
    let mut data = Vec::new();
    for project in project_iter {
        let a = project.unwrap();
        data.push(vec![
            a.id.cell(),
            a.title.cell(),
            a.language.cell(),
            a.year.cell(),
            a.completed.cell(),
            a.commited.cell(),
            a.description.cell(),
            a.date.cell(),
        ]);
    }

    let table = data
        .table()
        .title(vec![
            "ID".cell().bold(true),
            "TITLE".cell().bold(true),
            "LANGUAGE".cell().bold(true),
            "Year".cell().bold(true),
            "COMPLETED".cell().bold(true),
            "COMMITED".cell().bold(true),
            "DESCRIPTION".cell().bold(true),
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
