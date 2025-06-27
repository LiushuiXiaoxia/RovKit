use rovkit::idkit;

#[allow(dead_code)]
fn main() {
    let sf = idkit::snowflake_id(1, 1);
    for _ in 0..1000 {
        let id = sf.gen_id();
        let uuid = idkit::uuid_v4();
        println!("id: {}", id);
        println!("uuid = {}", uuid);
    }
}
