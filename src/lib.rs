#![feature(proc_macro_hygiene)]
#![feature(seek_stream_len)]

use serde::Serialize;
use arcropolis_api::*;
use std::io::Cursor;
use std::io::Write;
use std::fs::File;
use image::DynamicImage;
use skyline::hooks::{getRegionAddress, Region};

static ENDPOINT: &'static str = "http://10.0.0.42:6920";
const TIMEOUT: u64 = 10;

// Thank you to jam1garner for this
const MAX_WIDTH: usize = 2640;
const MAX_HEIGHT: usize = 1040;
const MAX_DATA_SIZE: usize = MAX_HEIGHT * MAX_WIDTH * 4;
const MAX_FILE_SIZE: usize = MAX_DATA_SIZE + 0xb0;

const PLAYER_INFOS_ADDRESS: usize = 0x5322680;

// const MAX_FILE_SIZE: usize = 22165296;

extern "C" {
    #[link_name = "\u{1}_ZN2nn3hid12GetNpadStateEPNS0_16NpadFullKeyStateERKj"]
    pub fn get_pro_state(
        arg1: u64,
        arg2: &u32
    );
    #[link_name = "\u{1}_ZN2nn3hid12GetNpadStateEPNS0_17NpadHandheldStateERKj"]
    pub fn get_handheld_state(
        arg1: u64,
        arg2: &u32
    );
    #[link_name = "\u{1}_ZN2nn3hid12GetNpadStateEPNS0_16NpadJoyDualStateERKj"]
    pub fn get_dual_joycon_state(
        arg1: u64,
        arg2: &u32
    );
    #[link_name = "\u{1}_ZN2nn3hid12GetNpadStateEPNS0_16NpadJoyLeftStateERKj"]
    pub fn get_left_joycon_state(
        arg1: u64,
        arg2: &u32
    );
    #[link_name = "\u{1}_ZN2nn3hid12GetNpadStateEPNS0_17NpadJoyRightStateERKj"]
    pub fn get_right_joycon_state(
        arg1: u64,
        arg2: &u32
    );
}

struct NpadHandheldState {
    update_count: i64,
    buttons: u64,
    l_stick_x: i32,
    l_stick_y: i32,
    r_stick_x: i32,
    r_stick_y: i32,
    flags: u32
}


#[derive(Serialize)]
struct Information {
    pub players: Vec<Player>
}

#[derive(Serialize)]
struct Player {
    pub player_num: u8,
    pub player_chara: u8,
    pub is_cpu: bool,
    pub is_amiibo: bool
}

#[derive(Debug)]
#[repr(C)]
struct PlayerInfos {
    pub unk: [u8;0x4f],
    pub ice_climber_going_first: u32,
    pub unk2: u32,
    pub fighter_id: u32,
    pub redirected_fighter_id: u32,
    pub unk3: u32,
    pub fighter_slot: u32,
    pub unk4: [u8;0x80]
}

pub fn offset_to_addr(offset: usize) -> *const () {
    unsafe { (getRegionAddress(Region::Text) as *const u8).add(offset) as _ }
}

#[arc_callback]
pub fn posts_hook(_hash: u64, data: &mut [u8]) -> Option<usize>{
    let player_1 = Player {
        player_num: 0,
        player_chara: 0,
        is_cpu: false,
        is_amiibo: false,
    };
    
    let player_2 = Player {
        player_num: 3,
        player_chara: 11,
        is_cpu: false,
        is_amiibo: false,
    };
    
    let info = Information {
        players: vec![player_1, player_2]
    };
    
    println!("Sending Request...");
    
    match minreq::post(ENDPOINT).with_header("Content-Type",  "application/json").with_json(&info).unwrap().with_timeout(TIMEOUT).send() {
        Ok(s) => {
            println!("Request sent!");
            match image::load_from_memory(&s.as_bytes()) {
                Ok(img) => {
                    let img = img.to_rgba8();
                    let mut file = Cursor::new(data);
                    nutexb::writer::write_nutexb_unswizzled(
                        "posts",
                        &DynamicImage::ImageRgba8(img),
                        &mut file
                    ).unwrap();
                    
                    let out = file.into_inner();
                    println!("Posts loaded!");
                
                    let mut file = File::create("sd:/ultimate/test.nutexb").unwrap();
                    file.write_all(&out).unwrap();
                    Some(MAX_FILE_SIZE)
                }
                Err(_err) => {
                    println!("{:?}", _err);
                    load_original_file(_hash, data)
                }
            }
        }
        Err(_err) => {
            println!("{:?}", _err);
            load_original_file(_hash, data)
        }
    }

}

#[skyline::main(name = "Miiverse")]
pub fn main() {
    println!("Hello from Skyline Rust Plugin!");
    posts_hook::install("stage/pictochat2/battle/model/background_line_set/post.nutexb", MAX_FILE_SIZE); // 50mb since most posts end up being big because ????

    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_secs(10));
        let mut toggle_flag: bool = false;
        let mut state = NpadHandheldState {
            update_count: 0,
            buttons: 0,
            l_stick_x: 0,
            l_stick_y: 0,
            r_stick_x: 0,
            r_stick_y: 0,
            flags: 0,
        };
        let mut controller_value: u32 = 0x20;
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
            unsafe {
                get_handheld_state(
                    &mut state as *mut NpadHandheldState as u64,
                    &controller_value,
                );
                if (state.buttons & (1 << 9)) != 0 && (state.buttons & (1 << 8)) != 0 {
                    toggle_flag = true;
                }
                for x in 0..8 {
                    if toggle_flag {
                        break;
                    }
                    controller_value = x as u32;
                    get_pro_state(
                        &mut state as *mut NpadHandheldState as u64,
                        &controller_value,
                    );
                    if (state.buttons & (1 << 9)) != 0 && (state.buttons & (1 << 8)) != 0 {
                        toggle_flag = true;
                        break;
                    }
                }

                if toggle_flag {
                    let address: *const PlayerInfos = (offset_to_addr(PLAYER_INFOS_ADDRESS) as *const PlayerInfos);
                    let players = std::slice::from_raw_parts(address, 8);
                    println!("---------------------------");
                    let mut i = 0;
                    for p in players.iter() {
                        println!("Player ID: {}", i);
                        println!("Fighter ID: {}", p.fighter_id);
                        println!("---");
                        i += 1;
                    }
                    println!("---------------------------");
                }
            }
            toggle_flag = false;
        }
    });

}