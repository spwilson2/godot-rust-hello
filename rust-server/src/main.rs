use std::mem::{size_of_val, size_of};
use std::ptr::slice_from_raw_parts;
use std::time::{Duration, SystemTime};
use std::{io::prelude::*, thread};
use std::net::TcpStream;

use bytemuck::{Pod, Zeroable, bytes_of};


fn update_tick(){}


fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:13001").unwrap();

    assert_eq!(std::mem::size_of::<Command>(), 20);
    thread::sleep(Duration::from_secs(2));
    let time = SystemTime::now();
    let mut cmds = Vec::<Command>::new();
    let entities = 5000;
    for i in 0..entities {
        let x = fastrand::f32() * 50.0;
        let y = fastrand::f32() * 50.0;
        cmds.push(spawn(x, y, i))
    }


    //let cmds = cmds.chunks(10);
    //let cmds = cmds.iter();
    let cmds = cmds.chunks((entities/10) as usize);
    for chnk in cmds {
        unsafe {
            //stream.write_all(std::mem::transmute::<&[Command],&[u8]>(chnk.as_slice())).unwrap();
            let slc = slice_from_raw_parts(chnk.as_ptr().cast() as *const u8, chnk.len() * size_of_val(&chnk[0]));
            stream.write_all(&*slc).unwrap();
            //stream.write_all(bytemuck::bytes_of(chnk)).unwrap();
        }
        //stream.read(&mut data).unwrap();
    }

    let mut cmds = Vec::<Command>::new();
    for i in 0..(60*100000) {
        for j in 0..500 {
            let id = fastrand::i32(0..entities);
            let x = fastrand::f32() * 50.0;
            let y = fastrand::f32() * 50.0;
            cmds.push(move_(x,y,id))
        }
        unsafe {
        let slc = slice_from_raw_parts(cmds.as_ptr().cast() as *const u8, cmds.len() * size_of::<Command>());
        stream.write_all(&*slc).unwrap();
        }
        cmds.clear();
        thread::sleep(Duration::from_secs_f32(1.0/30.0));
    }
    unsafe {
        for i in 0..2 {
            //let slc = slice_from_raw_parts((&end(0.0,0.0,0)) as *const _ as *const u8, size_of::<Command>());
            stream.write_all(bytes_of(&end(0.0,0.0,0))).unwrap();
            stream.flush().unwrap();
        }
    }
    let mut data = [0; 128];
    stream.read(&mut data).unwrap();

    println!("{:?}", time.elapsed().unwrap().as_millis());
    Ok(())
}

fn move_(x: f32,y: f32, id: i32, ) -> Command{
    let mut buf = [0;8];
    buf.copy_from_slice("move\0\0\0\0".as_bytes());
    Command { cmd: buf, pox_x: x, pox_y: y, uid: id}
}
fn spawn(x: f32,y: f32, id: i32, ) -> Command{
    let mut buf = [0;8];
    buf.copy_from_slice("spawn\0\0\0".as_bytes());
    Command { cmd: buf, pox_x: x, pox_y: y, uid: id}
}
fn end(x: f32,y: f32, id: i32, ) -> Command{
    let mut buf = [0;8];
    buf.copy_from_slice("end\0\0\0\0\0".as_bytes());
    Command { cmd: buf, pox_x: x, pox_y: y, uid: id}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Command {
    cmd: [u8;8], // Ascii string
    pox_x: f32,
    pox_y: f32,
    uid: i32,
}


unsafe impl Zeroable for Command { }
unsafe impl Pod for Command { }