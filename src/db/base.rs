use std::collections::HashMap;
use async_trait::async_trait;
use sqlx::{FromRow, SqlitePool};
use sqlx::Error::RowNotFound;
use sqlx::sqlite::SqliteRow;
use sqlx::types::JsonValue;
use tracing::log;


#[async_trait]
pub trait Table {
    // 返回表名
    fn table_name() -> String {
        // 默认行为：从类型名推导表名
        std::any::type_name::<Self>()
            .split("::")
            .last()
            .unwrap_or("unknown")
            .to_lowercase()
    }
}

#[async_trait]
pub trait InsertTable: Table {
    // 返回包含字段名和对应的值的哈希表，值为None的字段忽略
    fn to_fields(&self) -> HashMap<&'static str, Option<JsonValue>>;
}

#[async_trait]
pub trait SelectTable: Table {
    // 返回查询条件字段和对应的值
    fn to_conditions(&self) -> HashMap<&'static str, Option<JsonValue>>;
}

fn filter_non_empty_fields<K, V>(map: HashMap<K, Option<V>>) -> (Vec<K>, Vec<V>)
where
    K: Clone + Eq + std::hash::Hash,
    V: Clone,
{
    let fields = map.iter().filter(|(_, v)| v.is_some()).map(|(k, _)| k.clone()).collect();
    let values = map.into_iter().filter_map(|(_, v)| v).collect();
    (fields, values)
}

pub async fn insert_selective<T: InsertTable + Send + Sync>(pool: &SqlitePool, item: T) -> Result<(), sqlx::Error> {
    let fields_map = item.to_fields();

    // 提取非空字段和对应的值
    let (fields, values) = filter_non_empty_fields(fields_map);
    if fields.is_empty(){
        // 如果字段为空，则插入无效
        return Err(RowNotFound);
    }
    let placeholders: Vec<String> = (0..values.len()).map(|i| format!("${}", i)).collect();
    let query = format!(
        "INSERT INTO {} ({}) VALUES ({})",
        T::table_name(),
        fields.join(","),
        placeholders.join(",")
    );
    log::info!("Executing insert: {}", query);
    // 执行动态构建的查询
    let mut query_builder = sqlx::query(&query);
    // 使用 `bind_all` 动态绑定所有值
    for value in values {
        query_builder = query_builder.bind(value)
    }
    query_builder
        .execute(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to execute insert query:{}",e);
            e
        })?;
    Ok(())
}


pub async fn select_selective<T: SelectTable + for<'r> FromRow<'r, SqliteRow> + Send + Sync + Unpin>(pool: &SqlitePool, conditions: HashMap<&'static str, Option<JsonValue>>) -> Result<Vec<T>, sqlx::Error>
{
    // 提取非空条件字段和对应的值
    let (fields, values) = filter_non_empty_fields(conditions);

    // 构建 WHERE 子句
    let where_clause = if fields.is_empty() {
        String::new()
    } else {
        let conditions: Vec<String> = fields.iter().enumerate().map(|(i, field)| format!("{} = ${}", field, i + 1)).collect();
        format!("WHERE {}", conditions.join(" AND "))
    };

    // 构建完整查询语句
    let query = format!(
        "SELECT * FROM {} {}",
        T::table_name(),
        where_clause
    );

    log::info!("Executing query: {}", query);

    // 执行动态构建的查询
    let mut query_builder = sqlx::query_as::<_, T>(&query);
    for value in values {
        query_builder = query_builder.bind(value);
    }

    let results = query_builder
        .fetch_all(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to execute select query:{}",e);
            e
        })?;
    Ok(results)
}