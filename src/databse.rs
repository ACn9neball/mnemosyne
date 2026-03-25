use rusqlite::{Connection, Result};

pub fn db_setup(data: &str) -> Result<()> {
    let c = Connection::open(data)?;

    c.execute(
        "CREATE TABLE IF NOT EXISTS main (unique_id INTEGER PRIMARY KEY, title TEXT NOT NULL)",
        (),
    )?;

    c.execute(
        "CREATE TABLE IF NOT EXISTS anime (
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
        "CREATE TABLE IF NOT EXISTS manga (
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
        "CREATE TABLE IF NOT EXISTS cartoon (
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
        "CREATE TABLE IF NOT EXISTS series (
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
        "CREATE TABLE IF NOT EXISTS movie (
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
        "CREATE TABLE IF NOT EXISTS comic (
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
        "CREATE TABLE IF NOT EXISTS genre (unique_id INTEGER, genre TEXT NOT NULL)",
        (),
    )?;

    c.execute(
        "CREATE TABLE IF NOT EXISTS game (
            id INTEGER PRIMARY KEY,
            year INTEGER,
            original TEXT,
            unique_id INTEGER,
            genre TEXT,
            date TEXT NOT NULL
        )",
        (),
    )?;

    c.execute(
        "CREATE TABLE IF NOT EXISTS project (
            id INTEGER PRIMARY KEY,
            language TEXT NOT NULL,
            year INTEGER,
            completed TEXT NOT NULL,
            description TEXT,
            commited TEXT NOT NULL,
            unique_id INTEGER,  
            date TEXT NOT NULL
        )",
        (),
    )?;

    Ok(())
}
