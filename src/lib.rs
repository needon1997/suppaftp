#![crate_name = "suppaftp"]
#![crate_type = "lib"]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! # SuppaFTP
//!
//! SuppaFTP is an FTP client library written in Rust with optional FTPS support.
//! You can choose whether to use **sync** or **async** version of this library using cargo.toml *features*.
//! SuppaFTP is a library derived from [rust-ftp](https://github.com/mattnenterprise/rust-ftp), which has many additional features though, such as:
//!
//! - New methods to work with streams when transferring files, to give you complete freedom when you work with file transfers
//! - Method to retrieve the welcome message
//! - Supports for both sync and **async** rust
//! - Some extra features, such as the parser for the **LIST** command output
//! - Replaced openssl with rustls or native-tls as you prefer
//! - All the old statements have been replaced with modern rust
//! - Better error handling and possibility to retrieve error codes
//! - Test units and high code coverage to provide the community with a reliable library
//!
//! ## Get started
//!
//! To get started, first add **suppaftp** to your dependencies:
//!
//! ```toml
//! suppaftp = "^5.30"
//! ```
//!
//! ### Features
//!
//! #### SSL/TLS Support
//!
//! If you want to enable **support for FTPS**, you must enable the `native-tls` or `rustls` feature in your cargo dependencies, based on the TLS provider you prefer.
//!
//! ```toml
//! suppaftp = { version = "^5.30", features = ["native-tls"] }
//! # or
//! suppaftp = { version = "^5.30", features = ["rustls"] }
//! ```
//!
//! > 💡 If you don't know what to choose, `native-tls` should be preferred for compatibility reasons.
//!
//! #### Async support
//!
//! If you want to enable **async** support, you must enable `async` feature in your cargo dependencies.
//!
//! ```toml
//! suppaftp = { version = "^5.30", features = ["async"] }
//! ```
//!
//! > ⚠️ If you want to enable both **native-tls** and **async** you must use the **async-native-tls** feature ⚠️
//! > ⚠️ If you want to enable both **rustls** and **async** you must use the **async-rustls** feature ⚠️
//!
//! #### Deprecated methods
//!
//! If you want to enable deprecated methods of FTPS, please enable the `deprecated` feature in your cargo dependencies.
//!
//! This feature enables these methods:
//!
//! - `connect_secure_implicit()`: used to connect via implicit FTPS
//!
//! ## Usage
//!
//! Here is a basic usage example:
//!
//! ```rust
//! use suppaftp::FtpStream;
//! let mut ftp_stream = FtpStream::connect("127.0.0.1:10021").unwrap_or_else(|err|
//!     panic!("{}", err)
//! );
//! assert!(ftp_stream.login("test", "test").is_ok());
//!
//! // Disconnect from server
//! assert!(ftp_stream.quit().is_ok());
//! ```
//!
//! ## FTPS
//!
//! The client supports FTPS on demand. To enable it the client should be
//! compiled with feature `secure` enabled which requires
//! [rust-native-tls](https://github.com/sfackler/rust-native-tls).
//!
//! The client uses explicit mode for connecting FTPS what means you should
//! connect the server as usually and then switch to the secure mode (TLS is used).
//! For better security it's the good practice to switch to the secure mode
//! before authentication.
//!
//! ### FTPS Usage
//!
//! ```rust
//! use suppaftp::{NativeTlsFtpStream, NativeTlsConnector};
//! use suppaftp::native_tls::{TlsConnector, TlsStream};
//!
//! let ftp_stream = NativeTlsFtpStream::connect("test.rebex.net:21").unwrap();
//! // Switch to the secure mode
//! let mut ftp_stream = ftp_stream.into_secure(NativeTlsConnector::from(TlsConnector::new().unwrap()), "test.rebex.net").unwrap();
//! ftp_stream.login("demo", "password").unwrap();
//! // Do other secret stuff
//! assert!(ftp_stream.quit().is_ok());
//! ```
//!
//! ## Going async
//!
//! SuppaFTP also supports **async** execution as said before, through the **async** feature.
//! Basically there's no difference in the function you can use when using the async version of suppaftp.
//! Let's quickly see in the example how it works
//!
//! ```rust
//! use suppaftp::{AsyncFtpStream, AsyncNativeTlsConnector};
//! use suppaftp::async_native_tls::{TlsConnector, TlsStream};
//!
//! let ftp_stream = AsyncFtpStream::connect("test.rebex.net:21").await.unwrap();
//! // Switch to the secure mode
//! let mut ftp_stream = ftp_stream.into_secure(AsyncNativeTlsConnector::from(TlsConnector::new()), "test.rebex.net").await.unwrap();
//! ftp_stream.login("demo", "password").await.unwrap();
//! // Do other secret stuff
//! assert!(ftp_stream.quit().await.is_ok());
//! ```
//!

#![doc(html_playground_url = "https://play.rust-lang.org")]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/veeso/suppaftp/main/assets/images/cargo/suppaftp-128.png"
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/veeso/suppaftp/main/assets/images/cargo/suppaftp-512.png"
)]

// -- common deps
#[macro_use]
extern crate lazy_regex;
#[macro_use]
extern crate log;

mod async_ftp;
pub(crate) mod command;
mod regex;
mod status;

// -- public
pub mod list;
pub mod types;

// -- export (common)
pub use status::Status;

// -- export async
use async_ftp::AsyncNoTlsStream;
pub use async_ftp::ImplAsyncFtpStream;
pub type AsyncFtpStream = ImplAsyncFtpStream<AsyncNoTlsStream>;
pub use async_ftp::AsyncRustlsConnector;
use async_ftp::AsyncRustlsStream;
pub use async_ftp::AsyncTlsStream;
pub type AsyncRustlsFtpStream = ImplAsyncFtpStream<AsyncRustlsStream>;

// -- test logging
#[cfg(test)]
pub fn log_init() {
    let _ = env_logger::builder().is_test(true).try_init();
}
