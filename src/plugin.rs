#[no_mangle]
pub fn start() -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        println!("Plugin's Id {:?}", std::thread::current().id());
    })
}