use rocket::{fs::FileServer, get, launch, routes};
use rocket_dyn_templates::{Template, context};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Repo {
    id: u64,
    name: String,
    full_name: String,
    html_url: String,
    language: Option<String>,
    description: Option<String>,
    license: Option<License>,
    private: bool,
    owner: Owner,
    homepage: Option<String>,
    stargazers_count: u64,
    forks_count: u64,
    open_issues_count: u64,
    created_at: String,
    updated_at: String,
}
/// Represents GitHub repository license information.
#[derive(Debug, Serialize, Deserialize)]
struct License {
    spdx_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Owner {
    login: String,
    id: u64,
    avatar_url: String,
    html_url: String,
}

fn config() -> (String, String, Vec<String>) {
    let hub_toml = std::fs::read_to_string("hub.toml").expect("Unable to read hub.toml");
    let value: toml::Value = toml::from_str(&hub_toml).expect("Unable to parse hub.toml");
    if let Some(name) = value.get("name") {
        if let Some(email) = value.get("email") {
            if let Some(orgs) = value.get("orgs") {
                let orgs_vec: Vec<String> = orgs
                    .as_array()
                    .expect("orgs is not an array")
                    .iter()
                    .map(|o| o.as_str().expect("org is not a string").to_string())
                    .collect();
                return (
                    name.as_str().expect("name is not a string").to_string(),
                    email.as_str().expect("email is not a string").to_string(),
                    orgs_vec,
                );
            } else {
                panic!("orgs not found in hub.toml");
            }
        } else {
            panic!("email not found in hub.toml");
        }
    } else {
        panic!("name not found in hub.toml");
    }
}

async fn my_repositories() -> Vec<Repo> {
    let (name, _email, organisations) = config();
    let url = format!("https://api.github.com/users/{name}/repos");
    let token = std::env::var("GITHUB_TOKEN").unwrap_or_default();
    let client = reqwest::Client::new();
    let mut repos: Vec<Repo> = client
        .get(&url)
        .header("User-Agent", "RustExample")
        .bearer_auth(token.to_string())
        .send()
        .await
        .expect("msg")
        .json()
        .await
        .expect("msg");
    for org in &organisations {
        let org_url = format!("https://api.github.com/users/{org}/repos");
        let mut org_repos: Vec<Repo> = client
            .get(&org_url)
            .header("User-Agent", "RustExample")
            .bearer_auth(token.to_string())
            .send()
            .await
            .expect("msg")
            .json()
            .await
            .expect("msg");
        repos.append(&mut org_repos);
    }
    repos
}

#[get("/me")]
async fn search_repository() -> Template {
    let me: Vec<Repo> = my_repositories().await;
    Template::render("me", context! { repos: me })
}
#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, search_repository])
        .mount("/", FileServer::from("public"))
        .attach(Template::fairing())
}
