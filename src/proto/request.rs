/// 搜索请求消息
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
