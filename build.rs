extern crate prost_build;

fn main() {
    prost_build::compile_protos(
        &["keys-ext/service/rpc.proto"],
        &["keys-ext/", "protopatch/"],
    )
    .unwrap();
}
