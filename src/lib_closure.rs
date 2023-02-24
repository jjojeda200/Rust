/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          01-02-2023
    Titulo:         introducción a RUST
    Descripción:    closures, closure move
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
use std::thread;
use std::time::Duration;

#[allow(dead_code)]
pub fn closure_0() {
    let valor_i = 12;
    let valor_j = 4;

    rutina_aux0_closure_0(valor_i, valor_j);
}

//*************************************
fn rutina_aux0_closure_0(var_i: u32, var_j: u32) {
    /*
    Los Closures son funciones anónimas que pueden capturar variables del entorno envolvente.
    Esto es poderoso, significa que el cierre puede interactuar directamente con las variables
    que definió fuera del cuerpo del cierre (a diferencia de las funciones en las que tiene
    que pasar parámetros). Los cierres se utilizan en subprocesos y otra programación funcional
    como características de Rust (como Map, Reduce, Filter, etc.)
    
    Los closures o cierres son funciones anónimas que se pueden almacenar en una variable
    o pasar como argumentos a otras funciones, esto nos permite crearlo en un lugar y luego
    llamarla para evaluarla en un contexto diferente.
    Los dos pipes son los que delimitan al closure
    */
    // Función encapsulada en una variable --> |var_a|
    let cierre = |mut var_a| {
        println!("Valor recibido para var_a: {}", var_a);
        var_a -= 4;
        println!("Valor nuevo para var_a: {}", var_a);
        thread::sleep(Duration::from_secs(2));
        return var_a;
    };

    //---------------------------------
    if var_i < 10 {
        println!("Hoy haz {} de descanso", cierre(var_i));
    } else {
        if var_j == 4 {
            println!("Descansa un poco y recuerda hidratarte");
            cierre(var_j);
        } else {
            println!("Hoy estudia por {} minutos", cierre(var_i));
        }
    }
}

//*****************************************************************************
//Hyperbolic Time Academy: https://www.youtube.com/watch?v=vsVL8CVGFkM
#[allow(dead_code)]
pub fn closure_hyperbolic_1() {
    /*
    Un closure move es un tipo de closure en Rust que toma propiedad de las variables que
    captura en su alcance. Esto significa que las variables capturadas se mueven a la
    función closure y ya no están disponibles en su alcance original.
    */
    let mut var_x = 0;

    // Con el "move" val_x se mueve dentro (copia), si "move", se modifica el valor externo
    // de val_x, probar con y sin "move"
    let mut closure = move |v| {
        println!("v {}, var_x: {} ", v, var_x);
        var_x = v;  // NOTA 1
        println!("v {}, var_x: {} ", v, var_x);
    };
    /*
    Las variables val_x se capturan en el closure utilizando la palabra clave move.
    Cuando se llama a closure, el closure imprime los valores de val_x. Después de llamar
    al closure.
    NO SE CUMPLE --> intentar acceder a val_x causará un error, ya que se han movido a closure
    */
    closure(5);     // NOTA 1. El compilador mantendrá el primer tipo inferido
    //closure("xx");// NOTA 1. al usar el closure
    println!("     var_x: {} \n", var_x);
}

//*****************************************************************************
// Hyperbolic Time Academy: https://www.youtube.com/watch?v=vsVL8CVGFkM
#[allow(dead_code)]
pub fn closure_hyperbolic_2() {
    {
        // Fn Trait
        let mut var_str = "Texto principal,".to_string();
        let var_fn_a = |x| println!("{} {}", var_str, x);
        var_fn_a("texto adicional A");
        // FnMut Trait
        let mut var_fn_b = |x| var_str.push_str(x);
        var_fn_b(" texto adicional B");
        println!("{}", var_str);
    }
    //*********************************
    {
        let num = 10;
        add_0(|x|println!("Resultado = {}", num + x));
    }
    {
        let mut num = 10;
        add_1(|x|{
            num += 5;
            println!("Resultado = {}", num + x)});
    }
}
// Fn Trait
fn add_0<F>(func: F)
    where F: Fn(u8){
        func(5)
    }
// FnMut Trait
fn add_1<F>(mut func: F)
    where F: FnMut(u8){
        func(5)
    }
//*****************************************************************************
// Rhymu's Videos: https://www.youtube.com/watch?v=drYtaZopxgQ
#[allow(dead_code)]
pub fn closure_rhymu_0() {
    let numeros = [1, 2, 3];
    let numeros_on_iter = numeros.into_iter();
    println!("\nnumeros_on_iter     = {:?}", numeros_on_iter);
    println!("{:?}", numeros);
    println!("numeros contenido   = {:?}", numeros.into_iter());
    println!("numeros direcciones = {:?}", numeros.as_ptr_range());

    let multiplicado = numeros_on_iter.map(multiplicar(4));
    println!("Multiplicado        = {:?}", multiplicado);
    for i in multiplicado {
        println!("           = {:?}", i);
    }
}

/*
// Nota 2. Forma de hacerlo sin closure -->
type FunMulti = fn(i32) -> i32;

fn multiplcar(x: i32) -> FunMulti
{
    match x {
        2 => doble,
        3 => tripe,
        _ => unimplemented!(),
    }
}

fn doble(n: i32) -> i32 { return n * 2; }
fn tripe(n: i32) -> i32 { return n * 3; }
*/

// Nota 2. Forma de hacerlo con closure (traits) -->
fn multiplicar(mut x: i32) -> impl FnMut(i32) -> i32 {
    println!("x recibido = {:?}", x);
    move |n| {
        x += 1;
        println!("         x * {:?}", x);
        n * x
    }
}

//*****************************************************************************
// Rust (Rainer Stropek): https://www.youtube.com/watch?v=bgZa9VRBhYU
#[allow(dead_code)]
pub fn closure_rust_list() {
    {
        let val_x = 21;
        let get_respuesta = |y: i32| return val_x + y;
        println!("\nGet respuesta = {:?}, x = {}", get_respuesta(21), val_x);
    }
    //*************************************
    {
        // Función regular -->
        fn add(x: i32, y: i32) -> i32 { return x + y; }
        let _f = add;
        // Sustituimos la función regular por closure
        let _f = |x: i32, y: i32| { return x + y; };
        // Podemos ahorrar las llaves al ser una sola expresión
        let _f = |x: i32, y: i32| return x + y;
        // Closure infiere el tipo de los parámetro
        let f = |x, y| x + y;
        // Closure incluye la llamada a la función "Practica no recomendad"
        println!("Resultado closure directo   = {}", (|x, y| x + y)(1, 1));

        let resultado = f(2, 2);
        println!("Resultado closure variantes = {}", resultado);
    }
    //*************************************
    {
        // Función regular -->
        fn add(x: i32, y: i32) -> i32 { return x + y; }

        fn calc_and_print( x: i32, y: i32, calculator: fn (i32, i32) -> i32) {
            let resultado = calculator(x, y);
            println!("Resultado sin closure       = {}\n", resultado);
        }
        calc_and_print(3, 3, add);
    }
}

//*****************************************************************************