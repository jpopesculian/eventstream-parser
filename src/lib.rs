#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

mod event;
mod event_stream;
mod parser;
mod utf8_stream;
