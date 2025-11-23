use rusqlite::{Connection, Result};
use std::fs::metadata;

pub fn db_setup(data: &str) -> Result<()> {
    if !metadata(data).is_ok() {
        let c = Connection::open(data)?;

        c.execute(
            "CREATE TABLE main (unique_id INTEGER PRIMARY KEY, title TEXT NOT NULL)",
            (),
        )?;

        c.execute(
            "CREATE TABLE anime (
            id INTEGER PRIMARY KEY,
            audio TEXT NOT NULL,
            year INTEGER,
            episode TEXT,
            completed TEXT NOT NULL,
            unique_id INTEGER,
            date TEXT NOT NULL
        )",
            (),
        )?;

        c.execute(
            "CREATE TABLE manga (
            id INTEGER PRIMARY KEY,
            type TEXT NOT NULL,
            year INTEGER,
            chapter TEXT,
            completed TEXT NOT NULL,
            unique_id INTEGER,
            date TEXT NOT NULL
        )",
            (),
        )?;

        c.execute(
            "CREATE TABLE cartoon (
            id INTEGER PRIMARY KEY,
            audio TEXT NOT NULL,
            year INTEGER,
            episode TEXT,
            completed TEXT NOT NULL,
            unique_id INTEGER,
            date TEXT NOT NULL
        )",
            (),
        )?;

        c.execute(
            "CREATE TABLE series (
            id INTEGER PRIMARY KEY,
            audio TEXT NOT NULL,
            year INTEGER,
            episode TEXT,
            completed TEXT NOT NULL,
            unique_id INTEGER,
            date TEXT NOT NULL
        )",
            (),
        )?;

        c.execute(
            "CREATE TABLE movie (
            id INTEGER PRIMARY KEY,
            audio TEXT NOT NULL,
            year INTEGER,
            original TEXT,
            unique_id INTEGER,
            date TEXT NOT NULL
        )",
            (),
        )?;

        c.execute(
            "CREATE TABLE comic (
            id INTEGER PRIMARY KEY,
            year INTEGER,
            chapter TEXT,
            completed TEXT NOT NULL,
            unique_id INTEGER,
            date TEXT NOT NULL
        )",
            (),
        )?;

        c.execute(
            "CREATE TABLE genre (unique_id INTEGER, genre TEXT NOT NULL)",
            (),
        )?;
    }
    Ok(())
}
