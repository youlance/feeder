use chrono::NaiveDateTime;
use reqwest::Client;
use uuid::Uuid;

pub struct PosterClient {
    http_client: Client,
    base_url: String,
    path: String
}

impl PosterClient {
    pub fn new(base_url: String, path: String) -> Self {
        PosterClient {
            http_client: Client::new(),
            base_url,
            path
        }
    }

    pub async fn get_posts(&self, followings: Vec<String>, page: u64) -> Result<FeedPosts, reqwest::Error> {
        let url = format!("{}/{}",self.base_url, self.path);
        let request_body = FeedFollowing {
            page: page - 1,
            followings
        };

        let response: FeedPosts = self.http_client
            .post(&url)
            .json(&request_body)
            .send().await?
            .json().await?;

        Ok(response)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub username: String,
    pub img_url: String,
    pub caption: Option<String>,
    pub likes: i32,
    pub created_at: NaiveDateTime
}

#[derive(serde::Serialize)]
struct FeedFollowing {
    pub page: u64,
    pub followings: Vec<String>
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct FeedPosts {
    posts: Vec<Post>
}