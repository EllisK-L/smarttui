use libatasmart::Disk;
pub use libatasmart_sys::{SkDisk, SkSmartAttributeParsedData};
use std::{fs, path::Path, ffi::c_void, ffi::CStr};

extern "C" fn callback(d: *mut SkDisk, data: *const SkSmartAttributeParsedData, userdata: *mut c_void) {
    unsafe {
        let name = CStr::from_ptr((*data).name);
        let name_str = name.to_str().unwrap();
        println!("{name_str}");
    }
}

fn get_disk_attributes()

fn get_disk_names() -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    let paths = fs::read_dir("/sys/block").unwrap();

    for path in paths{
        let name = path.unwrap().file_name().into_string().unwrap();
        // println!("{name}");
        if !name.contains("loop") && name != "ram"{
            names.push(name);
        }
    }
    names
}

fn get_disks(disk_names: &Vec<String>) -> Vec<Disk> {
    let disk_prefix = String::from("/dev/");
    let mut disks: Vec<Disk> = Vec::new();

    for disk_name in disk_names{
        let full_name = disk_prefix.clone() + disk_name;
        println!("{}", full_name);
        let new_disk = Disk::new(Path::new(&(full_name))).unwrap(); //need to do error handling for permission denied/no support for smart
        disks.push(new_disk);
    }

    disks
}

fn main() {
    let disk_names = get_disk_names();
    let mut disks = get_disks(&disk_names);
    println!("{}", disks[0].get_disk_size().unwrap());
}