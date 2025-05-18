// Using Threads to Run Code Simultaneously: - this is a way of solving for (data races, deadlocks i.e two threads waiting on each other to complete and Bugs happening 
// - in one situation and hard to reproduce and fix reliably.
// Rust std lib use a 1:1 model of thread implementation, where a program uses one operating system thread per one language thread.
// There are crates that implement other models, making different trade-offs to the 1:1 model - (Rust's async system).


// Creating a New Thread with 'spawn'

use std::thread;
use std::time::Duration;

fn main() {
    // thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("Hi number {i} from the main thread");
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // for i in 1..5 {
    //     println!("Molo ke wena uqatywe ibhotolo ne jem {i} ousuka phi mbhem");
    //     thread::sleep(Duration::from_millis(1));
    // }
    // println!("Hello, world!");

    // Waiting for All Threads to Finish Using 'join'Handles:

    // - the code above not only stops the spawned thread prematurely most of the time due to main thread ending, but bcs theer is no guarantee on the order in which threads run, 
    // we also can't guarantee that the spawned thread will get to run at all.
    // We can fix the problem of prematurely endig threads or not running at all by saving the return value of 'thread::spawn'in a variable.
    // The return type of 'thread::spawn' is 'JoinHandle<T>'
    // A 'JoinHandle<T>' is an owned value that, when we call the join method on it, will wait for the its thread to finish. See here:

//     use std::thread;
//     use std::time::Duration;

// fn main() {
    let handle = thread::spawn(|| {
        for i in 1..35 {
            println!("Molweni apha {i} nisuka phi? ");
            thread::sleep(Duration::from_millis(1));
        }
    });

     handle.join().unwrap(); // where you call '.join affects how your threads run - first or last !

    for i in 1..10 {
        println!("Injalo ke mnumzana {i} osuka eQokolweni");
        thread::sleep(Duration::from_millis(1));
    }

   
//}    

}
