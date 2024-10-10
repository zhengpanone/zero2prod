use std::collections::HashMap;
use std::fmt;

use serde::Serialize;
use sqlx::types::JsonValue;
use time::format_description::well_known::Rfc3339;
use time::PrimitiveDateTime;

use crate::common::enums::LockedStatus;
use crate::db::base::InsertTable;
use crate::utils::date;

// 定义用户数据对象 UserDO
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct UserDO {
    pub id: Option<i64>,
    pub openid: Option<String>,
    pub username: String,
    pub email: String,
    pub password: String,
    pub session_key: Option<String>,
    pub create_at: PrimitiveDateTime,
    pub update_at: PrimitiveDateTime,
    pub locked_at: Option<LockedStatus>, // 使用枚举来表示锁定状态
}


impl fmt::Display for UserDO {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "User ID: {:?}, Openid: {:?}, SessionKey: {:?}, createAt: {}, updateAt: {}", self.id, self.openid, self.session_key, self.create_at, self.update_at)
    }
}

impl InsertTable for UserDO {
    fn to_fields(&self) -> HashMap<&'static str, Option<JsonValue>> {
        let mut fields = HashMap::new();
        fields.insert("openid", self.openid.clone().map(JsonValue::from));
        fields.insert("username", Option::from(self.username.clone()).map(JsonValue::from));
        fields.insert("email", Option::from(self.email.clone()).map(JsonValue::from));
        fields.insert("password", Option::from(self.password.clone()).map(JsonValue::from));
        fields.insert("session_key", self.session_key.clone().map(JsonValue::from));
        // 时间类型的字段转换为字符串
        fields.insert(
            "create_at",
            Option::from(JsonValue::from(self.create_at.format(&Rfc3339).unwrap_or_else(|_| "".to_string()))));
        fields.insert(
            "update_at",
            Option::from(JsonValue::from(self.update_at.format(&Rfc3339).unwrap_or_else(|_| "".to_string()))));

        // 枚举类型 locked_at 转换为 u8，并转换为字符串
        fields.insert("locked_at", self.locked_at.map(|status| status.as_u8().to_string()).map(JsonValue::from));
        fields
    }

    fn table_name() -> &'static str {
        "users"
    }
}


impl UserDO {
    pub fn new<A, B, C>(username: A, email: B, password_hash: C) -> Self
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<String>,
    {
        let now = date::offset_to_primitive(date::now_utc());
        Self {
            id: None,
            openid: None,
            username: username.into(),
            password: password_hash.into(),
            email: email.into(),
            session_key: None,
            create_at: now,
            update_at: now,
            locked_at: Option::from(LockedStatus::UnLocked),
        }
    }
}


