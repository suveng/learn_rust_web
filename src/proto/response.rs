/// 搜索响应消息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchResponse {
    /// 响应结果的名称
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// 在线用户数
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnlineUserCountResponse {
    /// 在线用户数
    #[prost(int32, tag = "1")]
    pub count: i32,
}
