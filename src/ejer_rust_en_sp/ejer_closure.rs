/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          29-04-2023
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
/* Apuntes:                                 
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
/* Detalles de ||                       
- Una || que no encierra ninguna variable exterior en su interior es una función anónima.
  No es, estrictamente, un closure.
- Una || que sí encierra una variable exterior en su interior sí es un cierre.
*/
#![allow(dead_code)]
#![allow(unused_variables)]

use colored::*;
use std::thread;
use std::time::Duration;
use std::collections::HashMap;

fn imprime_titulo(titulo: &String) {
    println!("\n{:*^80}", titulo.red());
}

//***************************************************************************** Introducción a Closure
pub fn closure() {
    let titulo = String::from(" Introducción a Closure ");
    imprime_titulo(&titulo);

    let num_vec = vec![2, 4, 6];

    let double_vec = num_vec        // usa el vector
        .iter()                     // para recorrerlo (iterar)
        .map(|number| number * 2)   // para cada elemento, lo multiplica por 2
        .collect::<Vec<i32>>();     // y crea un nuevo vector a partir de ello
    println!("{:?}", double_vec);

    double_vec
        .iter()      // itera sobre num_vec
        .enumerate() // obtiene pares de (índice, valor)
        .for_each(|(index, number)| println!("El índice {} contiene el valor {}", index, number)); // imprime para cada elemento

//*************************************
    println!();
    let some_numbers = vec![0, 1, 2, 3, 4, 5]; // un Vec<i32>
    let some_words = vec!["cero", "uno", "dos", "tres", "cuatro", "cinco"]; // un Vec<&str>

    let number_word_hashmap = some_numbers
        .into_iter()                 // obtiene un iter
        .zip(some_words.into_iter()) // se combinan con .zip() dos iter.
//        .collect::<HashMap<i32, &str>>();
        .collect::<HashMap<_, _>>();
    println!("Para la clave {} se obtiene {}.", 2, number_word_hashmap.get(&2).unwrap());

//*************************************
    println!();
    let new_vec = vec![8, 10, 0]; // solo un vec con números
    let number_to_add = 5;       // Se usa en el cálculo más adelante
    let mut empty_vec = vec![];  // El resultado va aquí

    for index in 0..5 {
        empty_vec.push(
            new_vec
               .get(index)
               .and_then(|number| Some(number + 1))
               .and_then(|number| Some(number + number_to_add))
        );
    }
    println!("{:?}", empty_vec);

//*************************************
    println!();
    let one = true;
    let two = false;
    let three = true;
    let four = true;
    println!("{}", one && three); // prints true
    println!("{}", one && two && three && four); // prints false

    let first_try = vec![Some("éxito"), None, Some("éxito"), Some("éxito"), None];
    let second_try = vec![None, Some("éxito"), Some("éxito"), Some("éxito"), Some("éxito")];
    let third_try = vec![Some("éxito"), Some("éxito"), Some("éxito"), Some("éxito"), None];

    for i in 0..first_try.len() {
        println!("{:?}", first_try[i].and(second_try[i]).and(third_try[i]));
    }

//*************************************
    println!();
    let num_vec = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    println!("{:?}", num_vec.iter().find(|&number| number % 3 == 0)); // find takes a reference, so we give it &number
    println!("{:?}", num_vec.iter().find(|&number| number * 2 == 30));
    println!("{:?}", num_vec.iter().position(|&number| number % 3 == 0));
    println!("{:?}", num_vec.iter().position(|&number| number * 2 == 30));
}

//***************************************************************************** Introducción a Closure 0
pub fn closure_0() {
    let titulo = String::from(" Introducción a Closure 0 ");
    imprime_titulo(&titulo);

    // Se define una variable mutable var_n de tipo u8 y se le asigna el valor 2.
    let mut var_n: u8 = 4;
    println!("Valor de var_n: {}", var_n);
//*************************************
    /* Se define un closure llamado mi_closure0 que toma un parámetro x de tipo u8 e
       imprime un mensaje con el valor incluido en la llamada almacenado en x. */
    let val_clo0 = |x: u8| println!("val_clo0: Valor de x = {}", x);
    // Se llama al closure mi_closure0 con el valor 2 como argumento.
    val_clo0(2);
    val_clo0(0);

//*************************************
    /* Se define una variable mutable val_clo1 que almacena un closure con la sintaxis
    move, lo que significa que el closure toma propiedad de la variable var_n y no la
    comparte con otros closures o funciones que puedan existir en el mismo ámbito. El
    closure toma un parámetro x de tipo u8 y devuelve un valor u8. */
    let mut val_clo1 = move | x: u8| -> u8 {
        var_n +=2;
        println!("val_clo1: valor de x = {}, de var_n = {}, de x*var_n = {}", x, var_n, x * var_n);
        return var_n;
    };
    println!("val_clo1: 1ª llamada");
    // OJO var_n vale 6 tras la 1ª llamada dentro del val_clo1 por el uso de move.
    println!("val_clo1: valor devuelto para val_clo1(2): {}", val_clo1(2));
    println!("Valor de var_n: {}", var_n);
    println!("val_clo1: 2ª llamada");
    // OJO var_n vale 6 tras la 2ª llamada dentro del val_clo1 por el uso de move.
    println!("val_clo1: valor devuelto para val_clo1(2): {}", val_clo1(2));
    println!("Valor de var_n: {}. No varia fuera del closure val_clo1\n", var_n);
    assert_eq!(var_n, 4);
    
//*************************************
    /* Se define un closure llamado val_clo2 que toma un parámetro x de tipo u8 y
    devuelve un valor u8. A diferencia de val_clo1, val_clo2 no usa la palabra
    clave move, por lo que no toma posesión de la variable var_n dentro del closure.
    */
    let mut val_clo2 = | x: u8|-> u8 {
        //let var_ref_n = &mut var_n;
        //*var_ref_n +=2;
        var_n +=2;
        println!("val_clo2: valor de x = {}, de var_n = {}, de x*var_n = {}", x, var_n, x * var_n);
        var_n
    };
    println!("val_clo2: 1ª llamada");
    // OJO var_n vale 4 tras la 1ª llamada fuera del val_clo2 por el no uso de move.    
    println!("val_clo2: valor devuelto para var_n por val_clo2(2): {}", val_clo2(2));
//    println!("Valor de var_n: {}", var_n);
    println!("val_clo2: 2ª llamada");
    println!("val_clo2: valor devuelto para var_n por val_clo2(2): {}", val_clo2(2));
    println!("Valor de var_n: {}. Varía fuera del closure val_clo2\n", var_n);
/*
En resumen, el código muestra cómo definir y utilizar closures en Rust, y cómo el uso
de la palabra clave move puede afectar la propiedad y la compartición de las variables
en el ámbito del closure. También se muestra cómo se puede utilizar un closure varias
veces con diferentes valores de entrada para generar diferentes resultados.
*/
}

//***************************************************************************** Introducción a Closure 1
pub fn closure_1() {
    let titulo = String::from(" Introducción a Closure 1 ");
    imprime_titulo(&titulo);

    let valor_i = 12;
    let valor_j = 4;
    rutina_aux0_closure_0(valor_i, valor_j);
}

//*************************************
fn rutina_aux0_closure_0(var_i: u32, var_j: u32) {
    // Función encapsulada en una variable --> |var_a|
    let cierre = |mut var_a| {
        println!("Valor recibido  para var_a: {}", var_a);
        var_a -= 4;
        println!("Valor calculado para var_a: {}", var_a);
        thread::sleep(Duration::from_secs(2));
        return var_a;
    };
    //---------------------------------
    if var_i < 10 {
        println!("Caso 1: valor devuelto por función anonima {}", cierre(var_i));
    } else if var_j == 4 {
            println!("Caso 2: valor devuelto por función anonima {}", cierre(var_j));
    } else {
            println!("Caso 3: valor devuelto por función anonima {}", cierre(var_i));
    }
}

//***************************************************************************** Ejemplo Hiperbolic 
//Hyperbolic Time Academy: https://www.youtube.com/watch?v=vsVL8CVGFkM
pub fn closure_hyperbolic_1() {
    let titulo = String::from(" Ejemplo Hiperbolic 1 ");
    imprime_titulo(&titulo);

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
        var_x = v; // NOTA 1
        println!("v {}, var_x: {} ", v, var_x);
    };
    /*
    Las variables val_x se capturan en el closure utilizando la palabra clave move.
    Cuando se llama a closure, el closure imprime los valores de val_x. Después de llamar
    al closure.
    NO SE CUMPLE --> intentar acceder a val_x causará un error, ya que se han movido a closure
    */
    closure(5); // NOTA 1. El compilador mantendrá el primer tipo inferido
                //closure("xx");// NOTA 1. al usar el closure
    println!("     var_x: {} \n", var_x);
}

//***************************************************************************** Ejemplo Hiperbolic 2
// Hyperbolic Time Academy: https://www.youtube.com/watch?v=vsVL8CVGFkM
pub fn closure_hyperbolic_2() {
    let titulo = String::from(" Ejemplo Hiperbolic 2 ");
    imprime_titulo(&titulo);

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
        add_0(|x| println!("Resultado = {}", num + x));
    }
    {
        let mut num = 10;
        add_1(|x| {
            num += 5;
            println!("Resultado = {}", num + x)
        });
        println!("{}", num);
    }
}

// Fn Trait
fn add_0<F>(func: F)
    where
        F: Fn(u8),
        {
            func(5)
        }
// FnMut Trait
fn add_1<F>(mut func: F)
    where
        F: FnMut(u8),
        {
            func(5)
        }

//***************************************************************************** Ejemplo Rhymu 
// Rhymu's Videos: https://www.youtube.com/watch?v=drYtaZopxgQ
pub fn closure_rhymu_0() {
    let titulo = String::from(" Ejemplo Rhymu 0 ");
    imprime_titulo(&titulo);

    let numeros = [1, 2, 3, 4, 1];
    /* into_iter() versus iter()            
    into_iter() consume la colección original, transfiriendo la propiedad de sus elementos, mientras
    que iter() permite la iteración de solo lectura sin consumir la colección.
    */
    let numeros_on_iter = numeros.into_iter();
    println!("numeros on iter     = {:?}", numeros_on_iter);
    
    let numeros_en_iter = numeros.iter();
    println!("numeros             = {:?}", numeros);
    println!("numeros_en_iter     = {:?}", numeros_en_iter);
    println!("numeros direcciones = {:?}", numeros.as_ptr_range());

    let multi = numeros_en_iter.map(multiplicar(5));
    println!("Multi               = {:?}", multi);

    for i in multi { println!("           = {:?}", i); }
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
fn multiplicar(mut x: i32) -> impl FnMut(&i32) -> i32 {
    // Al usar iter(), se debe usar impl FnMut con una referencia: "impl FnMut(&i32)"
    println!("x recibido = {:?}", x);
    move |n| {
        x += 1;
        println!("         x * {:?}", x);
        n * x
    }
}

//*****************************************************************************
// Rust (Rainer Stropek): https://www.youtube.com/watch?v=bgZa9VRBhYU
pub fn closure_rust_list() {
    let titulo = String::from(" Ejemplo Rust list ");
    imprime_titulo(&titulo);

    {
        let val_x = 21;
        let get_respuesta = |y: i32| return val_x + y;
        println!("\nGet respuesta = {:?}, x = {}", get_respuesta(21), val_x);
    }
    //*************************************
    {
        // Función regular -->
        fn add(x: i32, y: i32) -> i32 {
            return x + y;
        }
        let _f = add;
        // Sustituimos la función regular por closure
        let _f = |x: i32, y: i32| {
            return x + y;
        };
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
        fn add(x: i32, y: i32) -> i32 {
            return x + y;
        }

        fn calc_and_print(x: i32, y: i32, calculator: fn(i32, i32) -> i32) {
            let resultado = calculator(x, y);
            println!("Resultado sin closure       = {}\n", resultado);
        }
        calc_and_print(3, 3, add);
    }
}

//*****************************************************************************
