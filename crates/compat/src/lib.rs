//! Compile-time compatibility checks for public cross-package types.

#[cfg(test)]
use futures::future::BoxFuture;
#[cfg(test)]
use gpui::http_client::{AsyncBody, Request, Response, http::HeaderValue};
#[cfg(test)]
use url::Url;

#[cfg(test)]
struct ConsumerHttpClient;

#[cfg(test)]
impl http_client::HttpClient for ConsumerHttpClient {
    fn user_agent(&self) -> Option<&HeaderValue> {
        None
    }

    fn proxy(&self) -> Option<&Url> {
        None
    }

    fn send(
        &self,
        _request: Request<AsyncBody>,
    ) -> BoxFuture<'static, anyhow::Result<Response<AsyncBody>>> {
        Box::pin(async {
            Err(anyhow::anyhow!(
                "compatibility client does not send requests"
            ))
        })
    }
}

#[cfg(test)]
fn accepts_gpui_http_client(_client: &dyn gpui::http_client::HttpClient) {}

#[allow(dead_code)]
fn platform_and_tokio_entry_points_compile() {
    let _application = gpui_platform::application();
    let _tokio_init: fn(&mut gpui::App) = gpui_tokio::init;
    let _headless = gpui::TestAppContext::build;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_http_client_type_identity_is_preserved() {
        accepts_gpui_http_client(&ConsumerHttpClient);
    }

    #[test]
    fn accesskit_reexports_preserve_type_identity() {
        let role: gpui::accesskit::Role = gpui::Role::Button;
        assert_eq!(role, gpui::accesskit::Role::Button);
    }
}
