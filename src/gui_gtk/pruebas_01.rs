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
use gtk::prelude::*;
use gtk::{Window, WindowType, Button};
use gdk::{EventMask};

fn on_mouse_enter(_window: &gtk::Window, _event: &gdk::Event) -> Inhibit {
    // Manejar el evento de ratón sobre la ventana
    println!("¡Mouse entra en la ventana!");
    Inhibit(false)
}


pub fn gtk_prueba_00() {
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

