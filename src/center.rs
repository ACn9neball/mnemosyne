const DB: &str = "database.db";

use cli_table::*;
use rusqlite::{Connection, Result};
use std::{error::Error, io};

#[derive(Debug)]
struct Main {
    unique_id: i64,
    title: String,
}

pub fn update(id: i64) -> Result<()> {
    let c = Connection::open(DB)?;
    println!("Title");
    let title: String = input();
    let main = Main {
        unique_id: 0,
        title: title.clone(),
    };
    c.execute(
        "UPDATE main SET title = ?2 WHERE unique_id = ?1",
        (id, &main.title),
    )?;
    println!("{} added.", title);

    Ok(())
}

pub fn display() -> Result<(), Box<dyn Error>> {
    let c = Connection::open(DB)?;
    let mut all = c.prepare("SELECT * FROM main")?;
    let main_iter = all.query_map([], |row| {
        Ok(Main {
            unique_id: row.get(0)?,
            title: row.get(1)?,
        })
    })?;
    let mut data = Vec::new();
    for main in main_iter {
        let m = main.unwrap();
        data.push(vec![
            m.unique_id.cell().bold(true),
            m.title.cell().bold(true),
        ]);
    }

    let table = data
        .table()
        .title(vec![
            "Unique_ID".cell().bold(true),
            "Title".cell().bold(true),
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
