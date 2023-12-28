/// 搜索响应消息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchResponse {
    /// 响应结果的名称
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
