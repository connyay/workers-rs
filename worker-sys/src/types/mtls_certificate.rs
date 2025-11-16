use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// mTLS certificate binding for presenting client certificates when making requests.
    ///
    /// This type is identical to Fetcher in the JavaScript runtime, but represents a binding
    /// configured with an mTLS certificate that will be presented during TLS handshakes.
    #[wasm_bindgen(extends=js_sys::Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type MtlsCertificate;

    #[wasm_bindgen(method, catch)]
    pub fn fetch(
        this: &MtlsCertificate,
        input: &web_sys::Request,
    ) -> Result<js_sys::Promise, JsValue>;

    #[wasm_bindgen(method, catch, js_name=fetch)]
    pub fn fetch_with_str(this: &MtlsCertificate, input: &str) -> Result<js_sys::Promise, JsValue>;

    #[wasm_bindgen(method, catch, js_name=fetch)]
    pub fn fetch_with_init(
        this: &MtlsCertificate,
        input: &web_sys::Request,
        init: &web_sys::RequestInit,
    ) -> Result<js_sys::Promise, JsValue>;

    #[wasm_bindgen(method, catch, js_name=fetch)]
    pub fn fetch_with_str_and_init(
        this: &MtlsCertificate,
        input: &str,
        init: &web_sys::RequestInit,
    ) -> Result<js_sys::Promise, JsValue>;
}

unsafe impl Send for MtlsCertificate {}
unsafe impl Sync for MtlsCertificate {}
