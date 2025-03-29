use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::runtime::Builder;
use tokio::task;
use tokio::time::sleep;

fn main() {
    let mut counter = Arc::new(0);
    // EmailJob
    let email_handle = thread::spawn(move || {
        // some work here
        let runtime = Builder::new_multi_thread()
            .worker_threads(1)
            .thread_name("email-thread")
            .build()
            .unwrap();

        runtime.block_on(async move { email_job(counter) });
    });

    // Data processing
    let data_handle = thread::spawn(move || {
        // some work here
        let runtime = Builder::new_multi_thread()
            .worker_threads(9)
            .thread_name("data-thread")
            .build()
            .unwrap();

        runtime.block_on(async { data_processing_job() });
    });

    email_handle.join().unwrap();
    data_handle.join().unwrap();
}

async fn email_job(counter: Arc<i32>) {
    let total_worker_thread = 5;
    let mut handles = Vec::new();

    for _ in 0..total_worker_thread {
        let handle = task::spawn(async {
            let email_job = EmailJob::new();
            email_job.run().await;
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        let _ = handle.await;
    }
}

async fn data_processing_job() {
    let total_worker_thread = 5;
    let mut handles = Vec::new();

    for _ in 0..total_worker_thread {
        let handle = task::spawn(async {
            let data_processing_job = DataProcessingJob::new();
            data_processing_job.run().await;
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        let _ = handle.await;
    }
}

struct EmailJob {}

impl EmailJob {
    fn new() -> Self {
        EmailJob {}
    }

    async fn run(&self) {
        println!("email job running");
        sleep(Duration::from_secs(1)).await
    }
}

struct DataProcessingJob {}

impl DataProcessingJob {
    fn new() -> Self {
        DataProcessingJob {}
    }

    async fn run(&self) {
        println!("email job running");
        sleep(Duration::from_secs(5)).await
    }
}
