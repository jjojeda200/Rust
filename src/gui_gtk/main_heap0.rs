/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          20-04-2023
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

use gtk::prelude::*;
use gtk::{ButtonsType, DialogFlags, MessageDialog, MessageType};

// Define una estructura para representar un elemento en la pila
#[derive(Debug)]
struct Element {
    value: i32,
}

// Define una estructura para representar la pila
struct Heap {
    elements: Vec<Element>,
}

impl Heap {
    // Función para agregar un elemento a la pila
    fn push(&mut self, element: Element) {
        self.elements.push(element);
    }

    // Función para eliminar el último elemento de la pila
    fn pop(&mut self) -> Option<Element> {
        self.elements.pop()
    }
}

pub fn xx() {
    // Inicializa GTK
    if gtk::init().is_err() {
        println!("Falló al inicializar GTK");
        return;
    }

    // Crea una ventana de diálogo para mostrar mensajes
    let dialog = MessageDialog::new(
        None::<&gtk::Window>,
        DialogFlags::empty(),
        MessageType::Info,
        ButtonsType::Ok,
        "",
    );

    // Crea una pila vacía
    let mut heap = Heap { elements: vec![] };

    // Agrega elementos a la pila
    heap.push(Element { value: 10 });
    heap.push(Element { value: 20 });
    heap.push(Element { value: 30 });

    // Muestra los elementos en la pila
    println!("Elementos en la pila antes de pop:");
    for element in &heap.elements {
        println!("{:?}", element);
    }

    // Elimina el último elemento de la pila
    let popped_element = heap.pop();

    // Muestra los elementos en la pila después de pop
    println!("Elementos en la pila después de pop:");
    for element in &heap.elements {
        println!("{:?}", element);
    }

    // Muestra el elemento eliminado
    match popped_element {
        Some(element) => {
            println!("Elemento eliminado: {:?}", element);
            dialog.set_markup(&format!("Elemento eliminado: {:?}", element));
        }
        None => {
            println!("La pila está vacía");
            dialog.set_markup("La pila está vacía");
        }
    }

    // Muestra el mensaje en la ventana de diálogo
    dialog.run();
    unsafe {dialog.destroy();}
    
}