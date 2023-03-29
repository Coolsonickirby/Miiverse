#![feature(proc_macro_hygiene)]
#![feature(seek_stream_len)]
#![feature(llvm_asm)]

use serde::Serialize;
use arcropolis_api::*;
use std::io::Cursor;
use std::io::Write;
use std::fs::File;
use image::DynamicImage;
use skyline::{hook, install_hook, logging::hex_dump_ptr, hooks::{getRegionAddress, Region}};
use std::ptr::null;
use parking_lot::Mutex;

static ENDPOINT: &'static str = "http://10.0.0.42:6920";
static mut PLAYER_IDX: usize = 0;
static mut PLAYER_INFO_DONE: bool = false;
const TIMEOUT: u64 = 10;
// const FIGHTER_INFO_ADDR: usize = 0x066dcd0;
const FIGHTER_INFO_ADDR: usize = 0x17e44c0;

// Thank you to jam1garner for this
const MAX_WIDTH: usize = 2640;
const MAX_HEIGHT: usize = 1040;
const MAX_DATA_SIZE: usize = MAX_HEIGHT * MAX_WIDTH * 4;
const MAX_FILE_SIZE: usize = MAX_DATA_SIZE + 0xb0;

const PLAYER_INFOS_ADDRESS: usize = 0x5322680;
const PLAYER_INFO_LOOP_OVER: usize = 0x64f800;

// const MAX_FILE_SIZE: usize = 22165296;

#[derive(Serialize)]
pub struct Information {
    pub players: Vec<Player>
}

impl Information {
    fn new() -> Information {
        Information {
            players: vec![
                Player::new(),
                Player::new(),
                Player::new(),
                Player::new(),
                Player::new(),
                Player::new(),
                Player::new(),
                Player::new(),
            ]
        }
    }
}

#[derive(Serialize)]
pub struct Player {
    pub player_num: u8,
    pub player_chara: u8,
    pub is_cpu: bool,
    pub is_amiibo: bool,
    pub is_used: bool
}

impl Player {
    fn new() -> Player{
        Player {
            player_num: 0,
            player_chara: 0,
            is_cpu: false,
            is_amiibo: false,
            is_used: false,
        }
    }
}

lazy_static::lazy_static! {
    pub static ref MIIVERSE_INFO: Mutex<Information> = Mutex::new(Information::new());
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
    cleanPlayers();
    // unsafe { 
    //     while !PLAYER_INFO_DONE { 
    //         println!("Player Info not done!");
    //         std::thread::sleep(std::time::Duration::from_secs(1));
    //     }
    //     println!("Player Info done!");
    //     PLAYER_INFO_DONE = false; 
    // }
    // match minreq::post(ENDPOINT).with_header("Content-Type",  "application/json").with_json(&*MIIVERSE_INFO.lock()).unwrap().with_timeout(TIMEOUT).send() {
    //     Ok(s) => {
    //         println!("Request sent!");
    //         match image::load_from_memory(&s.as_bytes()) {
    //             Ok(img) => {
    //                 let img = img.to_rgba8();
    //                 let mut file = Cursor::new(data);
    //                 nutexb::writer::write_nutexb_unswizzled(
    //                     "posts",
    //                     &DynamicImage::ImageRgba8(img),
    //                     &mut file
    //                 ).unwrap();
                    
    //                 let out = file.into_inner();
    //                 println!("Posts loaded!");
                
    //                 let mut file = File::create("sd:/ultimate/test.nutexb").unwrap();
    //                 file.write_all(&out).unwrap();
    //                 Some(MAX_FILE_SIZE)
    //             }
    //             Err(_err) => {
    //                 println!("{:?}", _err);
    //                 load_original_file(_hash, data)
    //             }
    //         }
    //     }
    //     Err(_err) => {
    //         println!("{:?}", _err);
    //         load_original_file(_hash, data)
    //     }
    // }
    load_original_file(_hash, data)
}

fn cleanPlayers(){
    unsafe{ PLAYER_IDX = 0; }
    for player in MIIVERSE_INFO.lock().players.iter_mut() {
        player.player_num = 0;
        player.player_chara = 0;
        player.is_cpu = false;
        player.is_amiibo = false;
        player.is_used = false;
    }
}

#[hook(offset = FIGHTER_INFO_ADDR)]
unsafe fn processFighterInfo(var: *mut u32) {
    PLAYER_INFO_DONE = false;
    // println!("[processFighterInfo] Start");
    // // Modify the Fighter ID
    *(var.offset(0x8) as *mut u8) = 0x25;
    // // Print Fighter ID
    // println!("Fighter ID (param_1->0x8): {:#x}", *(var.offset(0x8) as *const u8));
    // println!("Fighter Slot (param_1->0x19): {:#x}", *(var.offset(0x19) as *const u8) as u16);
    // // Print player slot (Player 1, 2, etc...)
    // println!("Player Slot (param_1->0xA7): {:#x}", *(var.offset(0xA7) as *const u8));
    // println!("param_1->0x1A8: {:#x}", *(var.offset(0x1A8) as *const u8));
    // println!("param_1->0x1AA: {:#x}", *(var.offset(0x1AA) as *const u8));
    // println!("param_1->0x1AC: {:#x}", *(var.offset(0x1AC) as *const u8));
    // println!("param_1->0x1AE: {:#x}", *(var.offset(0x1AE) as *const u8));
    // println!("param_1->0x1B0: {:#x}", *(var.offset(0x1B0) as *const u8));
    // println!("param_1->0x62: {:#x}", *(var.offset(0x62) as *const u8));
    // println!("param_1->0x66: {:#x}", *(var.offset(0x66) as *const u8));
    // println!("param_1->0x49: {:#x}", *(var.offset(0x49) as *const u8));
    // println!("------------------------------------");
    // println!("Modifying Player Idx {}!", PLAYER_IDX);
    // MIIVERSE_INFO.lock().players[PLAYER_IDX].player_num = 0; // *(var.offset(0xA7) as *const u8)
    // MIIVERSE_INFO.lock().unwrap().players[PLAYER_IDX].player_chara = *(var.offset(0x8) as *const u8);
    // MIIVERSE_INFO.lock().unwrap().players[PLAYER_IDX].is_used = true;
    // PLAYER_IDX += 1;
    // Call the original function
    original!()(var);
    println!("[processFighterInfo] End");
}

#[hook(offset = PLAYER_INFO_LOOP_OVER)]
unsafe fn playerInfoLoopOver(var: *mut u32) {
    PLAYER_IDX = 0;
    PLAYER_INFO_DONE = true;
    println!("Player Loop Done!");
    // Call the original function
    original!()(var);
}

#[skyline::main(name = "Miiverse")]
pub fn main() {
    println!("[Miiverse::main] Setting up...");
    posts_hook::install("stage/pictochat2/battle/model/background_line_set/post.nutexb", MAX_FILE_SIZE); // 50mb since most posts end up being big because ????
    install_hook!(processFighterInfo);
    install_hook!(playerInfoLoopOver);
}