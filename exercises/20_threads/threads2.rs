// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.

use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // `Arc` and `Mutex` to share and mutate the state across threads safely
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));

    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            let mut status_mut = status_shared.lock().unwrap();
            // Increment the jobs_completed count
            status_mut.jobs_completed += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Print the value of `JobStatus.jobs_completed`
    let status_final = status.lock().unwrap();
    println!("Jobs completed: {}", status_final.jobs_completed);
}
