use std::{thread, time::{Instant, Duration}};
use num_cpus;

fn slow_computation() {
    let start_time = Instant::now();
    let run_duration = Duration::from_secs(2); // Duration to run the computation for

    while Instant::now() - start_time < run_duration {
        // Perform some computation that doesn't get optimized away
        // In this example, we square each number from 0 to 1000
        let _results: Vec<_> = (0..1000).map(|x| x * x).collect();
    }
}

fn main() {
    let cpu_count = num_cpus::get();
    let physical_cpu_count = num_cpus::get_physical();
    println!("Number of available CPUs: {}", cpu_count);
    println!("Number of physical CPUs: {}", physical_cpu_count);

    // create two threads and wait for them to finish
    let handle1 = thread::spawn(|| {
        for i in 1..5 {
            println!("Hello from thread 1: {}", i);
            slow_computation();
        }
    });
    let handle2 = thread::spawn(|| {
        for i in 1..5 {
            println!("Hello from thread 2: {}", i);
            slow_computation();
        }
    });
    handle1.join().unwrap();
    handle2.join().unwrap();

    // sleep for 10 seconds
    thread::sleep(Duration::from_secs(10));
}