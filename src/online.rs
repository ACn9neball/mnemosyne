use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct Response {
    pub data: Vec<Content>,
}

#[derive(Debug, Deserialize)]
pub struct Content {
    pub title: String,
    pub status: String,
    pub score: Option<f32>,
    pub episodes: Option<i32>,
    pub chapters: Option<i32>,
    pub volumes: Option<i32>,
    pub synopsis: Option<String>,
}

#[tokio::main]
pub async fn anime_ranking() -> Result<(), Box<dyn Error>> {
    let url = "https://api.jikan.moe/v4/top/anime";

    let client = reqwest::Client::new();

    let response: Response = client
        .get(url)
        .query(&[("type", "tv")])
        .send()
        .await?
        .json()
        .await?;

    for anime in response.data {
        println!("Title: {}", anime.title);
        match anime.score {
            Some(s) => println!("Score {}/10", s),
            None => println!("N/A"),
        }
        println!("Status: {}", anime.status);
        match anime.episodes {
            Some(e) => println!("Episodes: {}", e),
            None => println!("N/A"),
        }
        match anime.synopsis {
            Some(s) => println!("{}", s),
            None => println!("N/A"),
        }
        println!("");
        println!("");
    }
    Ok(())
}

#[tokio::main]
pub async fn manga_ranking() -> Result<(), Box<dyn Error>> {
    let url = "https://api.jikan.moe/v4/top/manga";

    let client = reqwest::Client::new();

    let response: Response = client.get(url).send().await?.json().await?;

    for manga in response.data {
        println!("Title: {}", manga.title);
        match manga.score {
            Some(s) => println!("Score {}/10", s),
            None => println!("N/A"),
        }
        println!("Status: {}", manga.status);
        match manga.chapters {
            Some(c) => println!("Chapter: {}", c),
            None => match manga.volumes {
                Some(v) => println!("Volume: {}", v),
                None => println!("N/A"),
            },
        }
        match manga.synopsis {
            Some(s) => println!("{}", s),
            None => println!("N/A"),
        }
        println!("");
        println!("");
    }
    Ok(())
}
