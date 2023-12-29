use std::path::PathBuf;

fn main() {
    // 定义输出目录为"src/proto"
    let out_dir = PathBuf::from("src/proto");

    // 配置tonic_build
    tonic_build::configure()
        // 设置输出目录为out_dir
        .out_dir(out_dir)
        // 编译指定proto文件
        .compile(&[
            "proto/api.proto",
            "proto/request.proto",
            "proto/response.proto",
        ], &[
            "proto"
        ])
        // 返回结果，确保没有错误
        .unwrap();
}
