use cursive::align::HAlign;
use cursive::traits::*;
use cursive::views::{Dialog, DummyView, LinearLayout, TextView, Panel, Button};

// This example uses a LinearLayout to stick multiple views next to each other.



fn main() {
    let mut siv = cursive::default();

    let mut main_view = LinearLayout::horizontal();

    let mut drive_list = LinearLayout::vertical();

    for _ in 0..5 {
        let button = Button::new("*", |s| s.quit());
        let panel = Panel::new(button).title("/dev/sda");

        drive_list.add_child(panel);
    }

    let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Mauris ac magna in velit interdum \
     hendrerit id a quam. Aenean at diam vel odio auctor malesuada ut sed lectus. Fusce gravida, purus ut iaculis malesuada, diam est cursus risus, et rhoncus dui eros \
     quis odio. Etiam cursus lobortis ante, vitae ullamcorper dui fermentum quis. Donec semper lacus sit amet placerat scelerisque. Praesent maximus justo quis justo \
     sagittis semper. Sed eu vehicula turpis, id molestie velit. Quisque consectetur eget est sed rhoncus. Fusce consequat, sem nec placerat tincidunt, \
     lorem elit eleifend augue, vitae vestibulum urna risus nec ipsum. Nunc nunc nisl, \
    maximus sit amet sollicitudin et, mattis nec lectus. Praesent mollis orci nec mi imperdiet maximus.";

    main_view.add_child(drive_list.scrollable());
    main_view.add_child(TextView::new(text).max_width(20).scrollable());

    siv.add_layer(main_view);

    siv.run();
}