use reqwest::{Client, Response};

pub struct JLC {
    user: String,
    password: String,
    host: String,
    port: u32,
    ssl: bool,
    client: Client
}

impl JLC {
    pub fn new(user: String, password: String, host: String, port: u32, ssl: bool) -> Self {
        JLC {
            user,
            password,
            host,
            port,
            ssl,
            client: Client::new()
        }
    }

    pub async fn execute(&self) -> Result<Response, reqwest::Error> {
        let port = self.port.to_string();
        let url = if self.ssl {
            format!("https://{}:{}", self.host, port)
        } else {
            format!("http://{}:{}", self.host, port)
        };

        self.client.get(format!("{url}/j-lawyer-io/rest/v1/cases/list"))
            .basic_auth(&self.user, Some(&self.password))
            .send()
            .await
    }
}