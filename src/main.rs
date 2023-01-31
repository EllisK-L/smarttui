use libatasmart::Disk;
pub use libatasmart_sys::{SkDisk, SkSmartAttributeParsedData};
use tui::Tui;
use std::{fs, path::Path, ffi::c_void, ffi::CStr};

mod tui;
mod utils;
use crate::utils::Drive;

// extern "C" fn callback(d: *mut SkDisk, data: *const SkSmartAttributeParsedData, userdata: *mut c_void) {
//     unsafe {
//         let name = CStr::from_ptr((*data).name);
//         let name_str = name.to_str().unwrap();
//         println!("{name_str}");
//     }
// }

// fn get_disk_attributes()

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

fn get_disks(disk_names: Vec<String>) -> Vec<Drive> {
    let disk_prefix = String::from("/dev/");
    let mut disks: Vec<Drive> = Vec::new();

    for disk_name in disk_names{
        let full_name = disk_prefix.clone() + &disk_name;
        let new_disk = match Disk::new(Path::new(&(full_name))){
            Ok(disk) => disk,
            Err(_) => continue,
        };
        let new_drive = Drive{
            disk:new_disk,
            disk_name:disk_name
        };
        disks.push(new_drive);
    }
    disks
}



fn main() {
    let disk_names = get_disk_names();
    let drives = get_disks(disk_names);

    let mut tui = Tui::new();
    tui.build_ui(&drives);
    tui.run();
}