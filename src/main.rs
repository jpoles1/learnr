#[macro_use]
extern crate serde;
extern crate serde_derive;
extern crate pretty_env_logger;
extern crate async_trait;

mod models;
use models::{MongoSchema, LearnrEntry};

use warp::Filter;

use std::convert::Infallible;

async fn recv_hello(name: String) -> Result<impl warp::Reply, Infallible> {
    let entry = LearnrEntry::new(vec![name.clone()], Vec::new());
    println!("{:?}", entry);
    entry.insert().await.unwrap();
    return Ok(format!("Hello, {}!", name));
}

async fn entry_get_handler(id: String) -> Result<impl warp::Reply, Infallible> {
    return Ok("")
}

async fn entry_post_handler(entry: LearnrEntry) -> Result<impl warp::Reply, Infallible> {
    let res = entry.insert().await;
    return Ok(match res {
        Ok(_) => "Entry added".to_owned(),
        Err(e) => format!("Failed to add entry: {}", e),
    });
}

async fn entry_delete_handler(entry: LearnrEntry) -> Result<impl warp::Reply, Infallible> {
    let res = entry.delete().await;
    return Ok(match res {
        Ok(_) => "Entry deleted".to_owned(),
        Err(e) => format!("Failed to delete entry: {}", e),
    });
}

pub async fn start_server() {
    println!("Starting learnr server!");
    let hello = warp::path!("hello" / String)
    .and_then(recv_hello);
    
    let entry_get = warp::get().and(warp::path::param()).and_then(entry_get_handler);
    let entry_post = warp::post().and(warp::body::json()).and_then(entry_post_handler);
    let entry_delete = warp::delete().and(warp::body::json()).and_then(entry_delete_handler);

    let entry_routes = warp::path!("entry").and(entry_get.or(entry_delete).or(entry_post));

    let router = hello.or(entry_routes).with(warp::log("router"));
    warp::serve(router)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    start_server().await;
}

mod tests {
    use restson::RestClient;

    use super::*;

    #[test]
    fn test_insert_entry() {
        let mut client = RestClient::new("http://127.0.0.1:3030").unwrap();
        let entry = LearnrEntry::new(vec!["tst".to_owned()], vec!["sts".to_owned()]);
        client.post((), &entry).unwrap();
    }
}