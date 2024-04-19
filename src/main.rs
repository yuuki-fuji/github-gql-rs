use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT, AUTHORIZATION };
use serde_json::{json, Value};
use std::fs::File;
use std::io::Write;
use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let token = env::var("GITHUB_TOKEN")?;
    let owner = env::var("OWNER")?;
    let repo_name = env::var("REPO_NAME")?;
    let client = reqwest::Client::new();

    let query = format!(
        r#"query {{
            repository(owner:"{}", name:"{}") {{
                pullRequests(first:10, states:MERGED) {{
                    edges {{
                        node {{
                            number
                            title
                            body
                            author {{
                                login
                            }}
                            comments(first: 5) {{
                                edges {{
                                    node {{
                                        body
                                        author {{
                                            login
                                        }}
                                        createdAt
                                    }}
                                }}
                            }}
                            reviews(first: 5) {{
                                edges {{
                                    node {{
                                        body
                                        author {{
                                            login
                                        }}
                                        state
                                        comments(first: 5) {{
                                            edges {{
                                                node {{
                                                    body
                                                    author {{
                                                        login
                                                    }}
                                                    createdAt
                                                }}
                                            }}
                                        }}
                                    }}
                                }}
                            }}
                        }}
                    }}
                }}
            }}
        }}"#,
        owner, repo_name
    );

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
    headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse().unwrap());

    let res = client.post("https://api.github.com/graphql")
    .headers(headers)
    .body(json!({ "query": query }).to_string())
    .send()
    .await?
    .text()
    .await?;

    let mut file = File::create("github_data.txt")?;
    file.write_all(res.as_bytes())?;

    Ok(())
}
