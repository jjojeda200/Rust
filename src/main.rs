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

***************************************************************************************/
/* Notas:                   
Expresiones son instrucciones que devuelven algún valor como resultado.
Declaraciones y Sentencias son instrucciones que no dan lugar a ningún resultado.
*/
/* Notas:                   
A este tipo de etiquetado con # seguido de un nombre, se le denomina en Rust atributo
#derive(Debug)
#[allow(dead_code)]
#[allow(unused_variables)]
*/

// Declaración de módulos   
//include! ("cripto.rs");
//pub mod cripto;
//use ejer_hiperbolic::*;   
mod ejer_hiperbolic;
mod ejer_lets_get_rusty;
mod ejer_rust_en_sp;
mod proyectos;
mod rust_standard_library;
mod lib_closure;
mod lib_flujos;
mod cap13;


/* función imprime_titulo   
La función imprime_titulo(titulo: &String) recibe como parámetro un puntero a
una cadena de texto String y utiliza la macro println!() para imprimir el valor
de la cadena de texto centrado en 80 caracteres y rodeado por asteriscos.
*/
fn imprime_titulo(titulo: &String) {
    println!("\n{:*^80}", titulo);
}



fn main() {
// $env:RUST_BACKTRACE=0
    let titulo = String::from(" Función main, punto principal de entrada al programa ");
    imprime_titulo(&titulo);
    
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


 

ejer_rust_en_sp::ejer_trait_rasgos2::ejemplo_generico_0();
ejer_rust_en_sp::ejer_trait_rasgos2::ejemplo_generico_memoria();


//*****************************************************************************
/* Características básicas                      
    ejer_rust_en_sp::ejer_basicos::var_y_tipos();
    ejer_rust_en_sp::ejer_basicos::var_y_funciones();
    ejer_rust_en_sp::ejer_basicos::var_y_ret_funciones();
    ejer_rust_en_sp::ejer_basicos::duración_funciones();
    ejer_rust_en_sp::ejer_basicos::duración_tipos();
*/
/* Estructuras de control                       
    ejer_rust_en_sp::ejer_estructuras_de_control::igual();
    ejer_rust_en_sp::ejer_estructuras_de_control::bucles_y_control();
    // Primera pruebas con Option
    ejer_rust_en_sp::ejer_estructuras_de_control::option_prueba_concepto_0();
    ejer_rust_en_sp::ejer_estructuras_de_control::option_prueba_concepto_1();
    ejer_rust_en_sp::ejer_estructuras_de_control::option_prueba_concepto_2();
*/
/* Estructuras 0                                
    ejer_rust_en_sp::ejer_estructuras0::arreglos(4);
    ejer_rust_en_sp::ejer_estructuras0::rangos_slice();
    ejer_rust_en_sp::ejer_estructuras0::cadenas_string();
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
/* Estructuras 2                                
    ejer_rust_en_sp::ejer_estructuras2::hashmap_0();
    ejer_rust_en_sp::ejer_estructuras2::btreemap_0();
    ejer_rust_en_sp::ejer_estructuras2::metodos_0();
    ejer_rust_en_sp::ejer_estructuras2::metodos_1();
    ejer_rust_en_sp::ejer_estructuras2::hashmap_vectores();
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
/* Errores Panic/?/Result/Unwrap/Assent         
    ejer_rust_en_sp::ejer_gestion_errores::funcion_panic();
    ejer_rust_en_sp::ejer_gestion_errores::fn_interrogante();
    ejer_rust_en_sp::ejer_gestion_errores::fn_unwrap_0();
    ejer_rust_en_sp::ejer_gestion_errores::fn_unwrap_1();
    ejer_rust_en_sp::ejer_gestion_errores::fn_expect_0();
    ejer_rust_en_sp::ejer_gestion_errores::fn_assent();
*/
/* Manejo de memoria 0                          
    ejer_rust_en_sp::ejer_memoria0::memoria_deref();
    ejer_rust_en_sp::ejer_memoria0::memoria_alloc_0();
    ejer_rust_en_sp::ejer_memoria0::memoria_alloc_1();
    ejer_rust_en_sp::ejer_memoria0::memoria_heap();
    ejer_rust_en_sp::ejer_memoria0::memoria_ptr0();
    ejer_rust_en_sp::ejer_memoria0::memoria_prt1();
    ejer_rust_en_sp::ejer_memoria0::memoria_manuallydrop();
*/
/* Manejo de memoria x                          
    ejer_rust_en_sp::ejer_memoriax::memoria();
*/
/* Trait 0 (Métodos de instancia y estáticos)   
ejer_rust_en_sp::ejer_trait_rasgos0::ejemplo_instancia_0();
ejer_rust_en_sp::ejer_trait_rasgos0::ejemplo_instancia_1();
ejer_rust_en_sp::ejer_trait_rasgos0::ejemplo_estatico_0();
ejer_rust_en_sp::ejer_trait_rasgos0::ejemplo_estatico_1();
*/
/* Trait 1 (Programación Orientada a Objetos)   
ejer_rust_en_sp::ejer_trait_rasgos1::ejemplo_poo_0();
*/
/* Trait 2 (Genéricos y manejo de memoria)      
ejer_rust_en_sp::ejer_trait_rasgos2::ejemplo_generico_0();
ejer_rust_en_sp::ejer_trait_rasgos2::ejemplo_generico_memoria();
*/
/* Trait x (Rasgos)                             
    ejer_rust_en_sp::ejer_trait_rasgosx::fn_windows();
    ejer_rust_en_sp::ejer_trait_rasgosx::fn_trait();
*/


/* Revisar                  
    lib_closure::closure_0();
    lib_closure::closure_Hyperbolic_1();
    lib_closure::closure_Hyperbolic_2();
    lib_closure::closure_rhymu_0();
    lib_closure::closure_rust_list();

    
    lib_flujos::flujos();
        
    //-----------------------------
    cap13::sorteo();
    
    main_0()
    cripto::main_0();
*/
}

//*****************************************************************************
/* The Rust Standard Library    
    rust_standard_library::std_ops_try::fn_std_ops_try_0();

*/
/* Hiperbolic                   
    ejer_hiperbolic::vt_03::vt_03();
    ejer_hiperbolic::vt_16::vt_16();
    
*/
/* Let's Get Rusty              
    ejer_lets_get_rusty::vt_39::xx();

*/

/* proyectos                    
    proyectos::firebase::firebase();
    proyectos::sim_cpu::sim_cpu();

*/
