use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        OnceLock,
    },
    thread,
    time::{Duration, SystemTime},
};

extern "C" {
    pub fn gds_move(id: usize, x: u32, y: u32);
    pub fn gds_instantiate(x: u32, y: u32);
}

static mut CELL: OnceLock<AtomicBool> = OnceLock::new();

fn run_loop() {
    unsafe {
        let time = SystemTime::now();
        for i in 0..100000 {
            let x = fastrand::f32() * 500.0;
            let y = fastrand::f32() * 500.0;
            gds_instantiate(x as u32, y as u32);
        }
        while !CELL.get().unwrap().load(Ordering::Acquire) {
            thread::sleep(Duration::from_micros(1));
        }
        println!("{:?}", time.elapsed().unwrap().as_millis());
    }
}

#[no_mangle]
pub unsafe extern "C" fn done_world() {
    CELL.get_mut().unwrap().store(true, Ordering::Release);
}

#[no_mangle]
pub unsafe extern "C" fn hello_world() {
    println!("Hello world");
    let handle = std::thread::spawn(run_loop);
    std::mem::forget(handle);

    CELL.set(AtomicBool::new(false)).unwrap();
}
