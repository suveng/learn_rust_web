use web::{build_runtime, grpc};

pub mod proto;
fn main() {
    let runtime = build_runtime();
    runtime.block_on(
        grpc()
    );
}


