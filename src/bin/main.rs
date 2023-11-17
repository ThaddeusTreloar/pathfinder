use gtk4::prelude::*;
use gtk4::{glib::ExitCode, Application, ApplicationWindow};
use pathfinder::prelude::*;

fn main() -> ExitCode {
    let app = Application::builder()
        .application_id("org.thaddeustreloar.Pathfinder")
        .build();

    app.connect_activate(|app| {
        // We create the main window.
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("Hello, World!")
            .build();

        // Show the window.
        window.present();
    });

    app.run()
}
