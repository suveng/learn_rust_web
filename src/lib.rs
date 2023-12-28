use std::net::SocketAddr;
use tokio::runtime::{Builder, Runtime};
use tonic::{Request, Response, Status};
use tonic::transport::Server;
use crate::proto::api::search_service_server::{SearchService, SearchServiceServer};
use crate::proto::request::SearchRequest;
use crate::proto::response::SearchResponse;

pub mod proto;

pub struct Api {}

#[tonic::async_trait]
impl SearchService for Api {
    /// #Example
    ///
    #[doc = include_str!("../tests/test_grpc_api.md")]
    // #[cfg(doctest)]
    async fn search(&self, request: Request<SearchRequest>) -> Result<Response<SearchResponse>, Status> {
        println!("请求: {:#?}", request);
        Ok(Response::new(SearchResponse{name:request.into_inner().name.clone()}))
    }
}



pub async fn grpc() {
    let socket_address: SocketAddr;
    match "127.0.0.1:50051".parse() {
        Ok(ok) => {
            socket_address = ok;
        }
        Err(err) => {
            eprint!("解析地址失败 {}", err);
            std::process::exit(1);
        }
    };

    let api: Api = Api {};

    Server::builder()
        .add_service(SearchServiceServer::new(api))
        .serve(socket_address)
        .await
        .expect("Failed to start server");
}

pub fn build_runtime() -> Runtime {
    let runtime: Runtime;
    match Builder::new_multi_thread().enable_all().build() {
        Ok(ok) => {
            runtime = ok;
        }
        Err(err) => {
            eprint!("创建运行时失败 {}", err);
            std::process::exit(1);
        }
    };
    runtime
}

