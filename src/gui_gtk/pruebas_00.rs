/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          01-05-2023
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
use gtk::{Application, ApplicationWindow, Button, TextView, Window, WindowType};
use gdk::{EventMask};

//***************************************************************************** 
pub fn gtk_prueba_00() {
    let app = Application::new(
        Some("com.example.gtk-rs-textview"),
        Default::default(),
    );

    app.connect_activate(|app| {
        // Crea ventana principal
        let window = ApplicationWindow::new(app);
        window.set_title("Ejemplo TextView con Botones");
        window.set_default_size(350, 70);

        // Crea el TextView
        let textview = TextView::new();
        let buffer = textview.buffer().unwrap();
        buffer.set_text("Texto de ejemplo");

        // Crea los botones
        let button1 = Button::with_label("Botón 1");
        let button2 = Button::with_label("Botón 2");

        // Conecta las señales "clicked" de los botones al callback
        let buffer_clone1 = buffer.clone();
        button1.connect_clicked(move |_| { buffer_clone1.set_text("Contenido actualizado por Botón 1"); });
        let buffer_clone2 = buffer.clone();
        button2.connect_clicked(move |_| { buffer_clone2.set_text("Contenido actualizado por Botón 2"); });


    //*************************************
        // Agrega máscara de evento para recibir eventos
        window.add_events(EventMask::BUTTON_PRESS_MASK);
        button2.add_events(EventMask::ENTER_NOTIFY_MASK);

        // Conecta una señal para manejar eventos de pulsación de botón
        window.connect_event(move |_, event| {
            match event.event_type() {
                gdk::EventType::ButtonPress => {
                    println!("presionaste en la ventana!");
                    println!("Coordenadas: ({:?},{:?})", event.button(), event.button());
                },
                gdk::EventType::EnterNotify => {
                    println!("¡Mouse entra en la ventana!");
                },
                _ => (),
            }
            Inhibit(false)
        });


    //*************************************
        // Conecta con key-press-event
        let buffer_clon_tecla0 = buffer.clone();
        window.connect_key_press_event(move |_, key| {
            println!("Tecla presionada: {}", key.keyval());
            buffer_clon_tecla0.set_text(&format!("Tecla soltada {:?}", key.keyval()));
            Inhibit(false)
        });

        // Conecta con key-release-event
        let buffer_clon_tecla0 = buffer.clone();
        window.connect_key_release_event(move |_, key| {
            println!("Tecla soltada   : {}", key.keyval());
            buffer_clon_tecla0.set_text(&format!("Tecla soltada {:?}", key.keycode()));
            Inhibit(false)
        });


    //*************************************
        // Conectamos la función al evento de entrada del ratón
        button1.connect_event(move |_, event| {
            if let gdk::EventType::EnterNotify = event.event_type() {
                println!("¡Mouse entra en bóton 1!");
                Inhibit(false)
            } else {
                gtk::Inhibit(false)
            }
        });
        
        // Conectamos la función al evento de salida del ratón
        button1.connect_event(move |_, event| {
            if let gdk::EventType::LeaveNotify = event.event_type() {
                println!("¡Mouse sale del bóton 1!");
                Inhibit(false)
            } else {
                gtk::Inhibit(false)
            }
        });
    

        // Crear una caja vertical para alinear los elementos
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
        vbox.pack_start(&button1, false, false, 8);
        vbox.pack_start(&button2, false, false, 8);
        vbox.pack_start(&textview, true, true, 0);

        // Agregar la caja vertical a la ventana
        window.add(&vbox);

        // Mostrar todo
        window.show_all();
    });

    app.run();
}

//***************************************************************************** 
fn on_mouse_enter(_window: &gtk::Window, _event: &gdk::Event) -> Inhibit {
    // Manejar el evento de ratón sobre la ventana
    println!("¡Mouse entra en la ventana!");
    Inhibit(false)
}

pub fn gtk_prueba_01() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Ejemplos GdkEvent");
    window.set_default_size(350, 70);

    let button = Button::with_label("¡Pulsa!");
    window.add(&button);

    // Conecta una señal para manejar los clics de los botones
    button.connect_clicked(move |_| { println!("Botón pulsado"); });

    // Agrega máscara de evento para recibir eventos
    window.add_events(EventMask::BUTTON_PRESS_MASK);
    window.add_events(EventMask::ENTER_NOTIFY_MASK);

    /*
    |win, event|: esto define los argumentos de la clausura. En este caso, la clausura espera dos argumentos:
    win, que es una referencia al objeto de ventana de la señal, y event, que es una referencia al objeto de
    evento emitido por la señal.

    win.downcast_ref().unwrap(): esto convierte win de &gtk::Widget a &gtk::Window. La función downcast_ref
    intenta convertir win a un tipo más específico (&gtk::Window en este caso) si es posible. Si la conversión
    tiene éxito, devuelve una referencia a ese tipo más específico, envuelto en Some(). Si la conversión no es
    posible, devuelve None. En este caso, sabemos que win es un objeto de tipo gtk::Window (ya que la señal se
    conecta a un objeto de tipo gtk::Window), por lo que podemos usar unwrap() para extraer la referencia
    &gtk::Window envuelta en Some(). Si downcast_ref falla (por ejemplo, si se conecta la señal a un objeto de
    tipo gtk::Button, que no es un gtk::Window), entonces unwrap() producirá un error en tiempo de ejecución.

    event: esto simplemente pasa el argumento event a la función on_mouse_enter.
    */
    window.connect_enter_notify_event(|win, event| on_mouse_enter(win.downcast_ref().unwrap(), event));



    // Conecta una señal para manejar eventos de pulsación de botón
    window.connect_event(move |_, event| {
        match event.event_type() {
            gdk::EventType::ButtonPress => {
                println!("Button pressed!");
                println!("Button press coordinates: ({:?},{:?})", event.button(), event.button());
            },
            gdk::EventType::EnterNotify => {
                println!("¡Mouse entra!");
            },
            _ => (),
        }
        Inhibit(false)
    });


    // Conectamos la función al evento de salida del ratón
    window.connect_event(move |_, event| {
        if let gdk::EventType::LeaveNotify = event.event_type() {
            println!("¡Mouse sale!");
            Inhibit(true)
        } else {
            gtk::Inhibit(false)
        }
    });



    window.show_all();

    // Se conecta la señal "delete-event" a la ventana
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}

//***************************************************************************** 