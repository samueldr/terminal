use gtk::{Application, gio, glib};
use vte4::prelude::*;

use samueldr_terminal_ii::terminal;

const APP_ID: &str = "com.samueldr.terminal";

const LOGGER: glib::GlibLogger = glib::GlibLogger::new(
    glib::GlibLoggerFormat::Plain,
    glib::GlibLoggerDomain::CrateTarget,
);

fn main() -> glib::ExitCode {
    log::set_logger(&LOGGER).expect("logger already set");
    log::set_max_level(log::LevelFilter::Debug);

    let app = Application::builder()
        .application_id(APP_ID)
        .flags(
            gio::ApplicationFlags::HANDLES_COMMAND_LINE | gio::ApplicationFlags::SEND_ENVIRONMENT,
        )
        .build();

    // We need the CWD from the remote instance...
    // https://developer.gnome.org/documentation/tutorials/application.html
    // ... otherwise new windows will be in the CWD from the initial start.
    app.connect_command_line(|app, cmdline| {
        // Clear this environment.
        // The VTE invocation will be the only place we manipulate the env.
        unsafe {
            std::env::vars().for_each(|(name, _)| std::env::remove_var(name));
        }

        // Get the environment in a more idiomatic shape to shuttle around.
        let mut environ: Vec<String> = cmdline
            .environ()
            .into_iter()
            .map(|s| s.into_string().expect("unexpectedly couldn't handle env."))
            .collect();

        // Make sure `TERM` is valid for VTE.
        environ.push("TERM=screen-256color".into());

        terminal::create(
            app,
            cmdline.cwd().expect("unexpectedly got no cwd."),
            environ,
        );

        glib::ExitCode::SUCCESS
    });

    app.run()
}
