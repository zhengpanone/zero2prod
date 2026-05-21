use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use crate::models::sys_dict_type::SysDictType;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateDictTypeRequest {
    /// 字典类型名称
    pub dict_type: String,
    /// 字典类型描述
    pub description: Option<String>,
    pub order_num: Option<u32>,
    pub status: String,
    /// 是否系统内置
    pub system_flag: bool,
    /// 备注信息
    pub remark: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateDictTypeRequest {}

#[derive(Debug, Serialize, ToSchema)]
pub struct SysDictTypeResponse{
    /// 字典类型ID
    pub id: String,
    /// 字典类型名称
    pub dict_type: String,
    /// 字典类型描述
    pub description: Option<String>,
    /// 是否系统内置
    pub system_flag: bool,
    /// 备注信息
    pub remark: String,
}

impl From<SysDictType> for SysDictTypeResponse {
    fn from(sys_dict_type: SysDictType) -> Self {
        Self{
            id: sys_dict_type.id,
            dict_type: sys_dict_type.dict_type,
            description: Some(sys_dict_type.description),
            system_flag: sys_dict_type.system_flag,
            remark: sys_dict_type.remark,
        }
    }
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateDictDataRequest {}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateDictDataRequest {}
