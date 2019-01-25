use crate::bang;

use percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};

#[derive(Deserialize, Debug)]
pub struct DuckDuckGo {}

impl DuckDuckGo {
    pub fn new() -> Self {
        Self {}
    }
}

impl bang::Handler for DuckDuckGo {
    fn handle(&self, query: &bang::Query) -> bang::Result {
        let full_query = if let Some(bang) = query.bang {
            format!("!{} {}", bang, query.rest)
        } else {
            query.rest.to_string()
        };

        let encoded_query = utf8_percent_encode(&full_query, DEFAULT_ENCODE_SET);

        bang::Result::Redirect {
            location: format!("https://duckduckgo.com/?q={}", encoded_query),
        }
    }
}
