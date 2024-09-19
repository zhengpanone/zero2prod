use sqlx::SqlitePool;
use crate::handlers::user::WxUser;
use crate::models::user::UserDO;

// 查询所有用户
pub async fn get_all_users(pool: &SqlitePool) -> Result<Vec<UserDO>, sqlx::Error> {
    let users = sqlx::query_as::<_, UserDO>("SELECT * FROM users")
        .fetch_all(pool)
        .await;
    users
}

// 根据openid查询用户
pub async fn get_user_by_openid(pool: &SqlitePool, openid: &str) -> Result<UserDO, sqlx::Error> {
    let user = sqlx::query_as::<_, UserDO>("select * from users where openid = ?")
        .bind(openid)
        .fetch_one(pool)
        .await;
    user
}

pub async fn insert_user(pool: &SqlitePool, wx_user: &WxUser) -> Result<(), sqlx::Error> {
    sqlx::query_as!(UserDO, r#"insert into users(openid,session_key) values($1, $2)"#,
        wx_user.openid,
        wx_user.session_key)
        .execute(pool)
        .await?;
    Ok(())
}



#[cfg(test)]
mod tests {
    use uuid::Uuid;
    use super::*;

    use crate::db::connection::establish_connection;

    #[tokio::test]
    async fn test_get_all_users() {
        // 设置测试环境，例如：内存中的 SQLite 数据库
        let pool = establish_connection().await;
        // 在这里准备数据库并插入测试数据
        let result = get_all_users(&pool).await;
        assert!(result.is_ok());
        let users = result.unwrap();
        assert!(!users.is_empty()); // 或根据测试数据做其他断言
        println!("所有用户:{:?}",users);
    }


    #[tokio::test]
    async fn test_insert_user() {
        // 设置测试环境，例如：内存中的 SQLite 数据库
        let pool = establish_connection().await;
        // 在这里准备数据库并插入测试数据
       let wx_user =  &WxUser{
           openid: String::from(Uuid::new_v4()),
           session_key: "abcd".to_string(),
       };
        let result = insert_user(&pool,wx_user).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_user_by_openid() {
        // 设置测试环境，例如：内存中的 SQLite 数据库
        let pool = establish_connection().await;
        // 在这里准备数据库并插入测试数据
        let open_id = "f1bc715c-5f81-4e42-9a1b-f1926921989b";
        let result = get_user_by_openid(&pool,open_id).await;
        assert!(result.is_ok());
        let users = result.unwrap();
        println!("用户:{:?}",users);
        println!("用户:{}",users);
    }

}