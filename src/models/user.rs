use std::fmt;
use serde::Serialize;
use time::PrimitiveDateTime;

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct UserDO {
    pub id: i64,
    pub openid: String,
    pub session_key: String,
    pub create_at: PrimitiveDateTime,
    pub update_at: PrimitiveDateTime,
}


impl fmt::Display for UserDO {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "User ID: {}, Openid: {}, SessionKey: {}, createAt: {}, updateAt: {}", self.id, self.openid, self.session_key, self.create_at, self.update_at)
    }
}


