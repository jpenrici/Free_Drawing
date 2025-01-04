use gtk::{prelude::*, EventControllerMotion};
use gtk::{Application, ApplicationWindow};
use gtk::{DrawingArea, Button, Label, Box, Orientation};
use gtk::GestureClick;

use std::cell::RefCell;
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

    let button_box = Box::new(Orientation::Horizontal, 5);
    let drawing_box = Box::new(Orientation::Vertical, 5);
    let status_box = Box::new(Orientation::Vertical, 5);

    let button = [
        Button::with_label("Btn 1"),
        Button::with_label("Btn 2"),
        Button::with_label("Btn 3"),
    ];

    let drawing_area = DrawingArea::builder()
    .content_width(width - 5)
    .content_height(height - 5)
    .build();

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

    button_box.append(&button[0]);
    button_box.append(&button[1]);
    button_box.append(&button[2]);
    drawing_box.append(&drawing_area);
    status_box.append(&*label_rc);

    vbox.append(&button_box);
    vbox.append(&drawing_box);
    vbox.append(&status_box);

    let window = ApplicationWindow::builder()
    .title("Free Drawing")
    .default_width(width)
    .default_height(height)
    .application(app)
    .child(&vbox)
    .build();

    setup_drawing(&drawing_area);

    window.present();
}

#[derive(Default)]
struct Pen {
    positions: Vec<(f64, f64)>,
    drawing: bool,
}

fn setup_drawing(drawing_area: &gtk::DrawingArea) {
    let state = Rc::new(RefCell::new(Pen::default()));

    let mouse_click = GestureClick::new();
    mouse_click.set_button(1);  // Left Mouse Button

    let state_clone = state.clone();
    mouse_click.connect_pressed(move |_, _, x, y| {
        let mut state = state_clone.borrow_mut();
        state.drawing = true;
        state.positions.push((x, y));
    });
 
    let state_clone = state.clone();
    mouse_click.connect_released(move |_, _, _, _|{
        let mut state = state_clone.borrow_mut();
        state.drawing = false;
    });

    drawing_area.add_controller(mouse_click);

    let mouse_position = EventControllerMotion::new();
    let state_clone = state.clone();
    let drawing_area_clone = drawing_area.clone();

    mouse_position.connect_motion(move |_, x, y|{
        let mut state = state_clone.borrow_mut();
        if state.drawing {
            state.positions.push((x, y));
            drawing_area_clone.queue_draw();
        }
    });

    drawing_area.add_controller(mouse_position);

    let state_clone = state.clone();
    drawing_area.set_draw_func(move |_area, cr, _width, _height| {
        cr.set_source_rgb(1.0, 1.0, 1.0);
        cr.paint().unwrap();

        cr.set_source_rgb(0.0, 0.0, 0.0);
        cr.set_line_width(2.0);

        let state = state_clone.borrow();
        if let Some((first_x, first_y)) = state.positions.first() {
            cr.move_to(*first_x, *first_y);
            
            for &(x, y) in state.positions.iter().skip(1) {
                cr.line_to(x, y);
            }
            
            cr.stroke().unwrap();
        }
    });
}

fn on_clicked(label: &Label, value: u32) {
    println!("Btn {}", value);
    label.set_label(&format!("Btn {}", value));
}
