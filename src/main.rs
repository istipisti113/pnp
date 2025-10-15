use warp::{Filter};
//use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let port = 4040;
    println!("running on port {}",port);

    let style = warp::path("style.css").and(warp::fs::file("style.css"));
    let bootstrapcss = warp::path("bootstrap.css").and(warp::fs::file("bootstrap.css"));
    let bootstrapjs = warp::path("bootstrap.js").and(warp::fs::file("bootstrap.js"));
    let home = warp::path::end().map(||{
        warp::reply::html(std::fs::read("index.html").unwrap())
    });

    let routes = home.or(style).or(bootstrapcss);
    warp::serve(routes).run(([0,0,0,0], port)).await;
}
