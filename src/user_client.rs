use reqwest::Client;
use tracing::info;

pub struct UserClient {
    http_client: Client,
    base_url: String,
    count_path: String,
    fetch_path: String
}

impl UserClient {
    pub fn new(base_url: String, count_path: String, fetch_path: String) -> Self {
        UserClient {
            http_client: Client::new(),
            base_url,
            count_path,
            fetch_path
        }
    }

    pub async fn get_following_count(&self, username: &str) -> Result<u64, reqwest::Error> {
        let url = format!("{}/{}/{}", self.base_url, self.count_path, username);
        dbg!(&url);
        let response: CountFollowee = self.http_client
            .get(&url)
            .send().await?
            .json().await?;

        Ok(response.count)
    }

    pub async fn get_followees(&self, username: &str, page_size: u64) -> Result<Vec<String>, reqwest::Error> {
        let url = format!("{}/{}", self.base_url, self.fetch_path);
        let request_body = RequestFollowees {
            follower_id: username.into(),
            page_id: 1,
            page_size: page_size as i32
        };

        let list: Vec<Followee> = self.http_client
            .get(&url)
            .json(&request_body)
            .send().await?
            .json().await?;


        let followees: Vec<String> = list
            .into_iter()
            .map(|f| {
                f.followee_id
            }).collect();

        Ok(followees)
    }

}

#[derive(serde::Deserialize)]
struct CountFollowee {
    count: u64
}

#[derive(Debug, serde::Deserialize)]
struct Followee {
    #[allow(dead_code)]
    follower_id: String,
    followee_id: String
}

#[derive(Debug, serde::Deserialize)]
struct ListFollowees {
    list: Vec<Followee>
}

#[derive(serde::Serialize)]
struct RequestFollowees {
    follower_id: String,
    page_id: i32,
    page_size: i32
}
