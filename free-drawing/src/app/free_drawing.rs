use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use gtk::{DrawingArea, Button, Label, Box, Orientation};

use std::rc::Rc;

pub fn start() {
    let app = Application::builder()
    .application_id("app.example.free_drawing.com")
    .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {

    let width = 300;
    let height = 300;

    let vbox =  Box::new(Orientation::Vertical, 5);

    let hbox = [
        Box::new(Orientation::Vertical, 5),
        Box::new(Orientation::Vertical, 5),
        Box::new(Orientation::Vertical, 5),
    ];

    let button = [
        Button::with_label("Btn 1"),
        Button::with_label("Btn 2"),
        Button::with_label("Btn 3"),
    ];

    let drawing_area = DrawingArea::builder()
    .content_width(width - 5)
    .content_height(height - 5)
    .build();

    paint(&drawing_area);

    let label = Label::builder()
    .label("Status")
    .build();

    let label_rc = Rc::new(label);

    let label_clone = label_rc.clone();
    button[0].connect_clicked(move |_| on_clicked(&label_clone, 1));

    let label_clone = label_rc.clone();
    button[1].connect_clicked(move |_| on_clicked(&label_clone, 2));

    let label_clone = label_rc.clone();
    button[2].connect_clicked(move |_| on_clicked(&label_clone, 3));  

    hbox[0].append(&button[0]);
    hbox[0].append(&button[1]);
    hbox[0].append(&button[2]);
    hbox[1].append(&drawing_area);
    hbox[2].append(&*label_rc);

    vbox.append(&hbox[0]);
    vbox.append(&hbox[1]);
    vbox.append(&hbox[2]);

    let window = ApplicationWindow::builder()
    .title("Free Drawing")
    .default_width(width)
    .default_height(height)
    .application(app)
    .child(&vbox)
    .build();

    window.present();
}

fn paint(drawing_area: &gtk::DrawingArea) {
    drawing_area.set_draw_func(move |area, cr, _, _| {
        cr.set_source_rgb(1.0, 1.0, 1.0);
        cr.paint().unwrap();
    });
}

fn on_clicked(label: &Label, value: u32) {
    println!("Btn {}", value);
    label.set_label(&format!("Btn {}", value));
}
