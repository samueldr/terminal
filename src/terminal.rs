use crate::misc::{G_LOG_DOMAIN, get_shell};
use gdk::{Key, ModifierType, RGBA};
use glib::{clone, debug, error};
use gtk::{ApplicationWindow, EventControllerKey, gdk, gio, glib};
use pango::FontDescription;
use vte4::prelude::*;

const FONT_FAMILY: &str = "Go Mono";
const FONT_SIZE: i32 = 12;
const BRIGHTNESS: f32 = -0.137;
const SCALE_RATIO: f64 = 1.04;

fn make_color(s: &str) -> RGBA {
    let mut color = RGBA::parse(s).unwrap();
    color.set_red(color.red() * (1.0 + BRIGHTNESS));
    color.set_blue(color.blue() * (1.0 + BRIGHTNESS));
    color.set_green(color.green() * (1.0 + BRIGHTNESS));
    color
}

pub fn create(app: &gtk::Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("WIP terminal")
        .build();

    let mut font = FontDescription::new();
    font.set_family(FONT_FAMILY);
    font.set_size(pango::SCALE * FONT_SIZE);

    let term = vte4::Terminal::builder()
        .cursor_shape(vte4::CursorShape::Block)
        .cursor_blink_mode(vte4::CursorBlinkMode::On)
        .font_desc(&font)
        .build();

    let foreground = &RGBA::new(0.0, 0.0, 0.0, 1.0);
    let background = &RGBA::new(1.0, 1.0, 1.0, 1.0);

    // This is the Tango palette.
    let mut palette = [
        // NOTE: missing: "brightness" adjustment by -0.137 to each component.
        &make_color("#2e2e34343636"),
        &make_color("#cccc00000000"),
        &make_color("#4e4e9a9a0606"),
        &make_color("#c4c4a0a00000"),
        &make_color("#34346565a4a4"),
        &make_color("#757550507b7b"),
        &make_color("#060698209a9a"),
        &make_color("#d3d3d7d7cfcf"),
        &make_color("#555557575353"),
        &make_color("#efef29292929"),
        &make_color("#8a8ae2e23434"),
        &make_color("#fcfce9e94f4f"),
        &make_color("#72729f9fcfcf"),
        &make_color("#adad7f7fa8a8"),
        &make_color("#3434e2e2e2e2"),
        &make_color("#eeeeeeeeecec"),
    ];

    // Overwrite palette's "black" and "bright white" with background/foreground.
    palette[0] = background;
    palette[15] = foreground;

    term.set_colors(Some(foreground), Some(background), &palette);

    term.set_scrollback_lines(0);
    term.set_audible_bell(false);

    term.set_yalign(vte4::Align::Center);
    term.set_xalign(vte4::Align::Center);
    term.set_yfill(true);
    term.set_xfill(true);

    window.set_child(Some(&term));

    window.present();

    let keyboard = EventControllerKey::new();
    keyboard.set_propagation_phase(gtk::PropagationPhase::Capture);

    {
        let term = term.clone();
        keyboard.connect_key_pressed(move |_, key, _keycode, modifiers| {
            let k_ctrl = modifiers.contains(ModifierType::CONTROL_MASK);
            let k_shift = modifiers.contains(ModifierType::SHIFT_MASK);

            match (k_ctrl, k_shift, key) {
                (true, false, Key::Return) => {
                    term.set_font_scale(1.0);
                    return glib::Propagation::Stop;
                }
                (true, _, Key::minus | Key::underscore) | (true, _, Key::equal | Key::plus) => {
                    let mut ratio = if k_shift {
                        (SCALE_RATIO - 1.0) / 2.0 + 1.0
                    } else {
                        SCALE_RATIO
                    };
                    match key {
                        Key::minus | Key::underscore => {
                            ratio = 1.0 / ratio;
                        }
                        _ => {}
                    };
                    term.set_font_scale(term.font_scale() * ratio);
                    return glib::Propagation::Stop;
                }
                (true, true, Key::C) => {
                    term.copy_clipboard_format(vte4::Format::Text);
                    return glib::Propagation::Stop;
                }
                (true, true, Key::V) => {
                    term.paste_clipboard();
                    return glib::Propagation::Stop;
                }
                _ => {}
            }

            glib::Propagation::Proceed
        });
    }

    window.add_controller(keyboard);

    // https://gnome.pages.gitlab.gnome.org/vte/gtk4/method.Terminal.spawn_async.html
    term.spawn_async(
        vte4::PtyFlags::DEFAULT,
        // Use CWD implicitly.
        None,
        &[get_shell().as_ref(), "-"],
        &[],
        // https://docs.gtk.org/glib/flags.SpawnFlags.html
        glib::SpawnFlags::DEFAULT,
        || (), // child_setup
        -1,    // timeout
        gio::Cancellable::NONE,
        clone!(
            #[weak]
            window,
            move |result| {
                match result {
                    Ok(pid) => {
                        debug!("Terminal process PID: {:?}", pid);
                    }
                    Err(e) => {
                        error!("Failed to spawn process...");
                        error!("{:?}", e);
                        window.close();
                        // TODO: report this through the terminal window and wait for action?
                        // I'd need a "framework" around that, if desirable.
                    }
                }
            }
        ),
    );

    term.connect_child_exited(clone!(
        #[weak]
        window,
        move |term, status| {
            debug!("Terminal child process exited...");
            debug!("  Term: {:?}", term);
            debug!("  Exited with status: {:?}", status);
            window.close();
        }
    ));
}
