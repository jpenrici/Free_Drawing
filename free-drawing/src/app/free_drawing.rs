use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use gtk::{DrawingArea, Button, Label, Box, Orientation, ColorChooserDialog};
use gtk::{GestureClick, EventControllerMotion};

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
struct Pen {
    positions: Vec<(f64, f64)>,
    colors: Vec<(f32, f32, f32)>, // RGB values (0.0 - 1.0)
    current_color: (f32, f32, f32),
    drawing: bool,
}

pub fn start() {
    let app = Application::builder()
        .application_id("app.example.free_drawing.com")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let width = 600;
    let height = 400;

    let vbox = Box::new(Orientation::Vertical, 5);
    let button_box = Box::new(Orientation::Horizontal, 5);
    let drawing_box = Box::new(Orientation::Vertical, 5);
    let status_box = Box::new(Orientation::Vertical, 5);

    let button_color = Button::with_label("Color");
    let button_clear = Button::with_label("Clear");

    let drawing_area = DrawingArea::builder()
        .content_width(width - 5)
        .content_height(height - 5)
        .build();

    let label = Label::builder().label("Status").build();
    let label_rc = Rc::new(label);

    let state = Rc::new(RefCell::new(Pen::default()));
    
    setup_color_button(&button_color, state.clone());
    setup_clear_button(&button_clear, &drawing_area, state.clone());

    button_box.append(&button_color);
    button_box.append(&button_clear);
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

    setup_drawing(&drawing_area, &label_rc, state);
    window.present();
}

fn setup_drawing(drawing_area: &DrawingArea, label: &Rc<Label>, state: Rc<RefCell<Pen>>) {
    setup_mouse_click(drawing_area, state.clone());
    setup_mouse_motion(drawing_area, label, state.clone());
    setup_draw_function(drawing_area, state);
}

fn setup_mouse_click(drawing_area: &DrawingArea, state: Rc<RefCell<Pen>>) {
    let mouse_click = GestureClick::new();
    mouse_click.set_button(1);

    let state_pressed = state.clone();
    mouse_click.connect_pressed(move |_, _, x, y| {
        let mut state = state_pressed.borrow_mut();
        let current_color_temp = state.current_color;  // Ensure memory security.
        state.positions.push((x, y));
        state.colors.push(current_color_temp);
        state.drawing = true;
    });

    let state_released = state;
    mouse_click.connect_released(move |_, _, _, _| {
        let mut state = state_released.borrow_mut();
        state.drawing = false;
    });

    drawing_area.add_controller(mouse_click);
}

fn setup_mouse_motion(drawing_area: &DrawingArea, label: &Rc<Label>, state: Rc<RefCell<Pen>>) {
    let motion = EventControllerMotion::new();
    
    let state_clone = state;
    let drawing_area_clone = drawing_area.clone();
    let label_clone = label.clone();

    motion.connect_motion(move |_, x, y| {
        let mut state = state_clone.borrow_mut();
        let current_color_temp = state.current_color;
        if state.drawing {
            state.positions.push((x, y));
            state.colors.push(current_color_temp);
            drawing_area_clone.queue_draw();
        }
        label_clone.set_label(&format!("Mouse: ({:.2}, {:.2})", x, y));
    });

    drawing_area.add_controller(motion);
}

fn setup_draw_function(drawing_area: &DrawingArea, state: Rc<RefCell<Pen>>) {
    drawing_area.set_draw_func(move |_area, cr, _width, _height| {
        // Desenhar fundo branco
        cr.set_source_rgb(1.0, 1.0, 1.0);
        cr.paint().unwrap();

        let state = state.borrow();
        if state.positions.is_empty() {
            return;
        }

        cr.set_line_width(2.0);

        // Desenhar cada segmento com sua cor específica
        for i in 1..state.positions.len() {
            let (x1, y1) = state.positions[i-1];
            let (x2, y2) = state.positions[i];
            let (r, g, b) = state.colors[i];
            
            cr.set_source_rgb(r as f64, g as f64, b as f64);
            cr.move_to(x1, y1);
            cr.line_to(x2, y2);
            cr.stroke().unwrap();
        }
    });
}

fn setup_color_button(button: &Button, state: Rc<RefCell<Pen>>) {
    button.connect_clicked(move |btn| {
        let window = btn.root().and_downcast::<gtk::Window>().unwrap();
        let dialog = ColorChooserDialog::new(Some("Choose Pen Color"), Some(&window));
        
        let state = state.clone();
        dialog.connect_response(move |dialog, response| {
            if response == gtk::ResponseType::Ok {
                let rgba = dialog.rgba();
                let mut state = state.borrow_mut();
                state.current_color = (rgba.red(), rgba.green(), rgba.blue());
            }
            dialog.close();
        });

        dialog.show();
    });
}

fn setup_clear_button(button: &Button, drawing_area: &DrawingArea, state: Rc<RefCell<Pen>>) {
    let drawing_area = drawing_area.clone();
    button.connect_clicked(move |_| {
        let mut state = state.borrow_mut();
        state.positions.clear();
        state.colors.clear();
        drawing_area.queue_draw();
    });
}