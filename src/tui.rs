use cursive::{CursiveRunnable};
use cursive::views::{LinearLayout, Panel, Button};

use crate::utils::Drive;

pub struct Tui {
    siv: CursiveRunnable,
}

impl Tui {
    pub fn new() -> Tui{
        Tui { siv: (cursive::default()) }
    }

    fn build_drive_list(&self, drives: &Vec<Drive>) -> LinearLayout{
        let mut drive_list = LinearLayout::vertical();

        for drive in drives {
            let button = Button::new("*", |s| s.quit());
            let panel = Panel::new(button).title(drive.disk_name.clone());

            drive_list.add_child(panel);
        }
        drive_list
    }

    pub fn build_ui(&mut self, disk_names: &Vec<Drive>){
        let drive_list = self.build_drive_list(disk_names);
        self.siv.add_layer(drive_list);
    }

    pub fn run(&mut self){
        self.siv.run();
    }
}