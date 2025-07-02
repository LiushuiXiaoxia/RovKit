use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use chrono::{DateTime, Utc};
use cron::Schedule;
use std::str::FromStr;
use tokio::task::JoinHandle;
use tokio::time;
use uuid::Uuid;

/// Max number of job history records to keep
pub const MAX_HISTORY_RECORDS: usize = 1000;

/// Job type: Interval or Cron
#[derive(Clone)]
pub enum JobType {
    Interval(Duration),
    Cron(String),
}

/// Execution record
#[derive(Debug, Clone)]
pub struct JobRecord {
    pub name: String,
    pub timestamp: DateTime<Utc>,
    pub duration: Duration,
    pub result: Result<(), String>,
}

/// Job function type
pub type JobFn = dyn Fn() -> Result<(), Box<dyn std::error::Error + Send + Sync>> + Send + Sync;

/// Metadata
#[derive(Clone)]
pub struct JobMeta {
    pub id: String,
    pub name: String,
    pub job_type: JobType,
    pub task: Arc<JobFn>,
}

/// Single job entry
pub struct JobEntry {
    pub meta: JobMeta,
    pub handle: Option<JoinHandle<()>>,
    pub history: Arc<Mutex<Vec<JobRecord>>>,
}

/// Job manager
pub struct JobKit {
    jobs: Arc<Mutex<HashMap<String, JobEntry>>>,
    name_map: Arc<Mutex<HashMap<String, String>>>,
    _max_concurrent_tasks: usize,
}

impl JobKit {
    pub fn new(_max_concurrent_tasks: usize) -> Self {
        Self {
            jobs: Arc::new(Mutex::new(HashMap::new())),
            name_map: Arc::new(Mutex::new(HashMap::new())),
            _max_concurrent_tasks,
        }
    }

    /// Add a new job
    pub fn add_job(
        &self,
        name: &str,
        job_type: JobType,
        task: impl Fn() -> Result<(), Box<dyn std::error::Error + Send + Sync>> + Send + Sync + 'static,
    ) -> Result<String, String> {
        let name = name.to_string();
        if self.name_map.lock().unwrap().contains_key(&name) {
            return Err("Job name already exists".to_string());
        }

        let id = Uuid::new_v4().to_string();
        let meta = JobMeta {
            id: id.clone(),
            name: name.clone(),
            job_type: job_type.clone(),
            task: Arc::new(task),
        };

        let history = Arc::new(Mutex::new(Vec::new()));
        let task_clone = meta.task.clone();
        let history_clone = history.clone();
        let task_name = name.clone();

        let handle = match job_type {
            JobType::Interval(interval) => tokio::spawn(Self::interval_runner(
                task_name,
                task_clone,
                history_clone,
                interval,
            )),
            JobType::Cron(expr) => {
                let schedule = Schedule::from_str(&expr).map_err(|e| e.to_string())?;
                tokio::spawn(Self::cron_runner(
                    task_name,
                    task_clone,
                    history_clone,
                    schedule,
                ))
            }
        };

        let entry = JobEntry {
            meta,
            handle: Some(handle),
            history,
        };

        self.jobs.lock().unwrap().insert(id.clone(), entry);
        self.name_map.lock().unwrap().insert(name, id.clone());
        Ok(id)
    }

    async fn interval_runner(
        task_name: String,
        task: Arc<JobFn>,
        history: Arc<Mutex<Vec<JobRecord>>>,
        interval: Duration,
    ) {
        let mut ticker = time::interval(interval);
        loop {
            ticker.tick().await;
            Self::execute_task(&task_name, &task, &history);
        }
    }

    async fn cron_runner(
        task_name: String,
        task: Arc<JobFn>,
        history: Arc<Mutex<Vec<JobRecord>>>,
        schedule: Schedule,
    ) {
        let mut upcoming = schedule.upcoming(Utc);
        while let Some(next) = upcoming.next() {
            let now = Utc::now();
            let delay = (next - now).to_std().unwrap_or(Duration::ZERO);
            time::sleep(delay).await;
            Self::execute_task(&task_name, &task, &history);
        }
    }

    fn execute_task(task_name: &str, task: &Arc<JobFn>, history: &Arc<Mutex<Vec<JobRecord>>>) {
        let start = Instant::now();
        let ts = Utc::now();
        let task = Arc::clone(task);

        let result = std::thread::spawn(move || {
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| task()))
        })
        .join()
        .map_err(|_| "Task thread panicked".to_string())
        .and_then(|r| {
            r.map_err(|_| "Task panicked".to_string())
                .and_then(|res| res.map_err(|e| e.to_string()))
        });

        let duration = start.elapsed();
        let record = JobRecord {
            name: task_name.to_string(),
            timestamp: ts,
            duration,
            result,
        };

        if let Err(ref e) = record.result {
            eprintln!(
                "[Task Error] name = {}, time = {}, e = {}",
                task_name, ts, e
            );
        }

        Self::push_history(history, record);
        println!(
            "[Task Success] name = {}, time = {}, duration = {:?}",
            task_name, ts, duration
        );
    }

    fn push_history(history: &Arc<Mutex<Vec<JobRecord>>>, record: JobRecord) {
        let mut h = history.lock().unwrap();
        h.push(record);
        if h.len() > MAX_HISTORY_RECORDS {
            h.remove(0);
        }
    }

    /// Stop a job by id or name
    pub fn stop(&self, id_or_name: &str) -> bool {
        let id = self.resolve_id(id_or_name);
        if let Some(id) = id {
            if let Some(mut job) = self.jobs.lock().unwrap().remove(&id) {
                if let Some(handle) = job.handle.take() {
                    handle.abort();
                }
                self.name_map.lock().unwrap().retain(|_, v| v != &id);
                return true;
            }
        }
        false
    }

    /// Stop all jobs
    pub fn stop_all(&self) {
        let mut jobs = self.jobs.lock().unwrap();
        for (_, job) in jobs.drain() {
            if let Some(handle) = job.handle {
                handle.abort();
            }
        }
        self.name_map.lock().unwrap().clear();
    }

    /// Get job history by id or name
    pub fn get_history(&self, id_or_name: &str) -> Option<Vec<JobRecord>> {
        let id = self.resolve_id(id_or_name)?;
        self.jobs
            .lock()
            .unwrap()
            .get(&id)
            .map(|entry| entry.history.lock().unwrap().clone())
    }

    /// List all jobs: (name, id)
    pub fn list_jobs(&self) -> Vec<(String, String)> {
        self.jobs
            .lock()
            .unwrap()
            .values()
            .map(|entry| (entry.meta.name.clone(), entry.meta.id.clone()))
            .collect()
    }

    fn resolve_id(&self, id_or_name: &str) -> Option<String> {
        if self.jobs.lock().unwrap().contains_key(id_or_name) {
            Some(id_or_name.to_string())
        } else {
            self.name_map.lock().unwrap().get(id_or_name).cloned()
        }
    }
}
