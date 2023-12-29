use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use web::mysql::diesel_connect_mysql::connect_mysql;
use web::mysql::model::models::Post;
use web::mysql::schema::schema::posts::dsl::posts;  // 使用 diesel 进行 MySQL 操作的依赖

#[test]
pub fn test_connect() {
    let connection = &mut connect_mysql();  // 连接 MySQL 数据库
    let results = posts
        .limit(5)  // 限制查询结果的数量为 5
        .select(Post::as_select())  // 选择返回的字段为 Post 结构体
        .load(connection)  // 加载查询结果
        .expect("Error loading posts");  // 如果加载失败，抛出错误

    println!("Displaying {} posts", results.len());  // 打印查询结果的数量
    for post in results {  // 遍历查询结果
        println!("{}", post.title);  // 打印 post 的标题
        println!("-----------\n");  // 打印分隔线
        println!("{}", post.body);  // 打印 post 的内容
    }
    return;  // 返回结果
}
