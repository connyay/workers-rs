use wasm_bindgen::JsCast;

use crate::{EnvBinding, Result};

// The full email surface comes from the auto-generated `crate::email`
// module (`chomp build:types` from `types/email.d.ts`). This file only
// adds the [`EnvBinding`] trait impl on top of the auto-gen `SendEmail`
// extern type so `Env::send_email("EMAIL")` resolves cleanly.
//
// `EmailMessage` is the constructor class imported from
// `cloudflare:email`, used to build raw-MIME messages and pass to
// `SendEmail.send(message)`. `StructuredEmailMessage` is the global
// envelope-getter interface exposed on `ForwardableEmailMessage` and
// `reply`. Same JS object, two distinct Rust types — see the d.ts EDIT
// note for the naming rationale.
pub use crate::email::email::EmailMessage;
pub use crate::email::{
    EmailAddress, EmailAttachment, EmailSendResult, SendEmail, SendEmailBuilder,
    StructuredEmailMessage,
};

impl EnvBinding for SendEmail {
    const TYPE_NAME: &'static str = "SendEmail";

    // `SendEmail` is a TypeScript interface, not a class — the runtime
    // doesn't expose a `SendEmail` global for the default
    // `constructor.name` check to match against. The TS types are
    // authoritative: if `env.EMAIL` is bound to a SendEmail per
    // `wrangler.toml`, the runtime hands us the right shape, so we
    // skip the check and `unchecked_into`.
    fn get(val: wasm_bindgen::JsValue) -> Result<Self> {
        Ok(val.unchecked_into())
    }
}
