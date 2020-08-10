#[macro_use]
extern crate serde;
extern crate serde_derive;

mod models;
use models::LearnrEntry;

use warp::Filter;

use std::convert::Infallible;

async fn recv_hello(name: String) -> Result<impl warp::Reply, Infallible> {
    let entry = LearnrEntry::new(vec![name.clone()], Vec::new());
    println!("{:?}", entry);
    entry.insert().await.unwrap();
    return Ok(format!("Hello, {}!", name));
}

#[tokio::main]
async fn main() {
    let hello = warp::path!("hello" / String)
    .and_then(recv_hello);

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}