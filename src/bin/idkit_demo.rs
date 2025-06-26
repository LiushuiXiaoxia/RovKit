use rovkit::idkit;

fn main() {
    let snowflake = idkit::snowflake(1, 1);
    for i in 0..1000 {
        let id = snowflake.gen_id();
        let uuid = idkit::uuid_v4();
        println!("id: {}", id);
        println!("uuid = {}", uuid);
    }
}
