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
