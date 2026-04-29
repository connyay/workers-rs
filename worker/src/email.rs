#[allow(dead_code)]
use crate::Context as ExecutionContext;
#[allow(dead_code)]
use crate::Env;
#[allow(dead_code)]
use ::web_sys::Event;
#[allow(dead_code)]
use ::web_sys::Headers;
#[allow(dead_code)]
use ::web_sys::ReadableStream;
#[allow(unused_imports)]
use js_sys::*;
#[allow(unused_imports)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Event , extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type ExtendableEvent;
    #[doc = " The **`ExtendableEvent.waitUntil()`** method tells the event dispatcher that work is ongoing."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/ExtendableEvent/waitUntil)"]
    #[wasm_bindgen(method, js_name = "waitUntil")]
    pub fn wait_until(this: &ExtendableEvent, promise: &Promise);
    #[doc = " The **`ExtendableEvent.waitUntil()`** method tells the event dispatcher that work is ongoing."]
    #[doc = " "]
    #[doc = " [MDN Reference](https://developer.mozilla.org/docs/Web/API/ExtendableEvent/waitUntil)"]
    #[wasm_bindgen(method, catch, js_name = "waitUntil")]
    pub fn try_wait_until(this: &ExtendableEvent, promise: &Promise) -> Result<(), JsValue>;
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type EmailSendResult;
    #[doc = " The Email Message ID"]
    #[wasm_bindgen(method, getter, js_name = "messageId")]
    pub fn message_id(this: &EmailSendResult) -> String;
    #[wasm_bindgen(method, setter, js_name = "messageId")]
    pub fn set_message_id(this: &EmailSendResult, val: &str);
}
impl EmailSendResult {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        #[allow(unused_imports)]
        use wasm_bindgen::JsCast;
        JsCast::unchecked_into(js_sys::Object::new())
    }
    pub fn builder() -> EmailSendResultBuilder {
        EmailSendResultBuilder {
            inner: Self::new(),
            required: 1u64,
        }
    }
}
pub struct EmailSendResultBuilder {
    inner: EmailSendResult,
    required: u64,
}
#[allow(unused_mut)]
impl EmailSendResultBuilder {
    pub fn message_id(mut self, val: &str) -> Self {
        self.inner.set_message_id(val);
        self.required &= 18446744073709551614u64;
        self
    }
    pub fn build(self) -> Result<EmailSendResult, JsValue> {
        if self.required != 0 {
            let mut missing = Vec::new();
            if self.required & 1u64 != 0 {
                missing.push("missing required property `messageId`");
            }
            return Err(JsValue::from_str(&format!(
                "{}: {}",
                stringify!(EmailSendResult),
                missing.join(", ")
            )));
        }
        Ok(self.inner)
    }
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type EmailMessage;
    #[doc = " Envelope From attribute of the email message."]
    #[wasm_bindgen(method, getter)]
    pub fn from(this: &EmailMessage) -> String;
    #[doc = " Envelope To attribute of the email message."]
    #[wasm_bindgen(method, getter)]
    pub fn to(this: &EmailMessage) -> String;
}
impl EmailMessage {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        #[allow(unused_unsafe)]
        unsafe {
            JsValue::from(js_sys::Object::new()).unchecked_into()
        }
    }
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = EmailMessage , extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type ForwardableEmailMessage;
    #[doc = " Stream of the email message content."]
    #[wasm_bindgen(method, getter)]
    pub fn raw(this: &ForwardableEmailMessage) -> ReadableStream;
    #[doc = " An [Headers object](https://developer.mozilla.org/en-US/docs/Web/API/Headers)."]
    #[wasm_bindgen(method, getter)]
    pub fn headers(this: &ForwardableEmailMessage) -> Headers;
    #[doc = " Size of the email message content."]
    #[wasm_bindgen(method, getter, js_name = "rawSize")]
    pub fn raw_size(this: &ForwardableEmailMessage) -> f64;
    #[doc = " Reject this email message by returning a permanent SMTP error back to the connecting client including the given reason."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `reason` - The reject reason."]
    #[doc = " "]
    #[doc = " ## Returns"]
    #[doc = " "]
    #[doc = " void"]
    #[wasm_bindgen(method, js_name = "setReject")]
    pub fn set_reject(this: &ForwardableEmailMessage, reason: &str);
    #[doc = " Reject this email message by returning a permanent SMTP error back to the connecting client including the given reason."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `reason` - The reject reason."]
    #[doc = " "]
    #[doc = " ## Returns"]
    #[doc = " "]
    #[doc = " void"]
    #[wasm_bindgen(method, catch, js_name = "setReject")]
    pub fn try_set_reject(this: &ForwardableEmailMessage, reason: &str) -> Result<(), JsValue>;
    #[doc = " Forward this email message to a verified destination address of the account."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `rcptTo` - Verified destination address."]
    #[doc = " * `headers` - A [Headers object](https://developer.mozilla.org/en-US/docs/Web/API/Headers)."]
    #[doc = " "]
    #[doc = " ## Returns"]
    #[doc = " "]
    #[doc = " A promise that resolves when the email message is forwarded."]
    #[wasm_bindgen(method, catch)]
    pub async fn forward(
        this: &ForwardableEmailMessage,
        rcpt_to: &str,
    ) -> Result<EmailSendResult, JsValue>;
    #[doc = " Forward this email message to a verified destination address of the account."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `rcptTo` - Verified destination address."]
    #[doc = " * `headers` - A [Headers object](https://developer.mozilla.org/en-US/docs/Web/API/Headers)."]
    #[doc = " "]
    #[doc = " ## Returns"]
    #[doc = " "]
    #[doc = " A promise that resolves when the email message is forwarded."]
    #[wasm_bindgen(method, catch, js_name = "forward")]
    pub async fn forward_with_headers(
        this: &ForwardableEmailMessage,
        rcpt_to: &str,
        headers: &Headers,
    ) -> Result<EmailSendResult, JsValue>;
    #[doc = " Reply to the sender of this email message with a new EmailMessage object."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `message` - The reply message."]
    #[doc = " "]
    #[doc = " ## Returns"]
    #[doc = " "]
    #[doc = " A promise that resolves when the email message is replied."]
    #[wasm_bindgen(method, catch)]
    pub async fn reply(
        this: &ForwardableEmailMessage,
        message: &EmailMessage,
    ) -> Result<EmailSendResult, JsValue>;
}
#[allow(dead_code)]
pub type EmailAttachment = JsValue;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type EmailAddress;
    #[wasm_bindgen(method, getter)]
    pub fn name(this: &EmailAddress) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_name(this: &EmailAddress, val: &str);
    #[wasm_bindgen(method, getter)]
    pub fn email(this: &EmailAddress) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_email(this: &EmailAddress, val: &str);
}
impl EmailAddress {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        #[allow(unused_imports)]
        use wasm_bindgen::JsCast;
        JsCast::unchecked_into(js_sys::Object::new())
    }
    pub fn builder() -> EmailAddressBuilder {
        EmailAddressBuilder {
            inner: Self::new(),
            required: 3u64,
        }
    }
}
pub struct EmailAddressBuilder {
    inner: EmailAddress,
    required: u64,
}
#[allow(unused_mut)]
impl EmailAddressBuilder {
    pub fn name(mut self, val: &str) -> Self {
        self.inner.set_name(val);
        self.required &= 18446744073709551614u64;
        self
    }
    pub fn email(mut self, val: &str) -> Self {
        self.inner.set_email(val);
        self.required &= 18446744073709551613u64;
        self
    }
    pub fn build(self) -> Result<EmailAddress, JsValue> {
        if self.required != 0 {
            let mut missing = Vec::new();
            if self.required & 1u64 != 0 {
                missing.push("missing required property `name`");
            }
            if self.required & 2u64 != 0 {
                missing.push("missing required property `email`");
            }
            return Err(JsValue::from_str(&format!(
                "{}: {}",
                stringify!(EmailAddress),
                missing.join(", ")
            )));
        }
        Ok(self.inner)
    }
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type SendEmail;
    #[wasm_bindgen(method, catch)]
    pub async fn send(this: &SendEmail, message: &EmailMessage)
        -> Result<EmailSendResult, JsValue>;
    #[wasm_bindgen(method, catch, js_name = "send")]
    pub async fn send_with_builder(
        this: &SendEmail,
        builder: &Object,
    ) -> Result<EmailSendResult, JsValue>;
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = ExtendableEvent , extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type EmailEvent;
    #[wasm_bindgen(method, getter)]
    pub fn message(this: &EmailEvent) -> ForwardableEmailMessage;
}
#[allow(dead_code)]
pub type EmailExportedHandler =
    Function<fn(ForwardableEmailMessage, Env, ExecutionContext) -> JsOption<Promise<Undefined>>>;
pub mod email {
    use super::*;
    use js_sys::*;
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen(module = "cloudflare:email")]
    extern "C" {
        # [wasm_bindgen (extends = Object)]
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub type EmailMessage;
        #[wasm_bindgen(constructor, catch)]
        pub fn new(from: &str, to: &str, raw: &str) -> Result<EmailMessage, JsValue>;
        #[wasm_bindgen(constructor, catch, js_name = "EmailMessage")]
        pub fn new_with_readable_stream(
            from: &str,
            to: &str,
            raw: &ReadableStream,
        ) -> Result<EmailMessage, JsValue>;
    }
}
