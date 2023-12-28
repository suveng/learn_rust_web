这是一个用来测试grpc的测试例子;

例子使用tokio test宏开启异步运行时的环境;

例子使用全双工通信, 例子本身开启grpc server和 grpc client, 并且使用grpc client 调用本身的grpc server;

[参考tonic的github例子实现mock](https://github.com/hyperium/tonic/blob/master/examples/src/mock/mock.rs)

以下代码是单元测试的例子
```rust
use tonic::transport::{Endpoint, Server, Uri};
use tower;
use tower::service_fn;

use web::proto::api::search_service_client::SearchServiceClient;
use web::proto::api::search_service_server::SearchServiceServer;
use web::proto::request::SearchRequest;

#[tokio::test]
pub async fn test_grpc() -> Result<(), Box<dyn std::error::Error>> {
    let (client, server) = tokio::io::duplex(1024);

    let greeter = web::Api {};

    tokio::spawn(async move {
        Server::builder()
            .add_service(SearchServiceServer::new(greeter))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server)))
            .await
    });

    // Move client to an option so we can _move_ the inner value
    // on the first attempt to connect. All other attempts will fail.
    let mut client = Some(client);
    let channel = Endpoint::try_from("http://[::]:50051")?
        .connect_with_connector(service_fn(move |_: Uri| {
            let client = client.take();

            async move {
                if let Some(client) = client {
                    Ok(client)
                } else {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Client already taken",
                    ))
                }
            }
        }))
        .await?;

    let mut client = SearchServiceClient::new(channel);

    let request = tonic::Request::new(SearchRequest {
        name: "Tonic".into(),
    });

    let response = client.search(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
```