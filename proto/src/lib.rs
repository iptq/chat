//! Exposes an API for encoding and decoding byte streams into structured data according to protocol.

#![deny(missing_docs)]

extern crate bytes;
extern crate failure;
extern crate protobuf;
extern crate tokio_io;

mod codec;
mod proto;
