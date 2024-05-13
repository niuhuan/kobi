pub use super::types::*;

pub struct Client {

}

impl Client {

    async fn request<T: for<'de> serde::Deserialize<'de>>(
        &self,
        method: reqwest::Method,
        path: &str,
        params: serde_json::Value,
    ) -> Result<T> {
        let mut obj = query.as_object()?;
        Ok(serde_json::from_str("")?)
    }
    
}
