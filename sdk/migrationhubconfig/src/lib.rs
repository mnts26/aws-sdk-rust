#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
//! <p>The AWS Migration Hub home region APIs are available specifically for working with your
//! Migration Hub home region. You can use these APIs to determine a home region, as well as to
//! create and work with controls that describe the home region.</p>
//! <ul>
//! <li>
//! <p>You must make API calls for write actions (create, notify, associate, disassociate,
//! import, or put) while in your home region, or a <code>HomeRegionNotSetException</code>
//! error is returned.</p>
//! </li>
//! <li>
//! <p>API calls for read actions (list, describe, stop, and delete) are permitted outside of
//! your home region.</p>
//! </li>
//! <li>
//! <p>If you call a write API outside the home region, an <code>InvalidInputException</code>
//! is returned.</p>
//! </li>
//! <li>
//! <p>You can call <code>GetHomeRegion</code> action to obtain the account's Migration Hub
//! home region.</p>
//! </li>
//! </ul>
//! <p>For specific API usage, see the sections that follow in this AWS Migration Hub Home Region
//! API reference. </p>

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
    aws_http::user_agent::ApiMetadata::new("migrationhubconfig", PKG_VERSION);
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[cfg(feature = "client")]
pub use client::Client;
pub use smithy_http::endpoint::Endpoint;
