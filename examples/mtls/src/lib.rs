use worker::*;

/// Example demonstrating the use of mTLS certificate bindings in Workers.
///
/// mTLS (mutual TLS) certificate bindings allow Workers to present client certificates
/// when communicating with services that require mutual authentication.
///
/// # Setup
///
/// 1. Upload your certificate and private key:
///    ```bash
///    wrangler mtls-certificate upload --cert cert.pem --key key.pem
///    ```
///
/// 2. Configure the binding in wrangler.toml:
///    ```toml
///    mtls_certificates = [
///      { binding = "MY_CERT", certificate_id = "<CERTIFICATE_ID>" }
///    ]
///    ```
///
/// 3. Use the binding in your Worker to make authenticated requests.
#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_log!("Handling request: {}", req.path());

    // Get the mTLS certificate binding from the environment
    let cert = match env.get_binding::<MtlsCertificate>("MY_CERT") {
        Ok(cert) => cert,
        Err(e) => {
            return Response::error(
                format!(
                    "Failed to get mTLS certificate binding: {}. \
                     Make sure 'MY_CERT' is configured in wrangler.toml",
                    e
                ),
                500,
            )
        }
    };

    // Make an authenticated request using the mTLS certificate
    // The certificate will be presented automatically during the TLS handshake
    let upstream_url = "https://api.example.com/endpoint";

    console_log!("Making authenticated request to: {}", upstream_url);

    match cert.fetch(upstream_url, None).await {
        Ok(response) => {
            console_log!("Received response with status: {}", response.status_code());
            Ok(response)
        }
        Err(e) => Response::error(format!("Request failed: {}", e), 502),
    }
}
