use crate::bang;

use percent_encoding::{
  utf8_percent_encode,
  DEFAULT_ENCODE_SET
};

pub struct DuckDuckGo { }

impl DuckDuckGo {
    pub fn new() -> Self {
        Self{}
    }
}

impl bang::Handler for DuckDuckGo {
        fn handle(&self, query: &bang::Query) -> bang::Result {
            let encoded_query = utf8_percent_encode(query.rest, DEFAULT_ENCODE_SET);

                bang::Result::Redirect {
                        location: format!("https://duckduckgo.com/q?={}", encoded_query),
                }
        }
}
