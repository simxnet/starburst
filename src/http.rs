use std::sync::Arc;
use reqwest::Client;

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

    pub async fn get<T: for<'de> serde::Deserialize<'de>>(&self, endpoint: &str) -> Result<T, reqwest::Error> {
        let url = format!("https://discord.com/api/v10{}", endpoint);
        let res = self.client
            .get(&url)
            .header("authorization", format!("Bot {}", self.token))
            .send()
            .await?
            .json::<T>()
            .await?;
        Ok(res)
    }

    pub async fn post<T: for<'de> serde::Deserialize<'de>>(&self, endpoint: &str, body: for<'de> serde::Deserialize<'de>) -> Result<T, reqwest::Error> {
        let url = format!("https://discord.com/api/v10{}", endpoint);
        let res = self.client
            .post(&url)
            .header("authorization", format!("Bot {}", self.token))
            .json(&body)
            .send()
            .await?
            .json::<T>()
            .await?;
        Ok(res)
    }

    pub async fn patch<T: for<'de> serde::Deserialize<'de>>(&self, endpoint: &str, body: for<'de> serde::Deserialize<'de>) -> Result<T, reqwest::Error> {
        let url = format!("https://discord.com/api/v10{}", endpoint);
        let res = self.client
            .patch(&url)
            .header("authorization", format!("Bot {}", self.token))
            .json(&body)
            .send()
            .await?
            .json::<T>()
            .await?;
        Ok(res)
    }

    pub async fn delete<T: for<'de> serde::Deserialize<'de>>(&self, endpoint: &str) -> Result<T, reqwest::Error> {
        let url = format!("https://discord.com/api/v10{}", endpoint);
        let res = self.client
            .delete(&url)
            .header("authorization", format!("Bot {}", self.token))
            .send()
            .await?
            .json::<T>()
            .await?;
        Ok(res)
    }
}
