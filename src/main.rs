/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          21-02-2023
    Titulo:         introducción a RUST
    Descripción:    Inicio ;-)
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

    Dependencias:

    Compilar:

    Licencia:

***************************************************************************************/
/* Notas:   
A este tipo de etiquetado con # seguido de un nombre, se le denomina en Rust atributo
#derive(Debug)
#[allow(dead_code)]
#[allow(unused_variables)]
*/

//use ejer_hiperbolic::*;
mod ejer_hiperbolic;
mod ejer_lets_get_rusty;
mod ejer_rust_en_sp;
mod proyectos;

//include! ("cripto.rs");
//pub mod cripto;
mod poo;
mod lib_memoria;
mod lib_closure;
mod lib_flujos;
mod lib_errores;
mod cap13;



 fn main() {
// $env:RUST_BACKTRACE=0
    
    // let _result = std::process::Command::new("clear").status().unwrap();
    
/* Pasando argumentos   
    use std::env;
    
    let argumentos: Vec<String> = env::args().collect();
    //dbg!(argumentos);
    let parametro0 =&argumentos[0];
    println!("Uno: {}", parametro0);

    let primer_argumento = env::args().skip(1).next();
    match name {
        Some(n) => println!("¡Hola! {}", n),
        None => panic!("No he recibido ningún nombre"),
    }
*/



    ejer_rust_en_sp::ejer_basicos::var_y_tipos();




//*****************************************************************************
/* Características básicas  
    ejer_rust_en_sp::ejer_basicos::var_y_tipos();
    ejer_rust_en_sp::ejer_basicos::var_y_funciones();
    ejer_rust_en_sp::ejer_basicos::duración_funciones();
    ejer_rust_en_sp::ejer_basicos::duración_tipos();
*/
/* Estructuras de control   
    ejer_rust_en_sp::ejer_estructuras_de_control::igual();
    ejer_rust_en_sp::ejer_estructuras_de_control::bucles_y_control();
    ejer_rust_en_sp::ejer_estructuras_de_control::option_prueba_concepto_0();
    ejer_rust_en_sp::ejer_estructuras_de_control::option_prueba_concepto_1();
    ejer_rust_en_sp::ejer_estructuras_de_control::option_prueba_concepto_2();
*/
/* Estructuras 0            
    ejer_rust_en_sp::ejer_estructuras0::arreglos(4);
    ejer_rust_en_sp::ejer_estructuras0::vector();
    ejer_rust_en_sp::ejer_estructuras0::tupla();
    ejer_rust_en_sp::ejer_estructuras0::estructura();
    ejer_rust_en_sp::ejer_estructuras0::enumeradores();
    ejer_rust_en_sp::ejer_estructuras0::desestructurar();
    ejer_rust_en_sp::ejer_estructuras0::operador_punto();
*/
/* Estructuras 1            
    ejer_rust_en_sp::ejer_estructuras1::metodo_0();
    ejer_rust_en_sp::ejer_estructuras1::metodo_1();
    ejer_rust_en_sp::ejer_estructuras1::metodo_2();
*/
/* Genéricos/Option/Result  
    ejer_rust_en_sp::ejer_genericos_option_result::genericos_0();
    ejer_rust_en_sp::ejer_genericos_option_result::genericos_1();
    ejer_rust_en_sp::ejer_genericos_option_result::option_intro_0();
    ejer_rust_en_sp::ejer_genericos_option_result::option_intro_1();
    ejer_rust_en_sp::ejer_genericos_option_result::option_intro_2();
    ejer_rust_en_sp::ejer_genericos_option_result::option_0();
    ejer_rust_en_sp::ejer_genericos_option_result::option_1();
    ejer_rust_en_sp::ejer_genericos_option_result::option_2();
    ejer_rust_en_sp::ejer_genericos_option_result::result_0();
    ejer_rust_en_sp::ejer_genericos_option_result::result_1();
    ejer_rust_en_sp::ejer_genericos_option_result::result_2();
*/
/* Estructuras 2            
    ejer_rust_en_sp::ejer_estructuras2::hashmap_0();
    ejer_rust_en_sp::ejer_estructuras2::btreemap_0();
*/

/* Revisar                  
    lib_closure::closure_0();
    lib_closure::closure_Hyperbolic_1();
    lib_closure::closure_Hyperbolic_2();
    lib_closure::closure_rhymu_0();
    lib_closure::closure_rust_list();

    
    lib_flujos::flujos();


    lib_memoria::memoria_deref();
    lib_memoria::memoria_alloc_0();
    lib_memoria::memoria_alloc_1();
    lib_memoria::memoria_heap();
    lib_memoria::memoria_ptr0();
    lib_memoria::memoria_prt1();
    

    lib_errores::enum_opciones();
    lib_errores::gestion_error();


    poo::poo();

    //-----------------------------
    cap13::sorteo();
    
    main_0()
    cripto::main_0();
*/
}

//*****************************************************************************
/*  proyectos           
    proyectos::firebase::firebase();
    proyectos::sim_cpu::sim_cpu();

*/
/*  Hiperbolic          
    ejer_hiperbolic::vt_03::vt_03();
    ejer_hiperbolic::vt_16::vt_16();
    
*/
/*  Let's Get Rusty     
    ejer_lets_get_rusty::vt_39::xx();

*/
