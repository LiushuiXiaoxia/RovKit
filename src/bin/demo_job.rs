use rovkit::jobkit::{JobKit, JobType};
use std::time::Duration;

#[allow(dead_code)]
#[tokio::main]
async fn main() {
    env_logger::init();

    let kit = JobKit::new(10);

    kit.add_job(
        "job_print",
        JobType::Interval(Duration::from_secs(2)),
        || {
            println!("Hello from interval job");
            Ok(())
        },
    )
    .unwrap();

    kit.add_job(
        "job_panic_task",
        JobType::Cron("*/5 * * * * *".into()),
        || {
            panic!("模拟 panic");
            // #[allow(unreachable_code)]
            // Ok(())
        },
    )
    .unwrap();

    tokio::time::sleep(Duration::from_secs(10)).await;

    if let Some(records) = kit.get_history("job_print") {
        println!("执行记录:");
        for r in records {
            println!(
                "- [{}] {}, {:?} {:?}",
                r.name,
                r.timestamp,
                r.duration,
                r.result.as_ref().map(|_| "success").unwrap_or("failed")
            );
        }
    }

    kit.stop_all();
}
