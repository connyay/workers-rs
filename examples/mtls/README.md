# mTLS Certificate Binding Example

This example demonstrates how to use mTLS (mutual TLS) certificate bindings in Cloudflare Workers to make authenticated requests to services that require client certificates.

## What is mTLS?

When using HTTPS, a server presents a certificate for the client to authenticate in order to prove their identity. For even tighter security, some services require that the client also present a certificate. This bidirectional authentication is called mutual TLS (mTLS).

mTLS certificate bindings allow Workers to present client certificates when communicating with services that require mutual authentication. This moves security from application code to the TLS protocol level, where unauthorized clients are rejected during the handshake.

## Setup

### 1. Upload your certificate

First, upload your certificate and private key using wrangler:

```bash
wrangler mtls-certificate upload --cert cert.pem --key key.pem
```

This will return a certificate ID that you'll use in the next step.

### 2. Configure the binding

Add the mTLS certificate binding to your `wrangler.toml`:

```toml
mtls_certificates = [
  { binding = "MY_CERT", certificate_id = "<CERTIFICATE_ID>" }
]
```

Replace `<CERTIFICATE_ID>` with the ID returned from the upload command.

### 3. Use the binding in your code

```rust
use worker::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    // Get the mTLS certificate binding
    let cert = env.get_binding::<MtlsCertificate>("MY_CERT")?;

    // Make an authenticated request
    // The certificate is presented automatically during TLS handshake
    let response = cert.fetch("https://api.example.com/endpoint", None).await?;

    Ok(response)
}
```

## Important Limitations

⚠️ **Cannot be used with Cloudflare-proxied zones**: mTLS certificate bindings cannot be used to make requests to services that are proxied by Cloudflare. Attempting to do so will result in a 520 error.

## Running this example

1. Install dependencies:
```bash
npm install
```

2. Build the Worker:
```bash
npm run build
```

3. Deploy:
```bash
wrangler deploy
```

## Learn more

- [mTLS bindings documentation](https://developers.cloudflare.com/workers/runtime-apis/bindings/mtls/)
- [Mutual TLS announcement blog post](https://blog.cloudflare.com/mtls-workers/)
