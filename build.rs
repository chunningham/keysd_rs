fn main() {
    protoc_rust_grpc::Codegen::new()
        .input("keys-ext/service/rpc.proto")
        .includes(&["keys-ext/", "protopatch/"])
        .out_dir("src")
        .rust_protobuf(true)
        .run()
        .expect("protoc_rust_grpc")
}
