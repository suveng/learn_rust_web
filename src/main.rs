use web::{build_runtime, grpc_web};

pub mod proto;

// 主函数
fn main() {
    // 构建运行时环境
    let runtime = build_runtime();
    // 在运行时环境中运行grpc_web函数
    runtime.block_on(grpc_web());
}


