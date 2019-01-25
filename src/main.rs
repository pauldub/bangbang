#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate nom;
#[macro_use]
extern crate serde_derive;

extern crate percent_encoding;

mod bang;

use rocket::State;

use bang::handlers::{DuckDuckGo, GitlabProject};

#[get("/?<q>")]
fn handle_bang_query(q: String, dispatcher: State<bang::Dispatcher>) -> bang::Result {
    dispatcher.dispatch(bang::Query::parse(&q))
}

fn main() {
    let gitlab = GitlabProject::new("https://gitlab.com");
    let ddg = DuckDuckGo::new();

    let mut dispatcher = bang::Dispatcher::new(Box::new(ddg));
    dispatcher.add("gl", Box::new(gitlab));

    rocket::ignite()
        .manage(dispatcher)
        .mount("/", routes![handle_bang_query])
        .launch();
}
