/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          26-03-2023
    Titulo:         introducción a RUST
    Descripción:    Jugando con la memoria, punteros, referencias, etc.
    Referencias:
    Rust Programming Language
                https://doc.rust-lang.org/stable/book/
    Rust Reference
                https://doc.rust-lang.org/reference/introduction.html
    Rust by examples
                https://doc.rust-lang.org/beta/rust-by-example/index.html
    Recetas de Rust Cookbook
                https://rust-lang-nursery.github.io/rust-cookbook/
    El Lenguaje de Programación Rust
                https://github.com/goyox86/elpr-sources
    Rust en español fácil
                https://www.jmgaguilera.com/rust_facil/actualizaciones.html
    Tour de Rust
                https://tourofrust.com/TOC_es.html
    Crate std   https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/std/index.html
                https://doc.rust-lang.org/std/index.html
    Crate gtk   https://gtk-rs.org/gtk3-rs/git/docs/gtk/index.html

***************************************************************************************/
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::alloc::{alloc, dealloc, Layout};

fn imprime_titulo(titulo: &String) {
    println!("\n{:*^80}", titulo);
}

//***************************************************************************** Deref y size_of
use std::ops::Deref;

pub fn memoria_deref() {
    let titulo = String::from(" Deref (des referenciar) ");
    imprime_titulo(&titulo);

    let number = 5;
    let number_pointer = &number;

    println!("el valor almacenado en number es: {}", number_pointer.deref());
    println!("el valor almacenado en number es: {}", *number_pointer);


    let pais = String::from("Austria");      // String denominada pais
    let pais_ref = &pais;                   // pais_ref es una referencia al valor
    println!("{:p}   {:p}", pais_ref, &pais);
    let pais = 8;                               // Nueva variable pais con un valor 8.
                                                     // Sin relación con la anterior, ni con pais_ref
    println!("{}, {}", pais_ref, pais);              // pais_ref sigue "apuntando" al dato Austria
    println!("{:p}   {:p}", pais_ref, &pais);


    // std::mem::size_of::<Type>() devuelve el tamaño en bytes de un tipo
   println!("Un u8 ocupa {:?} bytes. Es de tamaño fijo.", std::mem::size_of::<u8>());
   println!("Un f64 ocupa {:?} bytes. Es de tamaño fijo.", std::mem::size_of::<u64>());
}

//***************************************************************************** Manejo alloc y dealloc
/* Notas:   
Es importante tener en cuenta que la función malloc no está disponible por defecto en Rust, por lo que debemos
importar la biblioteca std::alloc. Además, debemos usar la función con precaución, ya que estamos trabajando
con punteros y operaciones unsafe.
*/

pub fn memoria_alloc_0() {
    let titulo = String::from(" Manejo alloc y dealloc 0 ");
    imprime_titulo(&titulo);

    // Reserva un bloque de memoria de 16 bytes
    let size = 64;
    println!("Tamaño de la reserva {} Bit, {} Bytes", (64 * size/8), size);
    let ptr = malloc(size);
    println!("Dirección de memoria de inicio de la reserva {:p}", ptr);
    unsafe { println!("Contenido de la primera posición (no modificarlo): {}", *ptr as u8); }

    // Código para utilizar la memoria asignada (usar el puntero)
    unsafe { *(ptr as *mut u8) = 42 }
    unsafe { println!("Contenido de la primera posición (modificarlo)   : {:X}", *ptr as u8); }

    /* El siguiente bloque rellena la memoria reservada con el valor del contador i
     unsafe {
        for i in 0..size {
        let contenido:u8 = (i + 0).try_into().unwrap();
        ptr.add(i).write(contenido);
        ptr.add(i).write(0xaa);         // Rellena de AA toda la memoria reservada
        ptr.write(0xff);                // Escribe FF en la primera dirección
        }
    }
    */

    // Imprime el rango de memoria en hexadecimal
    println!(" Dir. Memoria   || 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F");
    println!("--------------  || -----------------------------------------------");
    for i in 0..size {
        
        if i % 16 == 0 {
            let address = unsafe { ptr.add(i) };
            print!("{:p}: || ", address);
        }
        let address = unsafe { ptr.add(i) };
        let value = unsafe { *address };
        print!("{:02X} ", value);
        if (i + 1) % 16 == 0 {
            println!("");
            if (i + 1) % 240 == 0 {
                println!("Presione cualquier tecla para continuar...");
                std::io::stdin().read_line(&mut String::new()).unwrap();
            }
        }
    }
    if size % 15 != 0 {
        println!("");
        println!("--------------  || -----------------------------------------------");
    } else {
        println!("--------------  || -----------------------------------------------");
    }

    // Imprime el rango de memoria en hexadecimal
    // for i in 0..size {
    //     let address = unsafe { ptr.add(i) };
    //     let value = unsafe { *address };
    //     println!("{:p}: 0x{:x}", address, value);
    // }

    // Liberar la memoria asignada
    free(ptr);
}

// Función que asigna un bloque de memoria dinámica
fn malloc(size: usize) -> *mut u8 {
    let layout = Layout::from_size_align(size, 1).unwrap();
    println!("Diseño(Layout) {:?}", layout);
    unsafe { alloc(layout) }
}

// Función que libera un bloque de memoria dinámica
fn free(ptr: *mut u8) {
    unsafe { dealloc(ptr, Layout::from_size_align(1, 1).unwrap()) }
}

//***************************************************************************** Manejo alloc y dealloc
pub fn memoria_alloc_1() {
    let titulo = String::from(" Manejo alloc y dealloc 1 ");
    imprime_titulo(&titulo);

    // Tamaño en bytes que queremos reservar de memoria
    let size = 10;
    println!("Tamaño de la reserva {} Bit, {} Bytes", (64 * size/8), size);
    // tamaño del bloque de memoria solicitado, medido en bytes.
    let diseno = Layout::from_size_align(size, 8).unwrap();
    println!("Diseño(Layout) {:?}", diseno);

    // Reservamos memoria con malloc
    let ptr = unsafe { alloc(diseno) };
    println!("Dirección de memoria de inicio de la reserva {:p}", ptr);
    unsafe { println!("Contenido de la primera posición (no modificarlo): {}", *ptr as u8); }

    // Código para utilizar la memoria asignada (usar el puntero)
    unsafe { *(ptr as *mut u8) = 42 }
    unsafe { println!("Contenido de la primera posición (modificarlo)   : {:X}", *ptr as u8); }

    // Imprime el rango de memoria en hexadecimal
    println!(" Dir. Memoria   || 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F");
    println!("--------------  || -----------------------------------------------");
    for i in 0..size {
        
        if i % 16 == 0 {
            let address = unsafe { ptr.add(i) };
            print!("{:p}: || ", address);
        }
        let address = unsafe { ptr.add(i) };
        let value = unsafe { *address };
        print!("{:02X} ", value);
        if (i + 1) % 16 == 0 {
            println!("");
            if (i + 1) % 240 == 0 {
                println!("Presione cualquier tecla para continuar...");
                std::io::stdin().read_line(&mut String::new()).unwrap();
            }
        }
    }
    if size % 15 != 0 {
        println!("");
        println!("--------------  || -----------------------------------------------");
    } else {
        println!("--------------  || -----------------------------------------------");
    }

    // Imprime el rango de memoria en hexadecimal
    // for i in 0..size {
    //     let address = unsafe { ptr.add(i) };
    //     let value = unsafe { *address };
    //     println!("{:p}: 0x{:x}", address, value);
    // }

    // Liberamos la memoria con free
    unsafe { dealloc(ptr, diseno) };
}

//***************************************************************************** Manejo HEAP
use std::collections::BinaryHeap;

pub fn memoria_heap() {
    let titulo = String::from(" Manejo HEAP ");
    imprime_titulo(&titulo);

    // let mut heap = BinaryHeap::new();
    let mut heap = BinaryHeap::with_capacity(6);

    heap.push(3);
    heap.push(2);
    heap.push(1);
    heap.push(4);
    heap.push(3);
    heap.push(2);
    println!(
        "Capacidad reservada heap {}, uso actual de heap {}, contenido {:?}",
        heap.capacity(),
        heap.len(),
        heap
    );
    let mut min = heap.pop();
    println!("Capacidad reservada heap {}, uso actual de heap {}, contenido {:?}, elemento eliminado: {:?}, ", heap.capacity(), heap.len(), heap, min);
    min = heap.pop();
    println!("Capacidad reservada heap {}, uso actual de heap {}, contenido {:?}, elemento eliminado: {:?}, ", heap.capacity(), heap.len(), heap, min);
    heap.push(1);
    heap.push(4);
    heap.push(1);
    println!(
        "Capacidad reservada heap {}, uso actual de heap {}, contenido {:?}",
        heap.capacity(),
        heap.len(),
        heap
    );
    heap.clear();
    println!(
        "Capacidad reservada heap {}, uso actual de heap {}, contenido {:?}",
        heap.capacity(),
        heap.len(),
        heap
    );
}

//***************************************************************************** Manejo HEAP y STACK
use std::ptr;

pub fn memoria_heap_stack() {
    let titulo = String::from(" Manejo HEAP y STACK");
    imprime_titulo(&titulo);

    println!("");
    println!("Memoria Stack");
    println!("Se crea la variable puntero *pun_a y se guarda a 0 (Rust no contempla Null)");
    println!("Se crea la variable var_a y se almacena el valor 20:");
    /* Detalles:                            
    *mut i32 : esto se refiere al tipo de variable que se está declarando. En este caso, es un puntero mutable
    a un entero de 32 bits. El asterisco (*) indica que estamos declarando un puntero, mientras que i32 es el
    tipo de datos al que apuntará el puntero. El modificador mut indica que el puntero será mutable, lo que
    significa que se puede modificar el valor al que apunta.
    ptr::null_mut() : esto inicializa el puntero con un valor nulo. ptr es un módulo dentro de la biblioteca estándar
    de Rust que proporciona funciones y tipos relacionados con punteros. null_mut() es una función que devuelve un
    puntero nulo de un tipo determinado.
    */
    let mut pun_a: *mut i32 = ptr::null_mut();
    println!("*pun_a\t{:p}", pun_a);
    let mut var_a: i32 = 20;
    println!("var_a\t{}", var_a);
    println!("Se almacena en pun_a la dirección de memoria que el sistema asigno a var_a");
    println!("Mostramos la dirección de memoria de var_a y su contenido:");
    pun_a = &mut var_a as *mut i32;
    println!("pun_a\t{:p}", pun_a);
    unsafe {
        println!("*pun_a\t{}", *pun_a);
    }

    println!("");
    println!("Memoria Heap");
    println!("Se crea *pun_b y se devuelve la dirección de memoria heap asignada en él");
    let mut pun_b: *mut i32 = Box::into_raw(Box::new(0));
    println!("*pun_b\t{:p}", pun_b);
    println!("Por desreferencia se almacena en esa dirección de memoria el valor 40");
    unsafe {
        *pun_b = 40;
    }
    println!("Mostramos nuevamente la dirección de memoria y su contenido");
    println!("pun_b\t{:p}", pun_b);
    unsafe {
        println!("*pun_b\t{}", *pun_b);
    }
    println!("Se borra el contenido del puntero y el puntero con Box::from_raw()\n");
    unsafe {
        pun_b = ptr::null_mut();
        drop(Box::from_raw(pun_b));
    }
}

//***************************************************************************** Manejo Punteros
/* Notas:   
Este código muestra cómo acceder a la memoria de un arreglo y como imprimir los
valores en hexadecimal. El arreglo data es un arreglo de u8 con 10 elementos, que
se utiliza para mostrar un ejemplo de inspección de memoria. Se obtiene un puntero
al inicio del arreglo usando as_ptr() y un puntero al final del arreglo usando la
función unsafe add(). Luego, se itera sobre el rango de memoria utilizando un bucle
for y se accede a los valores de la memoria usando unsafe *. Finalmente, se imprimen
los valores en hexadecimal usando la dirección de memoria y el valor.  -->
*/
//use std::ptr;

pub fn memoria_ptr0() {
    let titulo = String::from(" Manejo Punteros ");
    imprime_titulo(&titulo);

    let data: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let start_ptr = data.as_ptr();
    let end_ptr: *const u8 = unsafe { start_ptr.add(data.len()) };
    println!("Start: {:p} ", start_ptr);
    println!("End  : {:p} ", end_ptr);

    // Imprime el rango de memoria en hexadecimal
    for i in 0..data.len() {
        let address = unsafe { start_ptr.add(i) };
        let value = unsafe { *address };
        println!("{:p}: 0x{:x}", address, value);
    }
}

//***************************************************************************** Manejo Punteros
/* Notas:   
Este código convierte un slice de bytes en un vector de bytes y luego obtiene y muestra la
dirección de memoria en la pila y en la heap en formato hexadecimal, así como el tamaño en
 bytes de los datos en la pila y en la heap.
*/
use std::mem;

pub fn memoria_prt1() {
    let titulo = String::from(" Manejo Punteros ");
    imprime_titulo(&titulo);

    let data = b"Memoria";

    // Convertir un slice de bytes en un vector de bytes
    let vec = data.to_vec();

    // Obtener la dirección de memoria en la pila en formato hexadecimal
    let stack_ptr_hex = format!("{:p}", &data[0]);

    // Obtener la dirección de memoria en la heap en formato hexadecimal
    let heap_ptr_hex = format!("{:p}", vec.as_ptr());

    // Imprimir la dirección de memoria en la pila
    println!("Datos en la pila en: {}", stack_ptr_hex);

    // Imprimir la dirección de memoria en la heap
    println!("Datos en el heap en: {}", heap_ptr_hex);

    // Obtener el tamaño en bytes de los datos en la pila
    let stack_size = mem::size_of_val(&data);

    // Obtener el tamaño en bytes de los datos en la heap
    let heap_size = mem::size_of_val(&vec);

    // Imprimir el tamaño en bytes de los datos en la pila
    println!("Los datos en la pila son {} bytes", stack_size);

    // Imprimir el tamaño en bytes de los datos en la heap
    println!("Los datos en el heap son {} bytes", heap_size);
}

//***************************************************************************** ManuallyDrop
/* Notas:   
Ejemplo de cómo manipular y reconstruir un Vec sin ejecutar su destructor para
tener el control completo de la asignación de memoria.

Primero, se crea un Vec con los elementos [1, 2, 3]. Luego, se usa mem::ManuallyDrop
para evitar que se ejecute el destructor de var_vector, lo que permite que el código
tenga control total sobre la asignación de memoria.

A continuación, se extraen el puntero, la longitud y la capacidad de var_vector.
Luego, se utiliza un bucle for y la función ptr::write para sobrescribir la memoria
con los valores [4, 5, 6].

Por último, se utiliza la función Vec::from_raw_parts para reconstruir el Vec original
a partir de los elementos sobrescritos. Se realiza una comprobación assert para
asegurarse de que el Vec reconstruido tenga los valores correctos [4, 5, 6].
*/
#[allow(unused)]
pub fn memoria_manuallydrop() {
    let titulo = String::from(" ManuallyDrop ");
    imprime_titulo(&titulo);

    use std::ptr;
    use std::mem;

    let var_vector = vec![1, 2, 3];
    println!("{:?}", var_vector);

    // Evite ejecutar el destructor de `var_vector` para que tengamos el control completo
    // de la asignación.
    let mut var_vector = mem::ManuallyDrop::new(var_vector);

    // Extraiga las diversas piezas importantes de información sobre `var_vector`
    let p = var_vector.as_mut_ptr();
    let len = var_vector.len();
    let cap = var_vector.capacity();

    unsafe {
        // Sobrescribir la memoria con 4, 5, 6
        for i in 0..len {
            ptr::write(p.add(i), 4 + i);
        }

        // Pon todo de nuevo junto en un Vec
        let rebuilt = Vec::from_raw_parts(p, len, cap);
        assert_eq!(rebuilt, [4, 5, 6]);
        println!("{:?}", var_vector);
    }
    println!("{:?}", var_vector);
}

//***************************************************************************** 
pub fn ejemplo_impresion_datos_hex() {
    // Vector de 256 elementos u8
    let mut data = [0u8; 256];
    // Llenar el vector con datos de prueba
    for i in 0..data.len() {
        data[i] = i as u8;
    }
    // Imprimir los datos en formato hexadecimal
    let mut line_count = 0;
    for (i, byte) in data.iter().enumerate() {
        if i % 15 == 0 {
            print!("\n{:04X}: ", i);
            line_count += 1;
            if line_count == 16 {
                line_count = 0;
                // Esperar por una pulsación de tecla
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                if data[i] == 0xFF {
                    break;
                }
            }
        }
        print!("{:02X} ", byte);
    }
}

//***************************************************************************** 