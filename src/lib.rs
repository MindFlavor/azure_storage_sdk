#![feature(conservative_impl_trait)]

#[macro_use]
extern crate azure_core_sdk;
extern crate base64;

#[macro_use]
extern crate hyper;
extern crate mime;

extern crate chrono;
extern crate crypto;

extern crate futures;
extern crate hyper_tls;
extern crate native_tls;
extern crate tokio_core;

extern crate xml;

#[macro_use]
extern crate log;
extern crate quick_error;

extern crate url;

extern crate uuid;

pub mod client;
pub mod rest_client;
pub mod container;
pub mod blob;
