use sqlx::PgPool;
use crate::models::sys_dict_type::SysDictType;
use crate::errors::Result;
use crate::schemas::sys_dict_schemas::CreateDictTypeRequest;

#[derive(Debug)]
pub struct SysDictRepository {
	pub pool: PgPool,
}

impl SysDictRepository {
	pub fn new(pool: PgPool) -> Self {
		Self { pool }
	}

	pub async fn create(&self, dict_type: CreateDictTypeRequest)->Result<SysDictType>{
		let id = uuid::Uuid::new_v4().to_string();
		let query ="INSERT INTO sys_dict_type
			(id, dict_type, order_num, description, system_flag,status, remark,created_id,create_by,update_id,updated_at,is_deleted,deleted_at)
		VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12)
		RETURNING *";

		let dict_type = sqlx::query_as::<_, (SysDictType,)>(query)
		.bind(id).bind(dict_type.dict_type)
			.bind(dict_type.order_num).bind(dict_type.description)
			.bind(dict_type.system_flag).bind(dict_type.status).bind(dict_type.remark).fetch_one(&self.pool).await?;


	}
}
