/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          20-04-2023
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

***************************************************************************************/
#![allow(dead_code)]

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
        Some("com.convertidor-temperaturas"),
        Default::default(),
    );

    app.connect_activate(|app| {
        // Crea ventana
        let window = ApplicationWindow::new(app);
        window.set_title("Convertidor temperaturas");
        window.set_default_size(50, 20);

        // Cree un cuadro horizontal para contener los controles de entrada y salida
        let hbox = Box::new(gtk::Orientation::Horizontal, 8);

        // Crea una etiqueta y una entrada para la entrada Celsius
        let celsius_label = Label::new(Some("Celsius:"));
        celsius_label.set_hexpand(false);   // no se expande horizontalmente
        celsius_label.set_vexpand(false);   // no se expande verticalmente
        let celsius_entry = Entry::new();
        celsius_entry.set_max_length(4);
        celsius_entry.set_expand(false);
        celsius_entry.set_hexpand(false);   // no se expande horizontalmente
        celsius_entry.set_vexpand(false);   // no se expande verticalmente

        window.add(&hbox);
        hbox.pack_start(&celsius_label, false, false, 10);
        hbox.pack_start(&celsius_entry, false, false, 10);
        //hbox.add(&celsius_label);
        //hbox.add(&celsius_entry);

        // Crea una etiqueta para la salida Fahrenheit
        let fahrenheit_label = Label::new(Some("Fahrenheit:"));
        let fahrenheit_output = Label::new(None);
        fahrenheit_output.set_width_chars(12);
        hbox.pack_start(&fahrenheit_label, false, false, 10);
        hbox.pack_start(&fahrenheit_output, false, false, 10);
        //hbox.add(&fahrenheit_label);
        //hbox.add(&fahrenheit_output);

        // Crear un botón para realizar la conversión
        let convert_button = Button::with_label("Calcula");
        hbox.add(&convert_button);

        // Conectar el botón a la lógica de conversión
        convert_button.connect_clicked(move |_| {
            let celsius_str = celsius_entry.text(); //unwrap_or_default();
            if let Ok(celsius_val) = celsius_str.parse::<f32>() {
                let celsius = Celsius(celsius_val);
                let fahrenheit: Fahrenheit = celsius.into();
                fahrenheit_output.set_text(&format!("{:.1} grados", fahrenheit.0));
            } else {
                fahrenheit_output.set_text("Entrada invalida");
            }
        });
        window.show_all();
    });

    app.run();
}
