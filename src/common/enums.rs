use serde::Serialize;
use sqlx::{Decode, Sqlite, Type};
use sqlx::sqlite::SqliteValueRef;

// 定义枚举来表示锁定状态
#[derive(Debug, Clone, Copy, Serialize)]
pub enum LockedStatus {
    UnLocked = 0,
    Locked = 1,
}

impl LockedStatus {
    // 从 `u8` 转换为 `LockStatus`
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(LockedStatus::UnLocked),
            1 => Some(LockedStatus::Locked),
            _ => None,
        }
    }
    // 将 `LockStatus` 转换为 `u8`
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }
}

impl TryFrom<u8> for LockedStatus {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(LockedStatus::UnLocked),
            1 => Ok(LockedStatus::Locked),
            _ => Err(()),
        }
    }
}

// 实现 sqlx::Type<Sqlite> 和 sqlx::Decode<Sqlite>
impl Type<Sqlite> for LockedStatus {
    fn type_info() -> sqlx::sqlite::SqliteTypeInfo {
        u8::type_info() // 将其视为 u8 类型
    }
}

impl<'r> Decode<'r, Sqlite> for LockedStatus {
    fn decode(value: SqliteValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let decoded_value = u8::decode(value)?; // 先解码为 u8
        LockedStatus::try_from(decoded_value).map_err(|_| "Invalid LockedStatus value".into())
    }
}