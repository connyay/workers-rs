use crate::{env::EnvBinding, RequestInit, Result};
use std::convert::TryInto;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;

#[cfg(feature = "http")]
use crate::HttpResponse;
use crate::{Request, Response};

/// A binding for mTLS (mutual TLS) certificate authentication.
///
/// mTLS certificate bindings allow Workers to present client certificates when communicating
/// with services that require mutual authentication. The certificate is presented automatically
/// during TLS handshakes when using the `fetch` methods.
///
/// # Configuration
///
/// To use mTLS certificates, you must first upload your certificate and private key using:
/// ```bash
/// wrangler mtls-certificate upload --cert <CERT_PATH> --key <KEY_PATH>
/// ```
///
/// Then configure the binding in your `wrangler.toml`:
/// ```toml
/// mtls_certificates = [
///   { binding = "MY_CERT", certificate_id = "<CERTIFICATE_ID>" }
/// ]
/// ```
///
/// # Usage
///
/// ```no_run
/// # use worker::*;
/// # async fn example(env: Env) -> Result<Response> {
/// // Get the mTLS certificate binding
/// let cert = env.get_binding::<MtlsCertificate>("MY_CERT")?;
///
/// // Make an authenticated request
/// let response = cert.fetch("https://api.example.com/endpoint", None).await?;
/// # Ok(response.into())
/// # }
/// ```
///
/// # Important Limitations
///
/// ⚠️ **Cannot be used with Cloudflare-proxied zones**: mTLS certificate bindings cannot be used
/// to make requests to services that are proxied by Cloudflare. Attempting to do so will result
/// in a 520 error.
///
/// For more information, see: <https://developers.cloudflare.com/workers/runtime-apis/bindings/mtls/>
#[derive(Debug, Clone)]
pub struct MtlsCertificate(worker_sys::MtlsCertificate);

#[cfg(not(feature = "http"))]
type FetchResponseType = Response;
#[cfg(feature = "http")]
type FetchResponseType = HttpResponse;

impl MtlsCertificate {
    /// Make an authenticated request using the mTLS certificate.
    ///
    /// The client certificate will be presented automatically during the TLS handshake.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to request
    /// * `init` - Optional request configuration
    ///
    /// # Returns
    ///
    /// Returns [`Response`](crate::Response) unless `http` feature is enabled,
    /// in which case it returns [`http::Response<worker::Body>`].
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use worker::*;
    /// # async fn example(cert: MtlsCertificate) -> Result<()> {
    /// // Simple GET request
    /// let response = cert.fetch("https://api.example.com/data", None).await?;
    ///
    /// // POST request with custom options
    /// let mut init = RequestInit::new();
    /// init.with_method(Method::Post);
    /// let response = cert.fetch("https://api.example.com/submit", Some(init)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn fetch(
        &self,
        url: impl Into<String>,
        init: Option<RequestInit>,
    ) -> Result<FetchResponseType> {
        let path = url.into();
        let promise = match init {
            Some(ref init) => self.0.fetch_with_str_and_init(&path, &init.into()),
            None => self.0.fetch_with_str(&path),
        }?;

        let resp_sys: web_sys::Response = JsFuture::from(promise).await?.dyn_into()?;
        #[cfg(not(feature = "http"))]
        let result = Ok(Response::from(resp_sys));
        #[cfg(feature = "http")]
        let result = crate::response_from_wasm(resp_sys);
        result
    }

    /// Make an authenticated request using an existing [Request].
    ///
    /// The client certificate will be presented automatically during the TLS handshake.
    ///
    /// # Arguments
    ///
    /// Argument type is [`Request`](crate::Request) or [`http::Request<worker::Body>`].
    ///
    /// # Returns
    ///
    /// Returns [`Response`](crate::Response) unless `http` feature is enabled,
    /// in which case it returns [`http::Response<worker::Body>`].
    pub async fn fetch_request<T, E>(&self, request: T) -> Result<FetchResponseType>
    where
        T: TryInto<Request, Error = E>,
        crate::Error: From<E>,
    {
        let req = request.try_into()?;
        let promise = self.0.fetch(req.inner())?;
        let resp_sys: web_sys::Response = JsFuture::from(promise).await?.dyn_into()?;
        let response = Response::from(resp_sys);
        #[cfg(feature = "http")]
        let result = response.try_into();
        #[cfg(not(feature = "http"))]
        let result = Ok(response);
        result
    }
}

impl EnvBinding for MtlsCertificate {
    const TYPE_NAME: &'static str = "Fetcher";
}

impl JsCast for MtlsCertificate {
    fn instanceof(val: &wasm_bindgen::JsValue) -> bool {
        val.is_instance_of::<worker_sys::MtlsCertificate>()
    }

    fn unchecked_from_js(val: wasm_bindgen::JsValue) -> Self {
        Self(val.into())
    }

    fn unchecked_from_js_ref(val: &wasm_bindgen::JsValue) -> &Self {
        unsafe { &*(val as *const JsValue as *const Self) }
    }
}

impl From<MtlsCertificate> for JsValue {
    fn from(cert: MtlsCertificate) -> Self {
        JsValue::from(cert.0)
    }
}

impl AsRef<wasm_bindgen::JsValue> for MtlsCertificate {
    fn as_ref(&self) -> &wasm_bindgen::JsValue {
        &self.0
    }
}

impl From<worker_sys::MtlsCertificate> for MtlsCertificate {
    fn from(inner: worker_sys::MtlsCertificate) -> Self {
        Self(inner)
    }
}

unsafe impl Send for MtlsCertificate {}
unsafe impl Sync for MtlsCertificate {}
