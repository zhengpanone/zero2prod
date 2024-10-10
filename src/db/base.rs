use std::collections::HashMap;
use sqlx::SqlitePool;


#[async_trait::async_trait]
pub trait InsertTable {
    // 返回包含字段名和对应的值的哈希表，值为None的字段忽略
    fn to_fields(&self) -> HashMap<&'static str, Option<sqlx::types::JsonValue>>;
    // 返回表名
    fn table_name() -> &'static str;
}


pub async fn insert_selective<T: InsertTable>(pool: &SqlitePool, item: T) -> Result<(), sqlx::Error> {
    let fields_map = item.to_fields();

    // 提取非空字段和对应的值
    let fields: Vec<&str> = fields_map.iter().filter(|(_, v)| v.is_some()).map(|(k, _)| *k).collect();
    let values: Vec<sqlx::types::JsonValue> = fields_map.iter().filter_map(|(_, v)| v.clone()).collect();
    let placeholders: Vec<String> = (1..values.len()).map(|i| format!("${}", i)).collect();
    let query = format!(
        "INSERT INTO {} ({}) VALUES ({})",
        T::table_name(),
        fields.join(","),
        placeholders.join(",")
    );
    // 执行动态构建的查询
    let mut query_builder = sqlx::query(&query);
    // 使用 `bind_all` 动态绑定所有值
    for value in values {
        query_builder = query_builder.bind(value)
    }
    query_builder.execute(pool).await?;
    Ok(())
}
