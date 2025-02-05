#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
//! <p>This is the <i>ACM Private CA API Reference</i>. It provides descriptions,
//! syntax, and usage examples for each of the actions and data types involved in creating
//! and managing private certificate authorities (CA) for your organization.</p>
//! <p>The documentation for each action shows the Query API request parameters and the XML
//! response. Alternatively, you can use one of the AWS SDKs to access an API that's
//! tailored to the programming language or platform that you're using. For more
//! information, see <a href="https://aws.amazon.com/tools/#SDKs">AWS
//! SDKs</a>.</p>
//! <p>Each ACM Private CA API operation has a quota that determines the number of times the operation
//! can be called per second. ACM Private CA throttles API requests at different rates depending
//! on the operation. Throttling means that ACM Private CA rejects an otherwise valid request
//! because the request exceeds the operation's quota for the number of requests per second.
//! When a request is throttled, ACM Private CA returns a <a href="https://docs.aws.amazon.com/acm-pca/latest/APIReference/CommonErrors.html">ThrottlingException</a> error. ACM Private CA does not guarantee a minimum request
//! rate for APIs. </p>
//! <p>To see an up-to-date list of your ACM Private CA quotas, or to request a quota increase,
//! log into your AWS account and visit the <a href="https://console.aws.amazon.com/servicequotas/">Service Quotas</a>
//! console.</p>

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

pub use config::Config;

mod aws_endpoint;
#[cfg(feature = "client")]
pub mod client;
pub mod config;
pub mod error;
mod error_meta;
pub mod input;
mod json_deser;
mod json_errors;
mod json_ser;
pub mod model;
mod no_credentials;
pub mod operation;
mod operation_deser;
mod operation_ser;
pub mod output;
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub use smithy_http::byte_stream::ByteStream;
pub use smithy_http::result::SdkError;
pub use smithy_types::Blob;
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("acmpca", PKG_VERSION);
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[cfg(feature = "client")]
pub use client::Client;
pub use smithy_http::endpoint::Endpoint;
