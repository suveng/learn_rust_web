use diesel::prelude::*;

/// 这个函数定义了一个结构体Post，它代表一个博客帖子。
///
/// 结构体有四个字段：id、title、body和published，分别表示帖子的ID、标题、内容和是否已发布。
///
/// 这个结构体使用了 Diesel ORM 的 Queryable 和 Selectable 特性，表示该结构体可以被查询和转换成 SQL 语句。
///
/// 通过 #[diesel(table_name = crate::mysql::schema::schema::posts)] 属性指定了数据库表名为 crate::mysql::schema::schema::posts，这样就可以在数据库中找到对应的帖子表。
///
/// 而 #[diesel(check_for_backend(diesel::mysql::Mysql))] 属性指定了后端为 diesel::mysql::Mysql，即 MySQL 数据库后端。
///
/// 这个结构体可以用于在 Rust 程序中对 MySQL 数据库的帖子表进行操作。
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::mysql::schema::schema::posts)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Post {
    /// 文章ID
    pub id: i32,
    /// 文章标题
    pub title: String,
    /// 文章内容
    pub body: String,
    /// 文章是否已发布
    pub published: bool,
}

