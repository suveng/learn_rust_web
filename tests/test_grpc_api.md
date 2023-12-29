这是一个用来测试grpc的测试例子;

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

/// 这个函数是一个用于测试gRPC的函数。
/// 首先，它创建了一个名为client和server的双向流，模拟客户端和服务器之间的连接。
/// 接下来，它使用一个Api实例创建了一个SearchServiceServer，用于提供搜索服务。
/// 然后，它使用Server来监听连接，并将创建的服务和服务器地址添加到Endpoint中。
/// 之后，它启动了一个新的任务，用于运行服务器，以便接受客户端的连接请求。
/// 在主函数中，它创建了一个SearchServiceClient，并将之前创建的通道传递给它。
/// 然后，它创建了一个包含搜索字符串的SearchRequest，并使用客户端发送请求。
/// 最后，它等待响应并将其打印出来。函数执行成功后，返回一个表示成功的结果。
#[tokio::test]
pub async fn test_grpc() -> Result<(), Box<dyn std::error::Error>> {
    // 创建客户端和服务器的双向流
    let (client, server) = tokio::io::duplex(1024);

    // 创建greeter实例
    let greeter = web::Api {};

    // 启动服务器，将服务和服务器地址添加到Endpoint中并监听连接
    tokio::spawn(async move {
        Server::builder()
            .add_service(SearchServiceServer::new(greeter))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server)))
            .await
    });

    // 将client移动到一个选项中，以便在第一次尝试连接时可以移动内部值。
    // 其他尝试将失败。
    let mut client = Some(client);
    // 尝试从URI连接到Endpoint，并使用所提供的连接器
    let channel = Endpoint::try_from("http://[::]:50051")?
        .connect_with_connector(service_fn(move |_: Uri| {
            // 获取client的值
            let client = client.take();

            // 返回一个异步任务
            async move {
                // 如果client存在，则返回client
                if let Some(client) = client {
                    Ok(client)
                } else {
                    // 否则返回错误
                    Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Client already taken",
                    ))
                }
            }
        }))
        .await?;

    // 创建SearchServiceClient并将channel传递给它
    let mut client = SearchServiceClient::new(channel);

    // 创建一个包含搜索字符串的SearchRequest
    let request = tonic::Request::new(SearchRequest {
        name: "Tonic".into(),
    });

    // 发送请求并等待响应
    let response = client.search(request).await?;

    // 打印响应
    println!("RESPONSE={:?}", response);

    // 返回成功
    Ok(())
}

```