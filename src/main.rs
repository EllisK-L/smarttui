use libatasmart::{Disk, nix::errno::Errno};
pub use libatasmart_sys::{SkDisk, SkSmartAttributeParsedData};
use tui::Tui;
use std::{fs, path::Path, ffi::c_void, ffi::CStr};
use std::process;

mod tui;
mod utils;
use crate::utils::Drive;

extern "C" fn callback(_d: *mut SkDisk, data: *const SkSmartAttributeParsedData, _userdata: *mut c_void) {
    unsafe {
        let name = CStr::from_ptr((*data).name);
        let name_str = name.to_str().unwrap();
        println!("{name_str}");
    }
}



fn get_disk_names() -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    let paths = fs::read_dir("/sys/block").unwrap();

    for path in paths{
        let name = path.unwrap().file_name().into_string().unwrap();
        if !name.contains("loop") && name != "ram"{
            names.push(name);
        }
    }
    names
}

pub fn get_drives(disk_names: Vec<String>) -> Vec<Drive> {
    let disk_prefix = String::from("/dev/");
    let mut drives: Vec<Drive> = Vec::new();

    for disk_name in disk_names{
        let full_name = disk_prefix.clone() + &disk_name;
        let new_drive = match create_drive(&full_name){
            Ok(drive) => drive,
            Err(_) => continue,
        };
        drives.push(new_drive);
    }
    if drives.len() == 0{
        println!("No drives with SMART have been detected");
        println!("This could be because you did not run this with root access, try again with sudo");
        process::exit(0);
    }
    drives
}

// This needs the full path, for example /dev/sdX
fn create_drive(disk_name: &String) -> Result<Drive, Errno>{
    let mut new_disk = match Disk::new(Path::new(disk_name)){
        Ok(disk) => disk,
        Err(err) => return Err(err),
    };

    let temp = new_disk.get_temperature().unwrap();

    let new_drive = Drive{
        disk:new_disk,
        disk_name:disk_name.clone(),
        tempature:temp,
    };

    Ok(new_drive)
}



fn main() {
    let disk_names = get_disk_names();
    let mut drives = get_drives(disk_names);

    drives[0].disk.parse_attributes(callback).unwrap();
    

    let mut tui = Tui::new();
    tui.build_ui(&drives);
    // tui.run();
}