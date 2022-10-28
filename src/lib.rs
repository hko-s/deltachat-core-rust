//! # Delta Chat Core Library.

#![recursion_limit = "256"]
#![forbid(unsafe_code)]
#![warn(
    unused,
    clippy::correctness,
    missing_debug_implementations,
    clippy::all,
    clippy::indexing_slicing,
    clippy::wildcard_imports,
    clippy::needless_borrow,
    clippy::cast_lossless,
    clippy::unused_async
)]
#![allow(
    clippy::match_bool,
    clippy::mixed_read_write_in_expression,
    clippy::bool_assert_comparison,
    clippy::manual_split_once,
    clippy::format_push_string
)]

#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate smallvec;
#[macro_use]
extern crate rusqlite;
#[macro_use]
extern crate strum_macros;

pub trait ToSql: rusqlite::ToSql + Send + Sync {}

impl<T: rusqlite::ToSql + Send + Sync> ToSql for T {}

#[macro_use]
pub mod log;

#[cfg(feature = "internals")]
#[macro_use]
pub mod sql;
#[cfg(not(feature = "internals"))]
#[macro_use]
mod sql;

pub mod headerdef;

pub(crate) mod events;
pub use events::*;

mod aheader;
mod blob;
pub mod chat;
pub mod chatlist;
pub mod config;
mod configure;
pub mod constants;
pub mod contact;
pub mod context;
mod decrypt;
pub mod download;
mod e2ee;
pub mod ephemeral;
mod imap;
pub mod imex;
mod scheduler;
#[macro_use]
mod job;
mod format_flowed;
pub mod key;
mod keyring;
pub mod location;
mod login_param;
pub mod message;
mod mimefactory;
pub mod mimeparser;
pub mod oauth2;
mod param;
pub mod peerstate;
pub mod pgp;
pub mod provider;
pub mod qr;
pub mod qr_code_generator;
pub mod quota;
pub mod securejoin;
mod simplify;
mod smtp;
pub mod stock_str;
mod sync;
mod token;
mod update_helper;
pub mod webxdc;
#[macro_use]
mod dehtml;
mod authres;
mod color;
pub mod html;
pub mod plaintext;
mod ratelimit;
pub mod summary;

pub mod receive_imf;
pub mod tools;

pub mod accounts;
pub mod reaction;

/// if set imap/incoming and smtp/outgoing MIME messages will be printed
pub const DCC_MIME_DEBUG: &str = "DCC_MIME_DEBUG";

#[cfg(test)]
mod test_utils;
#[cfg(test)]
mod tests;
