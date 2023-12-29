use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use web::mysql::diesel_connect_mysql::connect_mysql;
use web::mysql::model::models::Post;
use web::mysql::schema::schema::posts::dsl::posts;

#[test]
pub fn test_connect() {
    let connection = &mut connect_mysql();
    let results = posts
        .limit(5)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
    return;
}