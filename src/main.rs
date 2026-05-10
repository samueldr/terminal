use gtk::{Application, gio, glib};
use std::path::PathBuf;
use vte4::prelude::*;

mod misc;
mod terminal;

const APP_ID: &str = "com.samueldr.terminal";

const LOGGER: glib::GlibLogger = glib::GlibLogger::new(
    glib::GlibLoggerFormat::Plain,
    glib::GlibLoggerDomain::CrateTarget,
);

fn start(app: &gtk::Application, cwd: PathBuf) {
    terminal::create(app, cwd);
}

fn main() -> glib::ExitCode {
    log::set_logger(&LOGGER).expect("logger already set");
    log::set_max_level(log::LevelFilter::Debug);

    let app = Application::builder()
        .application_id(APP_ID)
        .flags(gio::ApplicationFlags::HANDLES_COMMAND_LINE)
        .build();

    // // We need the CWD from the remote instance...
    // // https://developer.gnome.org/documentation/tutorials/application.html
    // // ... otherwise new windows will be in the CWD from the initial start.
    app.connect_command_line(|app, cmdline| {
        start(app, cmdline.cwd().expect("unexpectedly got no cwd."));

        glib::ExitCode::new(89)
    });

    app.run();

    glib::ExitCode::SUCCESS
}
