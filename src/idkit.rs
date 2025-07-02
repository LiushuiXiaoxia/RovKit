use uuid::Uuid;

pub type SnowflakeGen = crate::internal::snowflake::SnowflakeId;

/// 雪花 ID 生成器
pub fn snowflake_id(work_id: u64, datacenter_id: u64) -> SnowflakeGen {
    SnowflakeGen::new(work_id, datacenter_id)
}

pub type UUID = Uuid;

/// UUID 生成器
pub fn uuid_v4() -> UUID {
    Uuid::new_v4()
}
