// @generated automatically by Diesel CLI.
/// 这个函数使用了diesel ORM库的table!宏来定义一个名为posts的表，它有四个字段：id、title、body和published。字段的类型分别为Integer、Varchar、Text和Bool。这个宏用于创建一个结构体，可以用来进行数据库操作。
diesel::table! {
    // 定义一个名为posts的表，主键为id
    posts (id) {
        // id为整数类型，作为主键
        id -> Integer,
        // title为字符串类型
        title -> Varchar,
        // body为文本类型
        body -> Text,
        // published为布尔类型，表示文章是否已发布
        published -> Bool,
    }
}
