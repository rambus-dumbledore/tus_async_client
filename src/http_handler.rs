use crate::http::{HttpHandler, HttpMethod, HttpRequest, HttpResponse};
use crate::Error;
use reqwest::header::{HeaderMap, HeaderName};
use reqwest::Method;
use std::collections::HashMap;
use std::str::FromStr;
use std::rc::Rc;

impl HttpHandler {
    pub fn new(client: Rc<reqwest::Client>) -> Self {
        Self(client)
    }

    pub(crate) async fn handle_request(&self, req: HttpRequest<'_>) -> Result<HttpResponse, Error> {
        let mut headers = HeaderMap::new();
        for (key, value) in req.headers {
            headers.insert(HeaderName::from_str(&key).unwrap(), value.parse().unwrap());
        }

        let mut builder = match req.method {
            HttpMethod::Head => self.0.head(&req.url),
            HttpMethod::Patch => self.0.patch(&req.url),
            HttpMethod::Options => self.0.request(Method::OPTIONS, &req.url),
            HttpMethod::Post => self.0.post(&req.url),
            HttpMethod::Delete => self.0.delete(&req.url),
        }
        .headers(headers);

        if let Some(body) = req.body {
            builder = builder.body(Vec::from(body));
        }

        let response = match builder.send().await {
            Ok(resp) => resp,
            Err(err) => return Err(Error::HttpHandlerError(err.to_string())),
        };

        let mut headers = HashMap::new();
        for (key, value) in response.headers() {
            headers.insert(
                key.to_string(),
                value.to_str().map(String::from).unwrap_or_default(),
            );
        }

        Ok(HttpResponse {
            status_code: response.status().as_u16() as usize,
            headers,
        })
    }
}
