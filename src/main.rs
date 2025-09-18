use warp::{Filter};

#[tokio::main]
async fn main() {
    let port = 4040;
    let home = warp::path::end().map(||{});

    let routes = home;
    warp::serve(routes).run(([0,0,0,0], port)).await;
}
