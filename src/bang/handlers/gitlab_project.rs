use crate::bang;

pub struct GitlabProject {
    base_url: String,
}

impl GitlabProject {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }
}

impl bang::Handler for GitlabProject {
    fn handle(&self, query: &bang::Query) -> bang::Result {
        bang::Result::Redirect {
            location: format!("{}/{}", self.base_url, query.rest),
        }
    }
}
