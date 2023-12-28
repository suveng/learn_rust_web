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
    // 构建gRPC服务器的地址
    let socket_address = build_addr();
    // 创建Api实例
    let api: Api = Api {};
    // 创建gRPC服务器并添加SearchServiceServer实例
    Server::builder()
        .add_service(SearchServiceServer::new(api))
        // 启动服务器并监听指定地址
        .serve(socket_address)
        // 等待服务器关闭
        .await
        // 服务器启动失败时抛出错误
        .expect("Failed to start server");
}


pub async fn grpc_web() {
    // 构建地址
    let socket_address = build_addr();

    // 创建 Api 实例
    let api: Api = Api {};
    // 创建 SearchServiceServer 实例，并传入 Api 实例
    let api = SearchServiceServer::new(api);
    // 启用 tonic_web
    let api = tonic_web::enable(api);

    // 创建 Server 实例
    Server::builder()
        // 允许 HTTP1 连接
        .accept_http1(true)
        // 添加服务
        .add_service(api)
        // 服务监听指定地址
        .serve(socket_address)
        // 异步启动服务器
        .await
        // 启动失败时输出错误信息
        .expect("Failed to start server");
}

fn build_addr() -> SocketAddr {
    // 将字符串地址解析为SocketAddr
    let socket_address: SocketAddr;
    match "127.0.0.1:50051".parse() {
        // 解析成功
        Ok(ok) => {
            socket_address = ok;
        }
        // 解析失败
        Err(err) => {
            // 打印错误信息
            eprint!("解析地址失败 {}", err);
            // 退出程序
            std::process::exit(1);
        }
    };
    socket_address
}


pub fn build_runtime() -> Runtime {
    // 创建一个新的多线程运行时环境
    let runtime: Runtime;
    match Builder::new_multi_thread().enable_all().build() {
        // 如果构建成功
        Ok(ok) => {
            runtime = ok;  // 将构建好的运行时环境赋值给变量runtime
        }
        // 如果构建失败
        Err(err) => {
            // 打印错误信息
            eprint!("创建运行时失败 {}", err);
            // 退出程序
            std::process::exit(1);
        }
    };
    runtime  // 返回变量runtime
}

