use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};
use lazy_static::lazy_static;

use tonic::{Request, Response, Status};

use crate::proto::api::user_service_server::UserService;
use crate::proto::response::OnlineUserCountResponse;

#[derive(Debug,Default)]
pub struct UserApi {}

lazy_static!{
    // 创建一个Arc<AtomicI32>类型的静态变量USER_COUNT，初始化值为0
    static ref USER_COUNT: Arc<AtomicI32> = Arc::new(AtomicI32::new(0));
}

#[tonic::async_trait]
impl UserService for UserApi {
    // 异步实现get_online_user_num函数，接收一个Request<>()类型的参数，返回Result类型的Response<OnlineUserCountResponse>结果
    /// # Example
    /// 这是一个并发调用 get_online_user_num 的例子, 测试线程安全
    #[doc = include_str!("../../tests/test_get_online_user.md")]
    async fn get_online_user_num(&self, request: Request<()>) -> Result<Response<OnlineUserCountResponse>, Status> {
        println!("获取用户数, 请求参数 {:?}", request);

        // 将1加到USER_COUNT变量上，并使用Ordering::SeqCst定义内存序
        USER_COUNT.fetch_add(1, Ordering::SeqCst);

        // 构造一个Response<OnlineUserCountResponse>类型的结果，并将USER_COUNT变量的值设置为响应体中的count字段
        // 使用USER_COUNT.fetch_or(0, Ordering::SeqCst)将当前USER_COUNT的值与0进行或操作，并将结果赋值给响应体中的count字段
        Ok(Response::new(OnlineUserCountResponse {
            count: USER_COUNT.fetch_or(0, Ordering::SeqCst)
        }))
    }
}
