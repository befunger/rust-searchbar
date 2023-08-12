#![feature(proc_macro_hygiene, decl_macro)]

//#[macro_use] extern crate rocket;
use rocket::routes;
use rocket::get;
use rocket::response::Redirect;
mod utils;

#[get("/")]
fn index() -> &'static str {
    "hello world!"
}

#[get("/search?<cmd>")]
fn search(cmd: String) -> Redirect {
    let command = utils::get_command_from_query_string(&cmd);

    let redirect_url = match command {
        "reddit" => String::from("https://reddit.com"),
        "git" => utils::github::construct_github_url(&cmd, &command),
        "yt" => utils::youtube::construct_youtube_search_url(&cmd, &command),
        "maps" => utils::maps::construct_maps_search_url(&cmd, &command),
        _ => utils::google::construct_google_search_url(&cmd)
    };
    println!("Query recieved is {} with command {}", cmd, command);
    println!("Redirected to {}", redirect_url);
    Redirect::to(redirect_url)
}

fn main() {
    rocket::ignite().mount("/", routes![index, search]).launch();
}