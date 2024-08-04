// Building on the last exercise, we want all of the threads to complete their
// work. But this time, the spawned threads need to be in charge of updating a
// shared value: `JobStatus.jobs_done`

use std::{sync::Arc, thread, time::Duration, sync::Mutex};

struct JobStatus {
    jobs_done: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_done: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = status.clone();
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value
            let mut counter = status_shared.lock().unwrap();
            counter.jobs_done += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        // Print the value of the JobStatus.jobs_completed.
        println!("jobs completed {}", status.lock().unwrap().jobs_done);
        // Did you notice anything interesting in the output? 
        // Do you have to 'join' on all the handles?       
    }
}