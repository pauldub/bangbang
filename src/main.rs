#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate nom;

mod bang;

struct GitlabProject {}

impl bang::Handler for GitlabProject {
        fn handle(&self, _query: bang::Query) -> bang::Result {
                bang::Result::Redirect {
                        location: "https://example.com".to_string(),
                }
        }
}

use rocket::State;

#[get("/?<q>")]
fn handle_bang_query(q: String, dispatcher: State<bang::Dispatcher>) -> bang::Result {
        dispatcher.dispatch(bang::Query::parse(&q))
}

fn main() {
        let mut dispatcher = bang::Dispatcher::new();
        dispatcher.add("gl", &GitlabProject {});

        rocket::ignite()
                .manage(dispatcher)
                .mount("/", routes![handle_bang_query])
                .launch();
}
