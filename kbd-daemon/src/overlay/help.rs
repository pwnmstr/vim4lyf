use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Label, CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION};

pub fn show_help_popup(app: &Application) {
    let window = ApplicationWindow::new(app);
    window.set_title(Some("kbd-daemon Help"));
    window.set_default_size(400, 300);

    let provider = CssProvider::new();
    provider.load_from_data(
        "window { background-color: #282a36; color: #f8f8f2; padding: 20px; }
         label { font-family: monospace; font-size: 14px; margin-bottom: 10px; }"
    );
    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().unwrap(),
        &provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let help_text = "
KBD-DAEMON COMMANDS

trigger-scan [region]
  Scan the screen and show labels.
  Regions: all, top-left, top-right, bottom-left, bottom-right

VIM-LIKE FEATURES (in Scan Mode)
  <number>
    Repeat the next action N times.
    Example: '5asd' clicks 'asd' 5 times.

  q <register>
    Start recording a macro. Then type a label.
    The action bound to that label is saved.
    Example: 'qa asd' saves the action at 'asd' to register 'a'.

  @ <register>
    Execute a saved macro.
    Example: '@a' executes register 'a'.
    You can combine with counts: '5@a' executes 'a' 5 times.

Escape
  Close the overlay without doing anything.
";

    let label = Label::builder()
        .label(help_text)
        .justify(gtk4::Justification::Left)
        .build();

    window.set_child(Some(&label));
    window.present();
}
