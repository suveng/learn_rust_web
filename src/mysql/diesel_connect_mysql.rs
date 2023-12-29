use diesel::{Connection, MysqlConnection};

/// 这个函数是一个用于连接MySQL数据库的函数。它使用一个特定的URL来建立数据库连接，并在连接失败时打印错误信息。
/// # Example
#[doc = include_str!("../../tests/test_query_mysql.md")]
pub fn connect_mysql() -> MysqlConnection {
    // 连接MySQL数据库
    let url = "mysql://root:@localhost:3306/rust"; // 数据库连接URL
    MysqlConnection::establish(url) // 建立数据库连接
        .unwrap_or_else(|_| panic!("Error connecting to {}", url)) // 如果连接失败，打印错误信息
}

