use std::sync::Arc;
use reqwest::Client;
use serde_json::Value;

pub struct Http {
    client: Arc<Client>,
    token: String,
}

impl Http {
    pub fn new(token: String) -> Self {
        Http {
            client: Arc::new(Client::new()),
            token,
        }
    }

    async fn get<T: for<'de> serde::Deserialize<'de>>(&self, endpoint: &str) -> Result<T, reqwest::Error> {
        let url = format!("https://discord.com/api/v10{}", endpoint);
        let res = self.client
            .get(&url)
            .bearer_auth(&self.token)
            .send()
            .await?
            .json::<T>()
            .await?;
        Ok(res)
    }

    async fn post<T: for<'de> serde::Deserialize<'de>>(&self, endpoint: &str, body: Value) -> Result<T, reqwest::Error> {
        let url = format!("https://discord.com/api/v10{}", endpoint);
        let res = self.client
            .post(&url)
            .bearer_auth(&self.token)
            .json(&body)
            .send()
            .await?
            .json::<T>()
            .await?;
        Ok(res)
    }

    async fn patch<T: for<'de> serde::Deserialize<'de>>(&self, endpoint: &str, body: Value) -> Result<T, reqwest::Error> {
        let url = format!("https://discord.com/api/v10{}", endpoint);
        let res = self.client
            .patch(&url)
            .bearer_auth(&self.token)
            .json(&body)
            .send()
            .await?
            .json::<T>()
            .await?;
        Ok(res)
    }

    async fn delete<T: for<'de> serde::Deserialize<'de>>(&self, endpoint: &str) -> Result<T, reqwest::Error> {
        let url = format!("https://discord.com/api/v10{}", endpoint);
        let res = self.client
            .delete(&url)
            .bearer_auth(&self.token)
            .send()
            .await?
            .json::<T>()
            .await?;
        Ok(res)
    }
}
