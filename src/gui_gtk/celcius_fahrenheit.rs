/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          04-03-2023
    Titulo:         introducción a RUST y GTK3
    Descripción:    Conversión de Celsius a Fahrenheit
                    
    Referencias:
    https://doc.rust-lang.org/beta/rust-by-example/index.html
    https://www.jmgaguilera.com/rust_facil/actualizaciones.html
    https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/all.html

    https://docs.rs/gtk/0.16.2/gtk/
    https://github.com/gtk-rs/gtk3-rs

    Dependencias:
    GTK

    Compilar:

    Licencia:

***************************************************************************************/

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Label, Entry, Box};

struct Celsius(f32);
struct Fahrenheit(f32);

impl From<Celsius> for Fahrenheit {
    fn from(celsius: Celsius) -> Self {
        Fahrenheit(celsius.0 * 1.8 + 32.0)
    }
}

pub fn c_f() {
    let app = Application::new(
        Some("com.example.temperature-converter"),
        Default::default(),
    );

    app.connect_activate(|app| {
        // Create the main window
        let window = ApplicationWindow::new(app);
        window.set_title("Temperature Converter");
        window.set_default_size(350, 100);

        // Create a horizontal box to hold the input and output controls
        let hbox = Box::new(gtk::Orientation::Horizontal, 5);
        window.add(&hbox);

        // Create a label and an entry for the Celsius input
        let celsius_label = Label::new(Some("Celsius:"));
        let celsius_entry = Entry::new();
        hbox.add(&celsius_label);
        hbox.add(&celsius_entry);

        // Create a label for the Fahrenheit output
        let fahrenheit_label = Label::new(Some("Fahrenheit:"));
        let fahrenheit_output = Label::new(None);
        hbox.add(&fahrenheit_label);
        hbox.add(&fahrenheit_output);

        // Create a button to perform the conversion
        let convert_button = Button::with_label("Convert");
        hbox.add(&convert_button);

        // Connect the button to the conversion logic
        convert_button.connect_clicked(move |_| {
            let celsius_str = celsius_entry.text(); // unwrap_or_default();
            if let Ok(celsius_val) = celsius_str.parse::<f32>() {
                let celsius = Celsius(celsius_val);
                let fahrenheit: Fahrenheit = celsius.into();
                fahrenheit_output.set_text(&format!("{:.1} degrees Fahrenheit", fahrenheit.0));
            } else {
                fahrenheit_output.set_text("Invalid input");
            }
        });

        // Show the window and start the GTK main loop
        window.show_all();
    });

    app.run();
}
