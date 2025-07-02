mod tests {
    use rovkit::idkit::{snowflake_id, uuid_v4};

    #[test]
    fn test_id_kit() {
        let sf = snowflake_id(1, 1);
        for _ in 0..1000 {
            let id = sf.gen_id();
            let uuid = uuid_v4();
            println!("snowflake: {}", id);
            println!("uuid = {}", uuid);
        }
    }
}
