#![feature(globs)]

extern crate libc;
extern crate rfmod;

use rfmod::enums::*;
use rfmod::types::*;
use rfmod::*;
use std::default::Default;
use std::io::timer::sleep;
use std::os;

fn main() {
    let channels = 2i32;
    let args = os::args();
    let tmp = args.tail();

    if tmp.len() < 1 {
        fail!("USAGE: ./3d [music_file]");
    }
    let fmod = match FmodSys::new() {
        Ok(f) => f,
        Err(e) => {
            fail!("FmodSys.new : {}", e);
        }
    };

    match fmod.init_with_parameters(10i32, FmodInitFlag(FMOD_INIT_NORMAL)) {
        fmod::Ok => {}
        e => {
            fail!("FmodSys.init failed : {}", e);
        }
    };

    println!("=========================================");
    println!("============== 3D example ===============");
    println!("=========================================");

    let arg1 = tmp.get(0).unwrap();
    let mut sound = match fmod.create_sound((*arg1).as_slice(), Some(FmodMode(FMOD_3D | FMOD_SOFTWARE)), None) {
        Ok(s) => s,
        Err(e) => fail!("create sound error: {}", e)
    };
    sound.set_3D_min_max_distance(4f32, 10000f32);
    sound.set_mode(FmodMode(FMOD_LOOP_NORMAL));

    let mut chan = match sound.play() {
        Ok(c) => c,
        Err(e) => fail!("sound.play error: {}", e)
    };
    chan.set_3D_attributes(&FmodVector{x: -10f32, y: 0f32, z: 0f32}, &Default::default());

    let mut last_pos = FmodVector::new();
    let mut listener_pos = FmodVector::new();
    let mut t = 0f32;
    let interface_update_time = 50f32;
    let compensate_time = 1000f32 / interface_update_time;

    while chan.is_playing().unwrap() {
        let forward = FmodVector{x: 0f32, y: 0f32, z: 1f32};
        let up = FmodVector{x: 0f32, y: 1f32, z: 0f32};
        let mut vel = FmodVector::new();

        listener_pos.x = (t * 0.05f32).sin() * 33f32; // left right ping-pong
        vel.x = (listener_pos.x - last_pos.x) * compensate_time;
        vel.y = (listener_pos.y - last_pos.y) * compensate_time;
        vel.z = (listener_pos.z - last_pos.z) * compensate_time;
        t += 30f32 * (1f32 / interface_update_time);

        last_pos = listener_pos;
        fmod.set_3D_listener_attributes(0, &listener_pos, &vel, &forward, &up);

        let mut tmp = String::from_str("|.......................<1>......................<2>....................|\r");
        unsafe { tmp.as_mut_bytes()[listener_pos.x as uint + 35u] = 'L' as u8; }
        print!("{}", tmp);
        fmod.update();
        sleep(interface_update_time as u64 - 1u64);
    }
}