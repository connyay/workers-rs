#[allow(unused_imports)] use email::EmailMessage;
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
    pub type StructuredEmailMessage;
    #[doc = " Envelope From attribute of the email message."]
    #[wasm_bindgen(method, getter)]
    pub fn from(this: &StructuredEmailMessage) -> String;
    #[doc = " Envelope To attribute of the email message."]
    #[wasm_bindgen(method, getter)]
    pub fn to(this: &StructuredEmailMessage) -> String;
}
impl StructuredEmailMessage {
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
    # [wasm_bindgen (extends = StructuredEmailMessage , extends = Object)]
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
        message: &StructuredEmailMessage,
    ) -> Result<EmailSendResult, JsValue>;
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type EmailAttachment;
    #[wasm_bindgen(method, getter)]
    pub fn content(this: &EmailAttachment) -> JsValue;
    #[wasm_bindgen(method, getter, js_name = "contentId")]
    pub fn content_id(this: &EmailAttachment) -> Option<JsValue>;
    #[wasm_bindgen(method, getter)]
    pub fn disposition(this: &EmailAttachment) -> JsValue;
    #[wasm_bindgen(method, getter)]
    pub fn filename(this: &EmailAttachment) -> String;
    #[wasm_bindgen(method, getter)]
    pub fn r#type(this: &EmailAttachment) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_content(this: &EmailAttachment, val: &str);
    #[wasm_bindgen(method, setter, js_name = "content")]
    pub fn set_content_with_array_buffer(this: &EmailAttachment, val: &ArrayBuffer);
    #[wasm_bindgen(method, setter, js_name = "content")]
    pub fn set_content_with_js_value(this: &EmailAttachment, val: &Object);
    #[wasm_bindgen(method, setter, js_name = "contentId")]
    pub fn set_content_id(this: &EmailAttachment, val: &str);
    #[wasm_bindgen(method, setter, js_name = "contentId")]
    pub fn set_content_id_with_undefined(this: &EmailAttachment, val: &Undefined);
    #[wasm_bindgen(method, setter)]
    pub fn set_disposition(this: &EmailAttachment, val: &str);
    #[wasm_bindgen(method, setter, js_name = "disposition")]
    pub fn set_disposition_with_js_value(this: &EmailAttachment, val: &str);
    #[wasm_bindgen(method, setter)]
    pub fn set_filename(this: &EmailAttachment, val: &str);
    #[wasm_bindgen(method, setter)]
    pub fn set_type(this: &EmailAttachment, val: &str);
}
impl EmailAttachment {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        #[allow(unused_imports)]
        use wasm_bindgen::JsCast;
        JsCast::unchecked_into(js_sys::Object::new())
    }
    pub fn builder() -> EmailAttachmentBuilder {
        EmailAttachmentBuilder {
            inner: Self::new(),
            required: 15u64,
        }
    }
}
pub struct EmailAttachmentBuilder {
    inner: EmailAttachment,
    required: u64,
}
#[allow(unused_mut)]
impl EmailAttachmentBuilder {
    pub fn content(mut self, val: &str) -> Self {
        self.inner.set_content(val);
        self.required &= 18446744073709551614u64;
        self
    }
    pub fn content_with_array_buffer(mut self, val: &ArrayBuffer) -> Self {
        self.inner.set_content_with_array_buffer(val);
        self.required &= 18446744073709551614u64;
        self
    }
    pub fn content_with_js_value(mut self, val: &Object) -> Self {
        self.inner.set_content_with_js_value(val);
        self.required &= 18446744073709551614u64;
        self
    }
    pub fn content_id(mut self, val: &str) -> Self {
        self.inner.set_content_id(val);
        self
    }
    pub fn content_id_with_undefined(mut self, val: &Undefined) -> Self {
        self.inner.set_content_id_with_undefined(val);
        self
    }
    pub fn disposition(mut self, val: &str) -> Self {
        self.inner.set_disposition(val);
        self.required &= 18446744073709551613u64;
        self
    }
    pub fn disposition_with_js_value(mut self, val: &str) -> Self {
        self.inner.set_disposition_with_js_value(val);
        self.required &= 18446744073709551613u64;
        self
    }
    pub fn filename(mut self, val: &str) -> Self {
        self.inner.set_filename(val);
        self.required &= 18446744073709551611u64;
        self
    }
    pub fn r#type(mut self, val: &str) -> Self {
        self.inner.set_type(val);
        self.required &= 18446744073709551607u64;
        self
    }
    pub fn build(self) -> Result<EmailAttachment, JsValue> {
        if self.required != 0 {
            let mut missing = Vec::new();
            if self.required & 1u64 != 0 {
                missing.push("missing required property `content`");
            }
            if self.required & 2u64 != 0 {
                missing.push("missing required property `disposition`");
            }
            if self.required & 4u64 != 0 {
                missing.push("missing required property `filename`");
            }
            if self.required & 8u64 != 0 {
                missing.push("missing required property `type`");
            }
            return Err(JsValue::from_str(&format!(
                "{}: {}",
                stringify!(EmailAttachment),
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
        builder: &SendEmailBuilder,
    ) -> Result<EmailSendResult, JsValue>;
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type SendEmailBuilder;
    #[wasm_bindgen(method, getter)]
    pub fn from(this: &SendEmailBuilder) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_from(this: &SendEmailBuilder, val: &str);
    #[wasm_bindgen(method, setter, js_name = "from")]
    pub fn set_from_with_email_address(this: &SendEmailBuilder, val: &EmailAddress);
    #[wasm_bindgen(method, getter)]
    pub fn to(this: &SendEmailBuilder) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_to(this: &SendEmailBuilder, val: &str);
    #[wasm_bindgen(method, setter, js_name = "to")]
    pub fn set_to_with_array(this: &SendEmailBuilder, val: &Array<JsString>);
    #[wasm_bindgen(method, getter)]
    pub fn subject(this: &SendEmailBuilder) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_subject(this: &SendEmailBuilder, val: &str);
    #[wasm_bindgen(method, getter, js_name = "replyTo")]
    pub fn reply_to(this: &SendEmailBuilder) -> Option<JsValue>;
    #[wasm_bindgen(method, setter, js_name = "replyTo")]
    pub fn set_reply_to(this: &SendEmailBuilder, val: &str);
    #[wasm_bindgen(method, setter, js_name = "replyTo")]
    pub fn set_reply_to_with_email_address(this: &SendEmailBuilder, val: &EmailAddress);
    #[wasm_bindgen(method, getter)]
    pub fn cc(this: &SendEmailBuilder) -> Option<JsValue>;
    #[wasm_bindgen(method, setter)]
    pub fn set_cc(this: &SendEmailBuilder, val: &str);
    #[wasm_bindgen(method, setter, js_name = "cc")]
    pub fn set_cc_with_array(this: &SendEmailBuilder, val: &Array<JsString>);
    #[wasm_bindgen(method, getter)]
    pub fn bcc(this: &SendEmailBuilder) -> Option<JsValue>;
    #[wasm_bindgen(method, setter)]
    pub fn set_bcc(this: &SendEmailBuilder, val: &str);
    #[wasm_bindgen(method, setter, js_name = "bcc")]
    pub fn set_bcc_with_array(this: &SendEmailBuilder, val: &Array<JsString>);
    #[wasm_bindgen(method, getter)]
    pub fn headers(this: &SendEmailBuilder) -> Option<Object<JsString>>;
    #[wasm_bindgen(method, setter)]
    pub fn set_headers(this: &SendEmailBuilder, val: &Object<JsString>);
    #[wasm_bindgen(method, getter)]
    pub fn text(this: &SendEmailBuilder) -> Option<String>;
    #[wasm_bindgen(method, setter)]
    pub fn set_text(this: &SendEmailBuilder, val: &str);
    #[wasm_bindgen(method, getter)]
    pub fn html(this: &SendEmailBuilder) -> Option<String>;
    #[wasm_bindgen(method, setter)]
    pub fn set_html(this: &SendEmailBuilder, val: &str);
    #[wasm_bindgen(method, getter)]
    pub fn attachments(this: &SendEmailBuilder) -> Option<Array<EmailAttachment>>;
    #[wasm_bindgen(method, setter)]
    pub fn set_attachments(this: &SendEmailBuilder, val: &Array<EmailAttachment>);
}
impl SendEmailBuilder {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        #[allow(unused_imports)]
        use wasm_bindgen::JsCast;
        JsCast::unchecked_into(js_sys::Object::new())
    }
    pub fn builder() -> SendEmailBuilderBuilder {
        SendEmailBuilderBuilder {
            inner: Self::new(),
            required: 7u64,
        }
    }
}
pub struct SendEmailBuilderBuilder {
    inner: SendEmailBuilder,
    required: u64,
}
#[allow(unused_mut)]
impl SendEmailBuilderBuilder {
    pub fn from(mut self, val: &str) -> Self {
        self.inner.set_from(val);
        self.required &= 18446744073709551614u64;
        self
    }
    pub fn from_with_email_address(mut self, val: &EmailAddress) -> Self {
        self.inner.set_from_with_email_address(val);
        self.required &= 18446744073709551614u64;
        self
    }
    pub fn to(mut self, val: &str) -> Self {
        self.inner.set_to(val);
        self.required &= 18446744073709551613u64;
        self
    }
    pub fn to_with_array(mut self, val: &Array<JsString>) -> Self {
        self.inner.set_to_with_array(val);
        self.required &= 18446744073709551613u64;
        self
    }
    pub fn subject(mut self, val: &str) -> Self {
        self.inner.set_subject(val);
        self.required &= 18446744073709551611u64;
        self
    }
    pub fn reply_to(mut self, val: &str) -> Self {
        self.inner.set_reply_to(val);
        self
    }
    pub fn reply_to_with_email_address(mut self, val: &EmailAddress) -> Self {
        self.inner.set_reply_to_with_email_address(val);
        self
    }
    pub fn cc(mut self, val: &str) -> Self {
        self.inner.set_cc(val);
        self
    }
    pub fn cc_with_array(mut self, val: &Array<JsString>) -> Self {
        self.inner.set_cc_with_array(val);
        self
    }
    pub fn bcc(mut self, val: &str) -> Self {
        self.inner.set_bcc(val);
        self
    }
    pub fn bcc_with_array(mut self, val: &Array<JsString>) -> Self {
        self.inner.set_bcc_with_array(val);
        self
    }
    pub fn headers(mut self, val: &Object<JsString>) -> Self {
        self.inner.set_headers(val);
        self
    }
    pub fn text(mut self, val: &str) -> Self {
        self.inner.set_text(val);
        self
    }
    pub fn html(mut self, val: &str) -> Self {
        self.inner.set_html(val);
        self
    }
    pub fn attachments(mut self, val: &Array<EmailAttachment>) -> Self {
        self.inner.set_attachments(val);
        self
    }
    pub fn build(self) -> Result<SendEmailBuilder, JsValue> {
        if self.required != 0 {
            let mut missing = Vec::new();
            if self.required & 1u64 != 0 {
                missing.push("missing required property `from`");
            }
            if self.required & 2u64 != 0 {
                missing.push("missing required property `to`");
            }
            if self.required & 4u64 != 0 {
                missing.push("missing required property `subject`");
            }
            return Err(JsValue::from_str(&format!(
                "{}: {}",
                stringify!(SendEmailBuilder),
                missing.join(", ")
            )));
        }
        Ok(self.inner)
    }
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
        #[wasm_bindgen(method, getter)]
        pub fn from(this: &EmailMessage) -> String;
        #[wasm_bindgen(method, getter)]
        pub fn to(this: &EmailMessage) -> String;
    }
}
