use gtk::prelude::*;

pub fn start() {
    let app = gtk::Application::builder()
    .application_id("app.example.free_drawing.com")
    .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &gtk::Application) {

    let vbox =  gtk::Box::new(gtk::Orientation::Vertical, 0);

    let hbox = [
        gtk::Box::new(gtk::Orientation::Vertical, 0),
        gtk::Box::new(gtk::Orientation::Vertical, 0),
        gtk::Box::new(gtk::Orientation::Vertical, 0),
    ];

    let button = [
        gtk::Button::with_label("Ops1"),
        gtk::Button::with_label("Ops1"),
        gtk::Button::with_label("Ops1"),
    ];

    hbox[0].append(&button[0]);
    hbox[1].append(&button[1]);
    hbox[2].append(&button[2]);

    vbox.append(&hbox[0]);
    vbox.append(&hbox[1]);
    vbox.append(&hbox[2]);

    let window = gtk::ApplicationWindow::builder()
    .title("Free Drawing")
    .application(app)
    .child(&vbox)
    .build();

    window.present();
}
