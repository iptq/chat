extern crate protoc_rust;

use protoc_rust::{Args, Customize};

fn main() {
    protoc_rust::run(Args {
        out_dir: "src/proto",
        input: &[
            "proto/chat.proto",
            "proto/util.proto",
        ],
        includes: &["proto"],
        customize: Customize {
            ..Default::default()
        },
    }).expect("protoc failed");
}
