/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          29-04-2023
    Titulo:         introducción a RUST y GTK3
    Descripción:    
                    
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
//#![allow(unused_imports)]

//Se necesita el siguiente cargo para usar los widgets de GTK
use gtk::{prelude::*};
use gtk::{Application, ApplicationWindow, Button, TextView};

pub fn gtk_prueba_00() {
    let app = Application::new(
        Some("com.example.gtk-rs-textview"),
        Default::default(),
    );

    app.connect_activate(|app| {
        // Crear ventana principal
        let window = ApplicationWindow::new(app);
        window.set_title("Ejemplo TextView con Botones");
        window.set_default_size(350, 70);

        // Crear el TextView
        let textview = TextView::new();
        let buffer = textview.buffer().unwrap();
        buffer.set_text("Texto de ejemplo");

        // Crear los botones
        let button1 = Button::with_label("Botón 1");
        let button2 = Button::with_label("Botón 2");

        // Conectar las señales "clicked" de los botones al callback
        let buffer_clone = buffer.clone();
        button1.connect_clicked(move |_| {
            buffer_clone.set_text("Contenido actualizado por Botón 1");
        });

        let buffer_clone2 = buffer.clone();
        button2.connect_clicked(move |_| {
            buffer_clone2.set_text("Contenido actualizado por Botón 2");
        });

        // Crear una caja vertical para alinear los elementos
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
        vbox.pack_start(&button1, false, false, 0);
        vbox.pack_start(&button2, false, false, 0);
        vbox.pack_start(&textview, true, true, 0);

        // Agregar la caja vertical a la ventana
        window.add(&vbox);

        // Mostrar todo
        window.show_all();
    });

    app.run();
}

/*
¿Como podemos en gtk-rs usar dos Button distintos para cambiar el contenido de un TextBuffer y por medio de el un TextView según el botón que se pulse?

Para usar dos botones distintos en Gtk-rs para cambiar el contenido de un TextBuffer y actualizar un TextView, puedes seguir los siguientes pasos:
Crea un TextBuffer y un TextView:

let buffer = TextBuffer::new(None);
let textview = TextView::new_with_buffer(&buffer);

Crea dos botones y añade un manejador de señales connect_clicked a cada uno de ellos para que se llame a una función diferente cuando se haga clic en ellos:

let button1 = Button::with_label("Botón 1");
button1.connect_clicked(clone!(@strong buffer, @strong textview => move |_| {
    cambiar_contenido_buffer(&buffer, "Contenido del botón 1");
    actualizar_textview(&textview, &buffer);
}));

let button2 = Button::with_label("Botón 2");
button2.connect_clicked(clone!(@strong buffer, @strong textview => move |_| {
    cambiar_contenido_buffer(&buffer, "Contenido del botón 2");
    actualizar_textview(&textview, &buffer);
}));

En este ejemplo, las funciones cambiar_contenido_buffer y actualizar_textview son funciones personalizadas que actualizan el contenido del TextBuffer y el TextView, respectivamente.
Define las funciones personalizadas cambiar_contenido_buffer y actualizar_textview:

fn cambiar_contenido_buffer(buffer: &TextBuffer, contenido: &str) {
    buffer.set_text(contenido);
}

fn actualizar_textview(textview: &TextView, buffer: &TextBuffer) {
    textview.set_buffer(Some(buffer));
}

En la función cambiar_contenido_buffer, se cambia el contenido del TextBuffer a la cadena especificada. En la función actualizar_textview, se actualiza el TextView con el TextBuffer proporcionado.
Agrega los botones y el TextView a una ventana y muestra la ventana:

let window = Window::new(WindowType::Toplevel);
window.add(&button1);
window.add(&button2);
window.add(&textview);
window.show_all();
*/