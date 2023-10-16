/*
    We can fix the problem of the spawned thread not running
    or ending prematurely by saving the return value of 
    thread::spawn in a variable. The return type of 
    thread::spawn is JoinHandle. A JoinHandle is an owned 
    value that, when we call the join method on it, 
    will wait for its thread to finish. 
    
    The following code shows how to use the JoinHandle 
    of the thread created and call join to make sure 
    the spawned thread finishes before main exits:

    Source: 
        https://doc.rust-lang.org/book/ch16-01-threads.html
*/
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..11 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..6 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}