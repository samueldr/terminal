use glib::{debug, error};
use gtk::{Application, gio, glib};
use std::ffi::OsString;
use vte4::prelude::*;

use samueldr_terminal_ii::misc::{G_LOG_DOMAIN, get_shell};
use samueldr_terminal_ii::terminal;

const APP_ID: &str = "com.samueldr.terminal";

const LOGGER: glib::GlibLogger = glib::GlibLogger::new(
    glib::GlibLoggerFormat::Plain,
    glib::GlibLoggerDomain::CrateTarget,
);

fn main() -> glib::ExitCode {
    log::set_logger(&LOGGER).expect("logger already set");
    log::set_max_level(log::LevelFilter::Debug);

    // The `TERMINAL_APPLICATION_ID` environment variable can be used for development purposes.
    // Otherwise the application ID of the "production" terminal that might be in use will be found
    // in the ambient environment, and will be used instead of a fresh dev build.
    let app = Application::builder()
        .application_id(std::env::var("TERMINAL_APPLICATION_ID").unwrap_or(APP_ID.into()))
        .flags(
            gio::ApplicationFlags::HANDLES_COMMAND_LINE | gio::ApplicationFlags::SEND_ENVIRONMENT,
        )
        .build();

    // We need the CWD from the remote instance...
    // https://developer.gnome.org/documentation/tutorials/application.html
    // ... otherwise new windows will be in the CWD from the initial start.
    app.connect_command_line(|app, cmdline| {
        debug!("Application ID: {:?}", app.application_id());

        // Clear this environment.
        // The VTE invocation will be the only place we manipulate the env.
        unsafe {
            std::env::vars().for_each(|(name, _)| std::env::remove_var(name));
        }

        // Sadly, we can't let GLib/gio handle the parameters for xterm compatibility.
        // There is no way to mark `-e` or `-x` as an argument that stops further processing.
        // The `--` delimiter can be used for that, but will be added to `cmdline.arguments()`.
        // So instead, we can "just" process the args ourselves.

        // Create a binding for args without argv[0].
        let args: Vec<OsString> = cmdline.arguments().into_iter().skip(1).collect();

        // Find out where we split the `-x` argv.
        // Fallback to "split" at the end, leaving an empty list.
        // We're keeping it as an `Option` so we can detect if the flag was given.
        let exec_index = args.iter().position(|arg| {
            arg == "-e" || // xterm
                    arg == "-x" // lilyterm and others
        });

        // Do the split.
        let (args, command_args) = args.split_at(exec_index.unwrap_or(args.len()));

        // Drop the `-e/-x` arg from the command arguments.
        let command_args: Vec<String> = command_args
            .iter()
            .skip(1)
            .cloned()
            .map(|s| {
                s.into_string()
                    .expect("unexpectedly coudln't handle argument.")
            })
            .collect();

        // Early check for a valid exec command line.
        if exec_index.is_some() && command_args.is_empty() {
            error!("No command given while command flag (-e/-x) was given.");
            return glib::ExitCode::FAILURE;
        }

        // Then handle params...
        for arg in args.iter() {
            // Though other than `-x` we don't handle any.
            let result = match arg.to_str().unwrap_or("") {
                "--help" | "-h" => {
                    println!("Sorry. No help for you.");
                    println!("Though `-x` works like with xterm.");
                    println!("That's it.");
                    Some(glib::ExitCode::SUCCESS)
                }
                _ => {
                    error!("Unexpected argument: {:?}", arg);
                    Some(glib::ExitCode::FAILURE)
                }
            };
            if let Some(exit) = result {
                return exit;
            }
        }

        // "Fall" back onto the user's login shell.
        let cmd: Vec<String> = if command_args.is_empty() {
            vec![get_shell(), "-".into()]
        } else {
            command_args
        };

        // Get the environment in a more idiomatic shape to shuttle around.
        let mut environ: Vec<String> = cmdline
            .environ()
            .into_iter()
            .map(|s| s.into_string().expect("unexpectedly couldn't handle env."))
            .collect();

        // The CWD comes from the remote instance.
        let cwd = cmdline.cwd().expect("unexpectedly got no cwd.");

        // Make sure `TERM` is valid for VTE.
        environ.push("TERM=screen-256color".into());

        debug!("Terminal command-line processing results:");
        debug!("  exec?: {:?}", exec_index.is_some());
        debug!("  args: {:?}", args);
        debug!("  command: {:?}", cmd);

        terminal::create(app, cwd, environ, cmd);

        glib::ExitCode::SUCCESS
    });

    app.run()
}
