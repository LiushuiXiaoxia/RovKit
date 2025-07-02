#[cfg(test)]
mod tests {
    use rovkit::jobkit::{JobKit, JobType, MAX_HISTORY_RECORDS};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::time::Duration;

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[tokio::test]
    async fn test_interval_job_execution() {
        init_logger();
        let kit = JobKit::new(4);

        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        let _id = kit
            .add_job(
                "test_interval",
                JobType::Interval(Duration::from_millis(300)),
                move || {
                    counter_clone.fetch_add(1, Ordering::SeqCst);
                    Ok(())
                },
            )
            .unwrap();

        tokio::time::sleep(Duration::from_secs(1)).await;

        let count = counter.load(Ordering::SeqCst);
        assert!(count >= 2, "任务应至少执行两次，但执行了 {}", count);

        let history = kit.get_history("test_interval").unwrap();
        assert!(!history.is_empty());
        assert!(history.iter().all(|r| r.result.is_ok()));

        kit.stop("test_interval");
    }

    #[tokio::test]
    async fn test_cron_job_execution() {
        init_logger();
        let kit = JobKit::new(4);

        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        kit.add_job(
            "test_cron",
            JobType::Cron("*/1 * * * * *".into()),
            move || {
                counter_clone.fetch_add(1, Ordering::SeqCst);
                Ok(())
            },
        )
        .unwrap();

        tokio::time::sleep(Duration::from_secs(2)).await;

        let count = counter.load(Ordering::SeqCst);
        assert!(count >= 1, "Cron 任务应至少执行一次，但执行了 {} 次", count);

        let history = kit.get_history("test_cron").unwrap();
        assert!(!history.is_empty());

        kit.stop_all();
    }

    #[tokio::test]
    async fn test_panic_handling() {
        init_logger();
        let kit = JobKit::new(2);

        kit.add_job(
            "panic_job",
            JobType::Interval(Duration::from_millis(200)),
            || {
                panic!("panic in task");
            },
        )
        .unwrap();

        tokio::time::sleep(Duration::from_secs(1)).await;

        let history = kit.get_history("panic_job").unwrap();
        assert!(!history.is_empty());

        let panics = history
            .iter()
            .filter(|r| r.result.as_ref().is_err())
            .count();
        assert!(panics >= 2, "应至少捕获 2 次 panic，实际 {}", panics);

        kit.stop("panic_job");
    }

    #[tokio::test]
    async fn test_stop_by_name_and_id() {
        init_logger();
        let kit = JobKit::new(2);

        let _job_id = kit
            .add_job(
                "named_job",
                JobType::Interval(Duration::from_millis(100)),
                || Ok(()),
            )
            .unwrap();

        assert!(kit.stop("named_job"));
        assert!(!kit.stop("named_job")); // 已停止，不能再停

        let new_id = kit
            .add_job(
                "named_job2",
                JobType::Interval(Duration::from_millis(100)),
                || Ok(()),
            )
            .unwrap();

        assert!(kit.stop(&new_id));
    }

    #[tokio::test]
    async fn test_history_limit() {
        init_logger();
        let kit = JobKit::new(2);

        kit.add_job(
            "fast_job",
            JobType::Interval(Duration::from_millis(10)),
            || Ok(()),
        )
        .unwrap();

        tokio::time::sleep(Duration::from_millis(500)).await;

        let history = kit.get_history("fast_job").unwrap();
        assert!(history.len() <= MAX_HISTORY_RECORDS);
        kit.stop("fast_job");
    }
}
