你需要先启动服务端 http://127.0.0.1:50051
`cargo run`

```rust 
use std::future::IntoFuture;
use std::sync::{Arc, Barrier};
use std::thread;

use tonic::Request;
use tower;

use web::build_runtime;
use web::proto::api::user_service_client::UserServiceClient;

#[tokio::test]
pub async fn test_get_online_user() -> Result<(), Box<dyn std::error::Error>> {
    let mut multi_request = vec![];
    let barrier = Arc::new(Barrier::new(100));
    for i in 0..100 {
        let b = barrier.clone();
        let k = i.clone();
        multi_request.push(thread::spawn(move || {
            let runtime = build_runtime();
            println!("wait {}",k);
            b.wait();
            let mut client = runtime.block_on(UserServiceClient::connect("http://127.0.0.1:50051")).unwrap();
            let response = runtime.block_on(client.get_online_user_num(Request::new(()))).unwrap();
            println!("wait {} response={:?}",k, response.get_ref().count);
        }));
    }

    for t in multi_request {
        t.join().unwrap();
    }

    Ok(())
}
```