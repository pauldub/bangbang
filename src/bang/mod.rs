pub mod query;

pub use query::Query;

use rocket;
use rocket::response;

use std::collections::HashMap;

pub enum Result {
        Redirect { location: String },
        BadRequest,
}

impl<'a> response::Responder<'a> for Result {
        fn respond_to(self, request: &rocket::Request) -> response::Result<'a> {
                match self {
                        Result::Redirect { location } => {
                                response::Redirect::to(location).respond_to(request)
                        }
                        _ => response::status::BadRequest(Some("could not process query"))
                                .respond_to(request),
                }
        }
}

pub trait Handler: Send + Sync {
        fn handle(&self, query: query::Query) -> Result;
}

pub struct Dispatcher<'a, 'b> {
        pub bangs: HashMap<&'a str, Box<&'b Handler>>,
}

impl<'a, 'b> Dispatcher<'a, 'b> {
        pub fn new() -> Self {
                Self {
                        bangs: HashMap::new(),
                }
        }

        pub fn add(&mut self, key: &'a str, handler: &'b Handler) {
                self.bangs.entry(key).or_insert(Box::new(handler));
        }

        pub fn dispatch(&self, query: Query) -> Result {
                query.bang
                        .and_then(|bang| self.bangs.get(bang))
                        .map(|handler| handler.handle(query))
                        .unwrap_or(Result::BadRequest)
        }
}
