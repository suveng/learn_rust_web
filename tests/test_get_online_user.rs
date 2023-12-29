use std::future::IntoFuture;
use std::sync::{Arc, Barrier};
use std::thread;

use tonic::Request;
use tower;

use web::build_runtime;
use web::proto::api::user_service_client::UserServiceClient;

/// 这段 Rust 代码是一个测试函数，使用了 tokio 库来实现并发执行多个线程。这段代码的目的是并发地调用 get_online_user_num 方法来获取在线用户数量，并打印出相应的结果。
///
/// 具体来说，该函数实现了以下功能：
///
/// 1. 创建一个向量 multi_request，用于存储要创建的线程。
///
/// 2. 创建一个 Arc 对象 barrier，用于在线程等待的屏障。
///
/// 3.使用 for 循环创建 100 个线程，并将这些线程推入 multi_request 向量中。
///
/// 4. 在每个线程中，首先创建一个 tokio 的 Runtime 对象。
///
/// 5. 打印出等待的循环变量 k。
///
/// 6. 等待所有线程到达屏障。
///
/// 7. 使用 blocking 模式连接 UserServiceClient，并获取一个可变的客户端对象 client。
///
/// 8. 调用 client 的 get_online_user_num 方法，该方法返回一个 Future 对象，并阻塞当前线程等待结果。
///
/// 9. 打印出等待的循环变量 k 和响应结果。
///
/// 10. 遍历 multi_request 向量，等待所有线程结束。
///
/// 11. 返回一个表示成功的 Result 对象。
#[tokio::test]
pub async fn test_get_online_user() -> Result<(), Box<dyn std::error::Error>> {
    // 创建一个存储线程的向量
    let mut multi_request = vec![];
    // 创建一个 Arc 对象，并设置初始化参数为 100
    let barrier = Arc::new(Barrier::new(100));
    // 循环 100 次
    for i in 0..100 {
        // 克隆 barrier 对象
        let b = barrier.clone();
        // 克隆当前循环变量 i
        let k = i.clone();
        // 将线程推入向量中
        multi_request.push(thread::spawn(move || {
            // 创建一个 Tokio 的 Runtime 对象
            let runtime = build_runtime();
            // 打印等待的循环变量 k
            println!("wait {}",k);
            // 等待所有线程到达屏障
            b.wait();
            // 使用 blocking 模式连接 UserServiceClient，并获取一个 mut 对象
            let mut client = runtime.block_on(UserServiceClient::connect("http://127.0.0.1:50051")).unwrap();
            // 调用 get_online_user_num 方法，返回一个 Future 对象，并阻塞线程等待结果
            let response = runtime.block_on(client.get_online_user_num(Request::new(()))).unwrap();
            // 打印等待的循环变量 k 和响应结果
            println!("wait {} response={:?}",k, response.get_ref().count);
        }));
    }

    // 遍历 multi_request 向量，等待所有线程结束
    for t in multi_request {
        t.join().unwrap();
    }

    // 返回成功
    Ok(())
}

