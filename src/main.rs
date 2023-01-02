use libatasmart::Disk;
use std::{fs, path::Path};

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
    let mut disks: Vec<Disk> = Vec::new();

    for disk_name in disk_names{
        let new_disk = Disk::new(Path::new(disk_name)).unwrap();
        disks.push(new_disk);
    }

    disks
}

fn main() {
    let disk_names = get_disk_names();
    let disks = get_disks(&disk_names);
}