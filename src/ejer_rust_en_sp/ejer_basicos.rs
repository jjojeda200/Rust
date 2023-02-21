/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          14-02-2023
    Titulo:         introducción a RUST
    Descripción:    Características básicas, variables, mutabilidad, tiempo (alcance)
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

fn imprime_titulo(titulo: &String){
    println!("\n{:*^80}", titulo);
}
//*****************************************************************************
/*  Nota: Opciones en llamada a función con variables
    1-> Transferencia de la propiedad de la variable con perdida. Si se intenta 
        usar nuevamente en la función dara error "value moved here"
    2-> Recuperación del contenido de la variable por retorno a nueva variable
        con sombreado variable original
    3-> Sin sombreado/ocultación (shadowing) en una nueva variable.
    4-> Transferencia por copia/clonación.
    5-> Transferencia por referencia, inmutable
    6-> Y mutable.
*/
#[allow(dead_code)]
pub fn var_y_funciones() {
    let titulo = String::from(" Introducción básica datos y funciones ");
    imprime_titulo(&titulo);

    /* Nota:
    Quien se hace dueño de un objeto puede decidir que sea modificable,
    aunque anteriormente no lo fuese.
    */
    let variable_0 = String::from("Variable 0");
    imprimir(variable_0);                                   // 1
    //imprimir(variable_0);     // 1 error "value moved here"

    let variable_1 = String::from("Variable 1");
    let variable_1 = imprimir(variable_1);          // 2    shadowing
    let variable_3 = imprimir(variable_1);          // 3    no shadowing
    imprimir(variable_3.clone());                           // 4
    imprimir_ref(&variable_3);                 // 5

    let mut variable_4 =String::from(variable_3);
    imprimir_ref_mut(&mut variable_4);     // 6
}

fn imprimir(variable: String) -> String {
    println!("{}", variable);
    return variable;
}

fn imprimir_ref(variable_ref: &String) {
     println!("{}", variable_ref);
}

fn imprimir_ref_mut(variable_ref_mut: &mut String) {
    variable_ref_mut.push_str(" - modificada");
    println!("{}", variable_ref_mut);
}

//*****************************************************************************
/* Nota: Tiempos de vida en funciones
https://github.com/goyox86/elpr-sources/blob/gh-pages/lifetimes.md#tiempos-de-vida
Declarar parámetros de duración genéricos entre corchetes angulares <> y agregar
la declaración entre la lista de parámetros y el nombre de la función, en valor
devuelto y todas las referencias de parámetro. todos deben tener la misma duración.
Por lo tanto, use el mismo nombre de duración, por ejemplo: 'dur o cualquier otro.
*/
#[allow(dead_code)]
pub fn duración_funciones() {
    let titulo = String::from(" Tiempos de vida en funcioness ");
    imprime_titulo(&titulo);

    let magic1 = String::from("¡abracadabra!");
    let magic2 = String::from("¡shazam!");

    let result = longest_word(&magic1, &magic2);
    println!("La palabra mágica es: {}\n", result);
}

fn longest_word<'dur>(x: &'dur String, y: &'dur String) -> &'dur String {
    if x.len() < y.len() { x } else { y }
}

//*****************************************************************************
/* Nota: Tiempos de vida en estructuras
https://github.com/goyox86/elpr-sources/blob/gh-pages/lifetimes.md#en-structs
Cada vez que un struct o una enumeración contienen una referencia en uno de sus campos,
se debe anotar esa definición de tipo con la duración de cada referencia que lleve a
cabo con ella.
*/
#[allow(dead_code)]
pub fn duración_tipos() {
    let titulo = String::from(" Tiempos de vida en estructuras ");
    imprime_titulo(&titulo);

    let texto = String::from("El veloz zorro marrón salta sobre el perro perezoso.");
    let zorro = Destacar(&texto[9..22]);
    let perro = Destacar(&texto[38..52]);
    println!("{:?}", zorro);
    println!("{:?}", perro);
}

#[derive(Debug)]
struct Destacar<'doc>(&'doc str);

//*****************************************************************************