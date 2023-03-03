use derive_builder::Builder;
use reqwest::{header::HeaderMap, Client, RequestBuilder};

use crate::{DocsClient, ReposClient, RequestMethod, YuqueError, DEFAULT_USER_AGENT};

/// The client of yuque.
///
/// # Example
///
/// ```
/// use yuque_rust::Yuque;
///
/// let yuque = Yuque::builder()
///                     .token("token")
///                     .host("example_host")
///                     .build()
///                     .unwrap();
///
/// ```
#[derive(Default, Builder, Clone, Debug)]
pub struct Yuque {
    #[builder(default = "Client::default()")]
    pub(crate) client: Client,
    pub(crate) token: String,
    pub(crate) host: String,
    #[builder(default = "DEFAULT_USER_AGENT.into()")]
    pub(crate) user_agent: String,
}

impl Yuque {
    pub fn builder() -> YuqueBuilder {
        YuqueBuilder::default()
    }

    /// Generate headers for sending to the yuque server.
    ///
    /// # Returns
    ///
    /// * `Result<HeaderMap, YuqueError>` - The headers wrapped in a result.
    ///
    /// # Example
    ///
    /// ```
    /// use yuque_rust::Yuque;
    ///
    ///
    /// let yuque = Yuque::builder()
    ///                     .token("token")
    ///                     .host("example_host")
    ///                     .build()
    ///                     .unwrap();
    ///   
    /// let headers = yuque.generate_headers().unwrap();
    ///
    /// assert_eq!(headers.get("X-Auth-Token").unwrap(), "token");
    /// assert_eq!(headers.get("User-Agent").unwrap(), "@yuque/sdk");
    ///
    /// ```
    pub fn generate_headers(&self) -> Result<HeaderMap, YuqueError> {
        let mut headers = HeaderMap::new();

        headers.insert("X-Auth-Token", self.token.parse()?);
        headers.insert("User-Agent", self.user_agent.parse()?);

        Ok(headers)
    }

    /// Generate requests for sending to the yuque server.
    ///
    /// # Arguments
    ///
    /// * `method` - The request method.
    /// * `token` - The token of the user.
    /// * `api` - The api of the request.
    /// * `data` - The data of the request.
    ///
    /// # Returns
    ///
    /// * `Result<RequestBuilder, YuqueError>` - The request builder wrapped in a result.
    fn request(
        &self,
        method: crate::RequestMethod,
        api: &str,
        data: Option<String>,
    ) -> Result<RequestBuilder, YuqueError> {
        let url = format!("{}{}", self.host, api);

        let request_builder: RequestBuilder = match method {
            crate::RequestMethod::Get => self.client.get(url).headers(self.generate_headers()?),
            crate::RequestMethod::Post => {
                let builder = self
                    .client
                    .post(url)
                    .headers(self.generate_headers()?)
                    .header("Content-Type", "application/json");

                if let Some(data) = data {
                    builder.body(data)
                } else {
                    builder
                }
            }
            crate::RequestMethod::Put => {
                let builder = self
                    .client
                    .put(url)
                    .headers(self.generate_headers()?)
                    .header("Content-Type", "application/json");

                if let Some(data) = data {
                    builder.body(data)
                } else {
                    builder
                }
            }
            crate::RequestMethod::Delete => {
                self.client.delete(url).headers(self.generate_headers()?)
            }
        };

        Ok(request_builder)
    }

    /// Generate a GET request for sending to the yuque server.
    ///
    /// # Arguments
    ///
    /// * `api` - The api of the request.
    ///
    /// # Returns
    ///
    /// * `Result<RequestBuilder, YuqueError>` - The request builder wrapped in a result.
    ///
    /// # Example
    ///
    /// ```rust
    /// use yuque_rust::Yuque;
    ///
    /// let client = Yuque::builder()
    ///                     .token("token")
    ///                     .host("example_host")
    ///                     .build()
    ///                     .unwrap();
    ///
    /// let response = client.get("example").unwrap().send().await.unwrap();
    ///
    /// println!("{:#?}", response);
    /// ```
    ///
    pub fn get(&self, api: &str) -> Result<RequestBuilder, YuqueError> {
        self.request(RequestMethod::Get, api, None)
    }

    /// Generate a POST request for sending to the yuque server.
    ///
    /// # Arguments
    ///
    /// * `api` - The api of the request.
    /// * `data` - The data of the request.
    ///
    /// # Returns
    ///
    /// * `Result<RequestBuilder, YuqueError>` - The request builder wrapped in a result.
    ///
    /// # Example
    ///
    /// ```rust
    /// use yuque_rust::Yuque;
    ///
    /// let client = Yuque::builder()
    ///                     .token("token")
    ///                     .host("example_host")
    ///                     .build()
    ///                     .unwrap();
    ///
    /// let response = client.post("example", Some("data".into())).unwrap().send().await.unwrap();
    ///
    /// println!("{:#?}", response);
    ///
    /// ```
    pub fn post(&self, api: &str, data: Option<String>) -> Result<RequestBuilder, YuqueError> {
        self.request(RequestMethod::Post, api, data)
    }

    /// Generate a PUT request for sending to the yuque server.
    ///
    /// # Arguments
    ///
    /// * `api` - The api of the request.
    /// * `data` - The data of the request.
    ///
    /// # Returns
    ///
    /// * `Result<RequestBuilder, YuqueError>` - The request builder wrapped in a result.
    ///
    /// # Example
    ///
    /// ```rust
    /// use yuque_rust::Yuque;
    ///
    /// let client = Yuque::builder()
    ///                     .token("token")
    ///                     .host("example_host")
    ///                     .build()
    ///                     .unwrap();
    ///
    /// let response = client.put("example", Some("data".into())).unwrap().send().await.unwrap();
    ///
    /// println!("{:#?}", response);
    ///
    /// ```
    pub fn put(&self, api: &str, data: Option<String>) -> Result<RequestBuilder, YuqueError> {
        self.request(RequestMethod::Put, api, data)
    }

    /// Generate a DELETE request for sending to the yuque server.
    ///
    /// # Arguments
    ///
    /// * `api` - The api of the request.
    ///
    /// # Returns
    ///
    /// * `Result<RequestBuilder, YuqueError>` - The request builder wrapped in a result.
    ///
    /// # Example
    ///
    /// ```rust
    /// use yuque_rust::Yuque;
    /// let client = Yuque::builder()
    ///                    .token("token")
    ///                    .host("example_host")
    ///                    .build()
    ///                    .unwrap();
    ///
    /// let response = client.delete("example").unwrap().send().await.unwrap();
    ///
    /// println!("{:#?}", response);
    /// ```
    pub fn delete(&self, api: &str) -> Result<RequestBuilder, YuqueError> {
        self.request(RequestMethod::Delete, api, None)
    }

    /// Get the client aimed to handle yuque doc.
    ///
    /// # Returns
    ///
    /// * `DocsClient` - The client aimed to handle yuque doc.
    ///
    /// # Example
    ///
    /// ```rust
    /// use yuque_rust::Yuque;
    ///
    /// let client = Yuque::builder()
    ///                   .token("token")
    ///                   .host("example_host")
    ///                   .build()
    ///                   .unwrap();
    ///
    /// let docs_client = client.docs();
    ///
    /// let response = docs_client.get("example").unwrap().send().await.unwrap();
    ///
    /// println!("{:#?}", response);
    /// ```
    pub fn docs(&self) -> DocsClient {
        DocsClient {
            client: self.clone(),
        }
    }
    /// Get the client aimed to handle yuque repo.
    ///
    /// # Returns
    ///
    /// * `ReposClient` - The client aimed to handle yuque repo.
    ///
    /// # Example
    ///
    /// ```rust
    /// use yuque_rust::Yuque;
    ///
    /// let client = Yuque::builder()
    ///                     .token("token")
    ///                     .host("example_host")
    ///                     .build()
    ///                     .unwrap();
    ///
    /// let repos_client = client.repos();
    ///
    /// let response = repos_client.get("example").unwrap().send().await.unwrap();
    ///
    /// println!("{:#?}", response);
    /// ```
    pub fn repos(&self) -> ReposClient {
        ReposClient {
            client: self.clone(),
        }
    }
}
