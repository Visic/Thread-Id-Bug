use std::thread::JoinHandle;
use libloading::{Library, Symbol};

fn main() {
    println!("Main's Id: {:?}", std::thread::current().id());
    
    let lib = Library::new("plugin.dll").unwrap();
    let start_fn = unsafe { lib.get::<Symbol<fn() -> JoinHandle<()>>>(b"start") }.unwrap();
    let join_handle = start_fn();
    join_handle.join().unwrap();
}