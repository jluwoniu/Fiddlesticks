use base64::{engine::general_purpose, Engine};
use reqwest::Client;
use reqwest::{header, Certificate};
use serde::Serialize;
use std::time::Duration;
use sysinfo::{ProcessExt, System, SystemExt};
#[derive(Debug)]
pub struct Credentials {
    name: String,
    password: String,
    port: String,
}

/// name只能为LeagueClientUx.exe或者RiotClientUx.exe
pub fn authenticate(name: &str) -> Result<Credentials, &str> {
    let mut sys = System::new_all();
    sys.refresh_processes();

    let args = sys
        .processes()
        .values()
        .find(|p| p.name() == name)
        .map(|p| p.cmd())
        .ok_or("解析进程参数失败")?;

    let port = args
        .iter()
        .find(|arg| arg.starts_with("--app-port="))
        .map(|arg| arg.strip_prefix("--app-port=").unwrap().to_string())
        .ok_or("解析端口信息失败")?;
    let password = args
        .iter()
        .find(|arg| arg.starts_with("--remoting-auth-token="))
        .map(|arg| {
            arg.strip_prefix("--remoting-auth-token=")
                .unwrap()
                .to_string()
        })
        .ok_or("解析token失败")?;
    Ok(Credentials {
        name: "riot".to_string(),
        password,
        port,
    })
}

type Error = Box<dyn std::error::Error>;
pub struct LeagueClient {
    credentials: Credentials,
    client: Client,
}

impl LeagueClient {
    /// Create a new instance of the LCU REST wrapper
    pub fn new() -> Result<Self, Error> {
        let credentials = authenticate("LeagueClientUx.exe")?;
        let token = general_purpose::STANDARD
            .encode(format!("{}:{}", credentials.name, credentials.password));
        let client = build_reqwest_client(Some(token));
        Ok(Self {
            credentials,
            client,
        })
    }

    /// Make a get request to the specified endpoint
    pub async fn get(&self, endpoint: String) -> Result<serde_json::Value, reqwest::Error> {
        self.client
            .get(format!(
                "https://127.0.0.1:{}{}",
                self.credentials.port, endpoint
            ))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
            .or_else(|_| Ok(serde_json::Value::Null))
    }

    /// Make a post request to the specified endpoint
    pub async fn post<T: Serialize>(
        &self,
        endpoint: String,
        body: T,
    ) -> Result<serde_json::Value, reqwest::Error> {
        self.client
            .post(format!(
                "https://127.0.0.1:{}{}",
                self.credentials.port, endpoint
            ))
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
            .or_else(|_| Ok(serde_json::Value::Null))
    }

    /// Make a put request to the specified endpoint
    pub async fn put<T: Serialize>(
        &self,
        endpoint: String,
        body: T,
    ) -> Result<serde_json::Value, reqwest::Error> {
        self.client
            .put(format!(
                "https://127.0.0.1:{}{}",
                self.credentials.port, endpoint
            ))
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
            .or_else(|_| Ok(serde_json::Value::Null))
    }

    /// Make a delete request to the specified endpoint
    pub async fn delete(&self, endpoint: String) -> Result<serde_json::Value, reqwest::Error> {
        self.client
            .delete(format!(
                "https://127.0.0.1:{}{}",
                self.credentials.port, endpoint
            ))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
            .or_else(|_| Ok(serde_json::Value::Null))
    }
}

pub(crate) fn build_reqwest_client(auth_token: Option<String>) -> reqwest::Client {
    let cert = Certificate::from_pem(include_bytes!("./riotgames.pem")).unwrap();
    let mut headers = header::HeaderMap::new();

    if let Some(token) = auth_token {
        let auth_header =
            header::HeaderValue::from_str(format!("Basic {}", token).as_str()).unwrap();
        headers.insert("Authorization", auth_header);
    }

    // todo 设置不使用代理
    reqwest::ClientBuilder::new()
        .add_root_certificate(cert)
        .default_headers(headers)
        .timeout(Duration::from_millis(500))
        .build()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authenticate() {
        let result = authenticate("LeagueClientUx.exe");
        // 断言result为Ok
        assert!(result.is_ok());
    }

    #[test]
    fn test_league_client_new() {
        let result = LeagueClient::new();
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_league_client_get() {
        let client = LeagueClient::new().unwrap();
        let result = client
            .get("/lol-summoner/v1/current-summoner".to_owned())
            .await;
        assert!(result.is_ok());
        println!("{:?}", result);
    }
}
