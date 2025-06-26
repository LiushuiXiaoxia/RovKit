use crate::internal::snowflake::SnowflakeId;
use uuid::Uuid;

pub type SnowflakeGen = SnowflakeId;

/// 雪花 ID 生成器
pub fn snowflake(work_id: u64, datacenter_id: u64) -> SnowflakeGen {
    SnowflakeId::new(work_id, datacenter_id)
}

pub type UUID = Uuid;

/// UUID 生成器
pub fn uuid_v4() -> UUID {
    Uuid::new_v4()
}

mod tests {
    #[test]
    fn test_id_kit() {
        use crate::idkit::{snowflake, uuid_v4};
        let snowflake = snowflake(1, 1);
        for _ in 0..1000 {
            let id = snowflake.gen_id();
            let uuid = uuid_v4();
            println!("snowflake: {}", id);
            println!("uuid = {}", uuid);
        }
    }
}
