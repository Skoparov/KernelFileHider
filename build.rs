extern crate protoc_rust;

fn main() {
    protoc_rust::Codegen::new()
        .out_dir("src/gen")
        .inputs(&["proto/commands.proto", "proto/response.proto"])
        .include("proto")
        .run()
        .expect("Running protoc failed");
}

