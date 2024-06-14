use {
    getset::{Getters, Setters},
    maybe_async::async_impl,
    reqwest::header::{
        HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, USER_AGENT,
    },
    reqwest::Method,
    reqwest_middleware::ClientBuilder,
    reqwest_middleware::ClientWithMiddleware,
    reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware},
    serde::{de::DeserializeOwned, Serialize},
    serde_json,
    std::time::Duration,
    std::{
        borrow::Borrow,
        convert::TryInto,
        fmt::Debug,
        ops::{Deref, DerefMut},
    },
    tracing,
};
/// Default values
const DEFAULT_TIMEOUT: u64 = 30 * 1000;
const DEFAULT_MIN_RETRY_INTERVAL: u64 = 100;
const DEFAULT_MAX_RETRY_INTERVAL: u64 = 5 * 1000;
const DEFAULT_MAX_RETRIES: u32 = 3;
// ────────────────────────────────────────────────────────────
#[derive(Debug, Clone)]
/// Client is a client that uses the reqwest crate to make HTTP requests.
// NOTE: do not implement Deref traits as deref coercion would prevent Client
// methods to get called
pub struct Client(ClientWithMiddleware);

// ────────────────────────────────────────────────────────────
#[allow(dead_code)]
impl Client {
    /// creates a new api client that is essentially a wrapper around reqwest_middleware::ClientWithMiddleware
    pub fn new(client: ClientWithMiddleware) -> Self {
        let span: tracing::Span =
            tracing::span!(tracing::Level::INFO, "Client");
        let _guard = span.enter();
        let span: tracing::Span = tracing::span!(tracing::Level::INFO, "new");
        let _guard = span.enter();

        Client(client)
    }
    /// executes the actual HTTP request
    ///
    /// ## Parameters
    /// * `method` : Http Verb
    /// * `endpoint` : url suffix that get's appended to the base url that was set in client builder to form the final endpoint address http request is sent to
    /// * `headers` : optional headers to add to the request
    /// * `mutator_fn` : a function that allows changing the underlying request builder (e.g add data)
    async fn exec<T, R>(
        &self,
        method: reqwest::Method,
        url: &str,
        headers: Option<&super::Headers>,
        mutator_fn: T,
    ) -> miette::Result<serde_json::Value, super::Error>
    where
        R: DeserializeOwned + Serialize + Debug,
        T: Fn(
            reqwest_middleware::RequestBuilder,
        ) -> reqwest_middleware::RequestBuilder,
    {
        let span: tracing::Span = tracing::span!(tracing::Level::INFO, "exec");
        let _guard = span.enter();
        tracing::info!("\nsending request to {}", url);
        // let url = [self.url_prefix.as_str(), "/", endpoint.as_ref()].concat();
        let request = self.0.request(method, url);
        let request = match headers {
            Some(headers) => {
                // tracing::debug!(
                //     "\nadding headers to request\n{}",
                //     serde_json::to_string(&headers).unwrap()
                // );
                tracing::debug!("\nadding headers to request",);

                // NOTE: The headers need to be converted
                // into a `reqwest::HeaderMap`, which won't fail as long as
                // its contents are ASCII. This is an internal function, so the
                // condition cannot be broken by the user and will always be
                // true.

                let headers: reqwest::header::HeaderMap = headers
                    .try_into()
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
                    .map_err(|e| {
                        let e = super::Error::Response {
                            canonical_reason: e.to_string(),
                            status: None,
                            body: None,
                        };
                        // tracing::error!("\n{:?}", &e);
                        return e;
                    })?;
                // NOTE: alternatively, the following could have been used but in that case, we wouldn't handled the error
                // let headers = headers.try_into().unwrap();
                request.headers(headers)
            }
            None => request,
        };

        tracing::debug!("\nmutating request for specific HTTP verb");
        let request = mutator_fn(request);
        // ─────────────────────────────────────────────────────────────────────────────
        // NOTE: it is commented as i don't need to check the schema
        // SECURITY: uncommenting is going to leak the authorization header
        //
        // tracing::trace!("\nMaking request {:?}", request);
        // ─────────────────────────────────────────────────────────────────────────────
        tracing::debug!("\nsending HTTP Request");
        let response =
            request
                .send()
                .await
                .map_err(|e: reqwest_middleware::Error| {
                    let e = super::Error::Response {
                        canonical_reason: e.to_string(),
                        status: None,
                        body: None,
                    };
                    // tracing::error!("\n{:?}", &e);
                    return e;
                })?;
        tracing::debug!("\nconverting server response to JSON");
        let status = response.status();
        // ─────────────────────────────────────────────────────────────────────────────
        // NOTE: using `response.json::<serde_json::Value>()` method to convert the response to JSON overrides
        // "content-type" to "application/json" which is undesirable thus we are
        // not using that method.
        // let response: serde_json::Value = response
        //     .json::<serde_json::Value>()
        //     .await
        //     .map_err(|e: reqwest::Error| {
        //         let e = super::Error::Response {
        //             canonical_reason: e.to_string(),
        //             status: None,
        //             body: None,
        //         };
        //         // tracing::error!("\n{:?}", &e);
        //         return e;
        //     })?;
        // ─────────────────────────────────────────────────────────────────────────────
        let response =
            response.text().await.map_err(|e: reqwest::Error| {
                let e = super::Error::Response {
                    canonical_reason: e.to_string(),
                    status: None,
                    body: None,
                };
                return e;
            })?;
        let response: serde_json::Value = serde_json::from_str(
            response.as_str(),
        )
        .map_err(|e: serde_json::Error| {
            let e = super::Error::Response {
                canonical_reason: format!(
                    "corrupted response JSON payload received. {}",
                    e.to_string()
                ),
                status: Some(
                    reqwest::StatusCode::BAD_REQUEST.as_str().to_string(),
                ),
                body: None,
            };
            tracing::error!("\nraw faulty response:\n{:?}", response);
            return e;
        })?;

        tracing::trace!(
            "Response Raw Data:\n{}",
            serde_json::to_string_pretty(&response).unwrap()
        );
        if !status.is_success() {
            let e = super::Error::Response {
                canonical_reason: status
                    .canonical_reason()
                    .unwrap_or("server returned an error response")
                    .to_string(),
                status: Some(status.as_str().to_string()),
                body: Some(response),
            };
            // tracing::error!("\n{:?}", &e);
            return Err(e);
        }
        Ok(response)
    }
}

#[async_impl]
impl super::HttpClient for Client {
    #[inline]
    async fn get<R, S>(
        &self,
        url: S,
        headers: Option<&super::Headers>,
        payload: Option<&super::Query>,
    ) -> Result<R, super::Error>
    where
        R: DeserializeOwned + Serialize + Debug,
        S: AsRef<str> + Sync + Send,
    {
        let span: tracing::Span = tracing::span!(tracing::Level::INFO, "get");
        let _guard = span.enter();

        let response = self
            .exec::<_, serde_json::Value>(
                Method::GET,
                url.as_ref(),
                headers,
                |req| {
                    if let Some(payload) = payload {
                        return req.query(payload);
                    }
                    req
                },
            )
            .await
            .map_err(|e| {
                // tracing::error!("\nServer Response Error:\n{:?}", &e);
                e
            })?;
        let result = serde_json::from_value(response).map_err(
            |e: serde_json::Error| {
                let mut canonical_reason = e.to_string();
                let e = super::Error::Response {
                    canonical_reason: canonical_reason,
                    status: None,
                    body: None,
                };
                // tracing::error!("\nJSON Conversion Error:\n{:?}", &e);
                return e;
            },
        )?;
        Ok(result)
    }

    #[inline]
    async fn post<R, S, T>(
        &self,
        url: S,
        headers: Option<&super::Headers>,
        payload: T,
    ) -> Result<R, super::Error>
    where
        R: DeserializeOwned + Serialize + Debug,
        S: AsRef<str> + Sync + Send,
        T: Serialize + Debug + Send + Sync,
    {
        // ─────────────────────────────────────────────────────────────────────────────
        // NOTE: this was used when `payload` was of Option type
        // self.exec(Method::POST, url.as_ref(), headers, |req| {
        //     // NOTE: checking to see if a payload was provided or not
        //     match &payload {
        //         Some(payload) => req.json(payload),
        //         None => req,
        //     }
        // })
        // .await
        // ─────────────────────────────────────────────────────────────────────────────
        let response = self
            .exec::<_, serde_json::Value>(
                Method::POST,
                url.as_ref(),
                headers,
                |req| req.json(&payload),
            )
            .await?;

        let result = serde_json::from_value(response).map_err(
            |e: serde_json::Error| {
                let mut canonical_reason = e.to_string();
                let e = super::Error::Response {
                    canonical_reason: canonical_reason,
                    status: None,
                    body: None,
                };
                // tracing::error!("\nJSON Conversion Error:\n{:?}", &e);
                return e;
            },
        )?;
        Ok(result)
    }
    #[inline]
    async fn put<R, S, T>(
        &self,
        url: S,
        headers: Option<&super::Headers>,
        payload: T,
    ) -> Result<R, super::Error>
    where
        R: DeserializeOwned + Serialize + Debug,
        S: AsRef<str> + Sync + Send,
        T: Serialize + Debug + Send + Sync,
    {
        let response = self
            .exec::<_, serde_json::Value>(
                Method::PUT,
                url.as_ref(),
                headers,
                |req| req.json(&payload),
            )
            .await?;
        // FIXME: cannot be infallible

        Ok(serde_json::from_value(response).expect("infallible"))
    }

    #[inline]
    async fn patch<R, S, T>(
        &self,
        url: S,
        headers: Option<&super::Headers>,
        payload: T,
    ) -> Result<R, super::Error>
    where
        R: DeserializeOwned + Serialize + Debug,
        S: AsRef<str> + Sync + Send,
        T: Serialize + Debug + Send + Sync,
    {
        let span: tracing::Span =
            tracing::span!(tracing::Level::INFO, "patch");
        let _guard = span.enter();
        tracing::trace!(
            "\nRequest Payload Data:\n{}",
            serde_json::to_string_pretty(&payload).unwrap()
        );
        let response = self
            .exec::<_, serde_json::Value>(
                Method::PATCH,
                url.as_ref(),
                headers,
                |req| {
                    // NOTE: using `response.json::<serde_json::Value>()` method to convert the response to JSON overrides
                    // "content-type" to "application/json" which is undesirable thus we are
                    // not using that method.
                    req.body(reqwest::Body::from(
                        // XXX: why does this work ?
                        serde_json::to_vec(&payload).unwrap(),
                    ))
                },
            )
            .await?;
        let response: R = serde_json::from_value(response).map_err(
            |e: serde_json::Error| {
                // let e = e.to_string().as_str().trim_matches("1").to_string();
                let e: String = e.to_string().replace("\\", "");

                let e = super::Error::Response {
                    canonical_reason: e,
                    status: Some(
                        reqwest::StatusCode::BAD_REQUEST.as_str().to_string(),
                    ),
                    body: None,
                };
                return e;
            },
        )?;
        Ok(response)
    }

    #[inline]
    async fn delete<R, S, T>(
        &self,
        url: S,
        headers: Option<&super::Headers>,
        payload: T,
    ) -> Result<R, super::Error>
    where
        R: DeserializeOwned + Serialize + Debug,
        S: AsRef<str> + Sync + Send,
        T: Serialize + Debug + Send + Sync,
    {
        let response = self
            .exec::<_, serde_json::Value>(
                Method::DELETE,
                url.as_ref(),
                headers,
                |req| req.json(&payload),
            )
            .await?;
        // FIXME: cannot be infallible
        Ok(serde_json::from_value(response).expect("infallible"))
    }
}
// ────────────────────────────────────────────────────────────
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Builder {
    /// request timeout in milliseconds. The timeout is applied from when the request starts connecting until the response body has finished
    timeout: u64,
    /// Minimum waiting time between two retry attempts in **milliseconds**.
    min_retry_interval: u64,
    /// Maximum waiting time between two retry attempt in **milliseconds**.
    max_retry_interval: u64,
    /// Maximum number of allowed retries attempts.
    max_retries: u32,
}
// ────────────────────────────────────────────────────────────
impl Builder {
    /// creates a new API client builder
    pub fn new() -> Self {
        Builder::default()
    }
    /// sets client timeout
    pub fn set_timeout(
        self,
        arg: u64,
    ) -> miette::Result<Self, Box<dyn std::error::Error>> {
        let mut res = self.clone();
        res.timeout = arg;
        Ok(res)
    }
    /// return client timeout
    pub fn get_timeout(&self) -> u64 {
        self.timeout
    }
    /// sets client min_retry_interval
    pub fn set_min_retry_interval(
        self,
        arg: u64,
    ) -> miette::Result<Self, Box<dyn std::error::Error>> {
        let mut res = self.clone();
        res.min_retry_interval = arg;
        Ok(res)
    }
    /// return client min_retry_interval
    pub fn get_min_retry_interval(&self) -> u64 {
        self.min_retry_interval
    }
    /// sets client max_retry_interval
    pub fn set_max_retry_interval(
        self,
        arg: u64,
    ) -> miette::Result<Self, Box<dyn std::error::Error>> {
        let mut res = self.clone();
        res.max_retry_interval = arg;
        Ok(res)
    }
    /// return client max_retry_interval
    pub fn get_max_retry_interval(&self) -> u64 {
        self.max_retry_interval
    }
    /// sets client max_retries
    pub fn set_max_retries(
        self,
        arg: u32,
    ) -> miette::Result<Self, Box<dyn std::error::Error>> {
        let mut res = self.clone();
        res.max_retries = arg;
        Ok(res)
    }
    /// return client max_retries
    pub fn get_max_retries(&self) -> u32 {
        self.max_retries
    }
    /// builds and returns upstream server client that supports request retries with exponential backoff that uses an exponent base of 2.
    pub fn build(
        self,
    ) -> miette::Result<super::ReqwestClient, Box<dyn std::error::Error>> {
        let mut headers = HeaderMap::new();
        // headers.insert(
        //     USER_AGENT,
        //     HeaderValue::from_static(env!("CARGO_PKG_NAME")),
        // );
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/vnd.api+json"),
        );
        let client: reqwest::Client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_millis(self.timeout))
            .build()?;
        let retry_policy = ExponentialBackoff::builder()
            .base(2)
            .retry_bounds(
                Duration::from_millis(self.min_retry_interval),
                Duration::from_millis(self.max_retry_interval),
            )
            .build_with_max_retries(self.max_retries);
        let client = ClientBuilder::new(client)
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();
        Ok(Client::new(client))
    }
}
// ────────────────────────────────────────────────────────────
impl Default for Builder {
    fn default() -> Self {
        Builder {
            timeout: DEFAULT_TIMEOUT,
            min_retry_interval: DEFAULT_MIN_RETRY_INTERVAL,
            max_retry_interval: DEFAULT_MAX_RETRY_INTERVAL,
            max_retries: DEFAULT_MAX_RETRIES,
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::core::HttpClient;

    // cargo test --all-targets -- "core::reqwest::tests" --nocapture
    // cargo watch -cx 'test --all-targets -- "core::reqwest::tests" --nocapture'
    use super::*;
    // write test cases for all setter methods
    #[test]
    fn set_timeout() {
        let builder = Builder::new();
        let timeout = 50;
        let builder = builder.set_timeout(timeout);
        assert_eq!(builder.unwrap().get_timeout(), timeout);
    }
    #[test]
    fn set_min_retry_interval() {
        let builder = Builder::new();
        let min_retry_interval = 50;
        let builder = builder.set_min_retry_interval(min_retry_interval);
        assert_eq!(
            builder.unwrap().get_min_retry_interval(),
            min_retry_interval
        );
    }
    #[test]
    fn set_max_retry_interval() {
        let builder = Builder::new();
        let max_retry_interval = 50;
        let builder = builder.set_max_retry_interval(max_retry_interval);
        assert_eq!(
            builder.unwrap().get_max_retry_interval(),
            max_retry_interval
        );
    }
    #[test]
    fn set_max_retries() {
        let builder = Builder::new();
        let max_retries = 50;
        let builder = builder.set_max_retries(max_retries);
        assert_eq!(builder.unwrap().get_max_retries(), max_retries);
    }
    #[tokio::test]
    async fn build() {
        let builder = Builder::new();
        let client = builder.build();
        assert!(client.is_ok());
    }
    /// passing test to an endpoint that does not need authentication
    #[tokio::test]
    async fn pass_api_call() {
        tracing();
        let url = "https://app.terraform.io/api/meta/ip-ranges";
        let builder = Builder::new();
        let client: Result<
            crate::core::ReqwestClient,
            Box<dyn std::error::Error>,
        > = builder.build();
        assert!(client.is_ok());
        let mut client: crate::core::ReqwestClient = client.unwrap();
        let result: Result<serde_json::Value, crate::core::Error> =
            client.get::<serde_json::Value, &str>(url, None, None).await;
        assert!(result.is_ok());
    }
    /// failing test to ensure it handles errors correctly
    #[tokio::test]
    async fn fail_api_call() {
        tracing();
        let url = "https://app.terraform.io/api/v2/account/details";
        let builder = Builder::new();
        let client = builder.build();
        assert!(client.is_ok());
        let mut client = client.unwrap();

        let result: Result<serde_json::Value, crate::core::Error> =
            client.get::<serde_json::Value, &str>(url, None, None).await;
        assert!(result.is_err());
        let actual = result.unwrap_err();
        let expected: crate::core::Error = crate::core::Error::Response {
            canonical_reason: "Unauthorized".to_string(),
            status: Some("401".to_string()),
            body: Some(serde_json::json!({
                "errors": [{
                "status": "401",
                "title":"unauthorized" ,
                }]
            })),
        };
        assert_eq!(actual, expected);
    }
    /// enables tracing in tests. used for debugging
    fn tracing() {
        // NOTE: set tracing level with env vars; e.g
        // RUST_LOG="tfcctl=trace" cargo run
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .try_init();
        // NOTE: making sure we are using terraform cloud
        std::env::remove_var("TFE_HOSTNAME");
    }
}
// ────────────────────────────────────────────────────────────
// vim: filetype=rust syntax=rust softtabstop=4 tabstop=4 shiftwidth=4 textwidth=79 fileencoding=utf-8 expandtab
// code: language=rust insertSpaces=true tabSize=4
