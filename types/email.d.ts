/*
 * Email only types from @cloudflare/worker-types. Valid as of 28/04/2026.
 * This file builds src/email.rs as auto-generated bindings using ts-gen.
 * 
 * NOTE: All hand edits to the @cloudflare/worker-types are marked with an "EDIT:" comment.
 */ 

/**
 * The **`ExtendableEvent`** interface extends the lifetime of the `install` and `activate` events dispatched on the global scope as part of the service worker lifecycle.
 *
 * [MDN Reference](https://developer.mozilla.org/docs/Web/API/ExtendableEvent)
 */
declare abstract class ExtendableEvent extends Event {
  /**
   * The **`ExtendableEvent.waitUntil()`** method tells the event dispatcher that work is ongoing.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/ExtendableEvent/waitUntil)
   */
  waitUntil(promise: Promise<any>): void;
}
/**
 * The returned data after sending an email
 */
interface EmailSendResult {
  /**
   * The Email Message ID
   */
  messageId: string;
}
// EDIT: upstream declares the legacy email constructor inside
// `declare module "cloudflare:email" { let _EmailMessage: { new(...) };
// export { _EmailMessage as EmailMessage } }` paired with a global
// `interface EmailMessage` of the same name. ts-gen then emits two
// distinct Rust types, which forces hand-rolled `unchecked_ref` casts at
// every call site.
//
// Until ts-gen learns to fully unify the same-named module export with
// the global interface, we sidestep by giving the *interface* a distinct
// name (`StructuredEmailMessage`) and letting the module-scoped class
// keep the runtime name `EmailMessage`. So:
//
//   * `EmailMessage` — module-scoped class imported from
//     `cloudflare:email`, used to construct raw-MIME messages and pass
//     to `SendEmail.send(message)`.
//   * `StructuredEmailMessage` — global interface, the envelope-getters
//     view exposed on `ForwardableEmailMessage` and elsewhere.
//
// Two names, two types — but the wasm-bindgen import `EmailMessage`
// matches the runtime export, so no shim renaming is required.
declare module "cloudflare:email" {
  class EmailMessage {
    constructor(from: string, to: string, raw: string | ReadableStream);
    readonly from: string;
    readonly to: string;
  }
  export { EmailMessage };
}
import { EmailMessage } from "cloudflare:email";

/**
 * An email message that can be sent from a Worker.
 *
 * EDIT: renamed from upstream's `EmailMessage` to avoid the same-name
 * collision with the `cloudflare:email` constructor class. See the
 * EDIT note on the module declaration below for the rationale.
 */
interface StructuredEmailMessage {
  /**
   * Envelope From attribute of the email message.
   */
  readonly from: string;
  /**
   * Envelope To attribute of the email message.
   */
  readonly to: string;
}
/**
 * An email message that is sent to a consumer Worker and can be rejected/forwarded.
 */
interface ForwardableEmailMessage extends StructuredEmailMessage {
  /**
   * Stream of the email message content.
   */
  readonly raw: ReadableStream<Uint8Array>;
  /**
   * An [Headers object](https://developer.mozilla.org/en-US/docs/Web/API/Headers).
   */
  readonly headers: Headers;
  /**
   * Size of the email message content.
   */
  readonly rawSize: number;
  /**
   * Reject this email message by returning a permanent SMTP error back to the connecting client including the given reason.
   * @param reason The reject reason.
   * @returns void
   */
  setReject(reason: string): void;
  /**
   * Forward this email message to a verified destination address of the account.
   * @param rcptTo Verified destination address.
   * @param headers A [Headers object](https://developer.mozilla.org/en-US/docs/Web/API/Headers).
   * @returns A promise that resolves when the email message is forwarded.
   */
  forward(rcptTo: string, headers?: Headers): Promise<EmailSendResult>;
  /**
   * Reply to the sender of this email message with a new EmailMessage object.
   * @param message The reply message.
   * @returns A promise that resolves when the email message is replied.
   */
  reply(message: StructuredEmailMessage): Promise<EmailSendResult>;
}
/** A file attachment for an email message */
type EmailAttachment =
  | {
      disposition: "inline";
      contentId: string;
      filename: string;
      type: string;
      content: string | ArrayBuffer | ArrayBufferView;
    }
  | {
      disposition: "attachment";
      contentId?: undefined;
      filename: string;
      type: string;
      content: string | ArrayBuffer | ArrayBufferView;
    };
/** An Email Address */
interface EmailAddress {
  name: string;
  email: string;
}
/**
 * A binding that allows a Worker to send email messages.
 */
interface SendEmail {
  // EDIT: this overload takes the `cloudflare:email`-imported
  // `EmailMessage` class directly — see the EDIT note on that module
  // declaration above for the naming rationale.
  send(message: EmailMessage): Promise<EmailSendResult>;
  send(builder: {
    from: string | EmailAddress;
    to: string | string[];
    subject: string;
    replyTo?: string | EmailAddress;
    cc?: string | string[];
    bcc?: string | string[];
    headers?: Record<string, string>;
    text?: string;
    html?: string;
    attachments?: EmailAttachment[];
  }): Promise<EmailSendResult>;
}
declare abstract class EmailEvent extends ExtendableEvent {
  readonly message: ForwardableEmailMessage;
}
declare type EmailExportedHandler<Env = unknown, Props = unknown> = (
  message: ForwardableEmailMessage,
  env: Env,
  ctx: ExecutionContext<Props>,
) => void | Promise<void>;
