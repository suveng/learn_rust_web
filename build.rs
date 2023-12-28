use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from("src/proto");

    tonic_build::configure()
        .out_dir(out_dir)
        .compile(&[
            "proto/api.proto",
            "proto/request.proto",
            "proto/response.proto",
        ], &[
            "proto"
        ])
        .unwrap();
}