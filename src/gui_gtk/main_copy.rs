/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          22-04-2023
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

pub fn maincopy() {
    // Inicializa GTK 
    // gtk::init().expect("Falló la inicialización de Gtk");
    if gtk::init().is_err() {
        println!("Error: No se pudo inicializar GTK.");
        return;
    }

    // Crea una ventana
    let ventana = gtk::Window::new(gtk::WindowType::Toplevel);
    // Establece el título de la ventana
    ventana.set_title("Hex de una dirección de memoria");
    // Establece el tamaño de la ventana
    ventana.set_default_size(640, 480);

    let boton_ventana1 = gtk::Button::with_label("Abre nueva ventana");
    boton_ventana1.connect_clicked(move |_| {
        let ventana1 = gtk::Window::new(gtk::WindowType::Toplevel);
        ventana1.set_title("Second Window");
        ventana1.set_default_size(200, 200);
        ventana1.show_all();
    });

    let memory_value = 0xffff;
    let memory_address = &memory_value as *const i32;
    println!("Value: {}",memory_value);
    println!("Address: 0x{:x}", memory_address as usize); 
    let etiqueta = gtk::Label::new(Some(&format!("Código en hex en dirección: {:?}, es: {:X?}", memory_address, memory_value)));

    // Crea cajas
    let vcaja = gtk::Box::new(gtk::Orientation::Vertical, 2);
    let vcaja1 = gtk::Box::new(gtk::Orientation::Vertical, 5);
    let hcaja1 = gtk::Box::new(gtk::Orientation::Horizontal, 5);

    // Crea vistas de texto
    let contenido_mem = gtk::TextView::new();
    let contenido_bufer = gtk::TextView::new();

    // Crea Búfer de texto
    let bufer1 = contenido_bufer.buffer().expect("Error al obtener el búfer1");
    //bufer1.set_text("Texto en el búfer1");
    let bufer2 = contenido_bufer.buffer().expect("Error al obtener el búfer2");
    //bufer2.set_text("Texto en el búfer2");


    // Agrega widget
    ventana.add(&vcaja);
    // vcaja.add(&etiqueta);
    vcaja.pack_start(&etiqueta, false, true, 10);
    vcaja.pack_start(&contenido_mem, false, true, 0);
    vcaja.pack_start(&vcaja1, false, true, 10);
    vcaja1.pack_start(&contenido_bufer, false, true, 10);
    vcaja1.pack_end(&hcaja1, false, false, 0);
    
    // Crea botones
    let boton_bufer1 = gtk::Button::with_label("Cambiar a Buffer 1");
    hcaja1.pack_start(&boton_bufer1, false, true, 0);
    let boton_bufer2 = gtk::Button::with_label("Cambiar a Buffer 2");
    hcaja1.pack_end(&boton_bufer2, false, true, 0);

    let boton_ok = gtk::Button::with_label("OK");
    // vcaja.pack_end(&boton_ok, false, false, 0);
    vcaja.add(&boton_ok);
    vcaja.add(&boton_ventana1);

    let boton_salir = gtk::Button::with_label("Salir");
    vcaja.pack_end(&boton_salir, false, false, 2);

    // Conecta las señales "clicked" al botones
    boton_salir.connect_clicked(|_| { gtk::main_quit(); });
        
    boton_ok.connect_clicked(move |_| {
        let contenido_mem_str = get_contenido_mem();
        contenido_mem.buffer().unwrap().set_text(&contenido_mem_str); 
                   //^ .get_buffer().unwrap().set_text(&contenido_mem_str);
    });

    boton_bufer1.connect_clicked(move |_| { bufer1.set_text("Texto en el búfer1"); });
    boton_bufer2.connect_clicked(move |_| { bufer2.set_text("Texto en el búfer2"); });
    
    // Muestra todos los widgets contenidos en la ventana
    ventana.show_all();

    // Se conecta la señal "delete-event" a la ventana
    ventana.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}


fn get_contenido_mem() -> String {

    let mut memory: Vec<u32> = vec![0; 32];

    memory[0] = 10;
    memory[1] = 20;
    memory[2] = 30;
    memory[3] = 40;

    for i in 0..4 {
        println!("memory[{}] = {}, ptr = {:?}", i, memory[i], memory.as_ptr_range());
    };

    // Here, you can get the contents of the memory
    return String::from("Este es el contenido de la memoria.")
}