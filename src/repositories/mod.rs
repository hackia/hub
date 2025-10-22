#[derive(Clone)]
pub enum Source {
    GitHub,
    GitLab,
}
#[derive(Clone)]
pub struct Search {
    source: Source,
    base_url: String,
    q: String,
    token: String,
}

pub struct SearchResults {
    source: Source,
    description: String,
    url: String,
}

fn base_url(source: Source) -> String {
    match source {
        Source::GitHub => String::from("https://api.github.com"),
        Source::GitLab => String::from("https://gitlab.com/api/v4"),
    }
}
fn token(source: Source) -> String {
    match source {
        Source::GitHub => std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN required"),
        Source::GitLab => std::env::var("GITLAB_TOKEN").expect("GITLAB_TOKEN required"),
    }
}
impl SearchResults {
    pub fn new(search: &mut Search) -> Self {
        let source = search.source.clone();
        Self {
            source: source.clone(),
            description: String::new(),
            url: String::new(),
        }
    }
}
impl Search {
    #[must_use]
    pub fn new(source: Source) -> Self {
        Self {
            source: source.clone(),
            q: String::new(),
            base_url: base_url(source.clone()),
            token: token(source.clone()),
        }
    }
    
    pub fn set_q(&mut self, q: &str) -> &mut Self {
        self.q.clear();
        match self.source {
            Source::GitHub => self.q.push_str(format!("{q}").as_str()),
            Source::GitLab => self.q.push_str(format!("{q}").as_str()),
        }
        self
    }
    pub fn get(&mut self) -> SearchResults {
        SearchResults::new(self)
    }
}
