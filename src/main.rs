use std::collections::HashSet;
use std::env;
use std::net::IpAddr;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use warp::filters::{body, method};
use warp::http::StatusCode;
use warp::{path, Filter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = env::var("HTTP_ADDRESS")
        .unwrap_or("::1".to_string())
        .parse::<IpAddr>()?;
    let port = env::var("HTTP_PORT")
        .unwrap_or("8080".to_string())
        .parse::<u16>()?;

    let beer_repository = Arc::new(Mutex::new(HashSet::new()));
    let beer_repository = warp::any().map(move || Arc::clone(&beer_repository));

    let hello = path::end().and(method::get()).map(|| "Habe die Ehre!");

    let beers_get = method::get()
        .and(beer_repository.clone())
        .and_then(list_beers);
    let beers_post = method::post()
        .and(body::json())
        .and(beer_repository.clone())
        .and_then(add_beer);
    let beers = path("beers").and(path::end()).and(beers_get.or(beers_post));

    let routes = hello.or(beers);
    warp::serve(routes).run((addr, port)).await;

    Ok(())
}

async fn list_beers(
    beer_repository: Arc<Mutex<HashSet<Beer>>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let beer_repository = beer_repository.lock().await;
    let mut beers = Vec::new();
    for beer in beer_repository.iter() {
        beers.push(beer);
    }
    Ok(warp::reply::json(&beers))
}

async fn add_beer(
    beer: Beer,
    beer_repository: Arc<Mutex<HashSet<Beer>>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut beer_repository = beer_repository.lock().await;
    beer_repository.insert(beer);
    Ok(StatusCode::CREATED)
}

#[derive(Eq, PartialEq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Beer {
    brewery: String,
    name: String,
}