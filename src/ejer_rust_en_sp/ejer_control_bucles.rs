/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          14-02-2023
    Titulo:         introducción a RUST
    Descripción:    Estructuras de control, bucles, match
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

fn imprime_titulo(titulo: &String) {
    println!("\n{:*^80}", titulo);
}
//***************************************************************************** Match
/* Nota:    
- A todo match le sigue un bloque de código {}
- Se escriben los patrones a la izquierda y se usa => para indicar qué hay que
  hacer cuando hay una coincidencia.
- A cada línea con un patrón se le denomina "brazo" del match.
- Entre cada "brazo" se pone una coma de separación (no se usa el punto y coma).
- La sentencia match siempre tiene que devolver el mismo tipo de datos en todas sus ramas.

Se puede usar @ para darle un nombre al valor de un patrón match con el fin
de poder usarlo en la expresión correspondiente a ese "brazo"
*/
#[allow(dead_code)]
pub fn igual() {
    let titulo = String::from(" Match (Igual) ");
    imprime_titulo(&titulo);

    let mi_numero: u8 = 2;
    match mi_numero {
        0 => println!("mi_numero es un cero"),
        1 => println!("mi_numero es un uno"),
        num @ 2 => println!("mi_numero es dos {}", num),
        _ => println!("mi_numero es algún otro número"),
    }
    
    let otro_numero = match mi_numero {
        0 => 0,
        5 => 10,
        _ => 2,
    };
    println!("otro_numero es un: {:?}\n", otro_numero);
    
    let children = 5;
    let married = true;
    
    match (children, married) {
        (children, married) if married == false => println!("Sin casar con {} niños", children),
        (children, married) if children == 0 && married == true => println!("Casado, pero sin niños"),
        _ => println!("¿Casado? {}. Número de niños: {}.\n", married, children),
    }
    
    let first = (200, 0, 0);
    let second = (50, 50, 50);
    let third = (200, 50, 0);
    match_colours(first);
    match_colours(second);
    match_colours(third);
    }
    
    /* Nota:    
    Las sentencias match siempre se detienen cuando encuentran una coincidencia y
    no chequea el resto de los "brazos".
    */
    fn match_colours(rbg: (i32, i32, i32)) {
        match rbg {
            (r, _, _) if r < 10 => println!("No muy rojo"),
            (_, b, _) if b < 10 => println!("No muy azul"),
            (_, _, g) if g < 10 => println!("No muy verde"),
            _ => println!("Cada color tiene al menos 10"),
    }
}

//***************************************************************************** Bucles
#[allow(dead_code)]
pub fn bucles_y_control() {
    let titulo = String::from(" Bucles y control ");
    imprime_titulo(&titulo);

    let num = 2;
//************************************* IF, ELSE IF, ELSE
    if num == 2 {
        println!("Es el dos");
    } else if num == 1 {
        println!("Es el uno")
    } else {
        println!("Es un número diferente")
    }

//************************************* MATCH
    match num {
        0 => println!("Es cero"),
        1 => println!("Es uno"),
        2 => println!("Es dos"),
        _ => println!("Es otro número ")
    }

//************************************* LOOP
    let mut contador = 0;
    loop {
        contador +=1;
        println!("El contador vale ahora {}", contador);
        if contador == num {
            contador = 0;
            break;      // break también se puede usar para devolver un valor. -->
            // break contador;
        }
    }

//************************************* WHILE
    while contador < num {
        contador +=1;
        println!("El contador = {}", contador);
    }

//************************************* FOR
/* Nota:    
Un bucle for repite la ejecución un número determinado de veces.
Este tipo de bucles suele utilizar rangos, se utiliza .. y ..=
para crear un rango.

.. crea un rango excluyente: 0..3 crea un rango con los siguientes
tres números 0, 1, 2.
..= crea un rango incluyente: 0..=3 crea un rango con los siguientes
cuatro números 0, 1, 2, 3.
*/
    for _ in 0..3 {
        println!("Imprimiendo lo mismo las tres veces");
    }
}

//***************************************************************************** Option - Enum
fn identificador(id: u8) -> Option<String> {
    if id == 1 {
        return Option::Some("Id : UNO".to_string());
    } else if id == 2 {
        return Option::Some("Id : DOS".to_string());
    }
    return None;
}

//************************************* Prueba_concepto_0
#[allow(dead_code)]
pub fn option_prueba_concepto_0() {
    let titulo = String::from(" Option - Pruebas de concepto 0 ");
    imprime_titulo(&titulo);

    let identidad = match identificador(1) {
        Some(id) => id,
        None => {
            println!("Identidad no existe");
            return;
        }
    };
    println!("Identidad 1 : {}", identidad);
}
//************************************* Prueba_concepto_1
#[allow(dead_code)]
pub fn option_prueba_concepto_1() -> Option<()> {
    let titulo = String::from(" Option - Pruebas de concepto 1 ");
    imprime_titulo(&titulo);

    let identidad = identificador(2)?;
    println!("Identidad 1 : {}", identidad);
    Some(())
}

//************************************* Prueba con propio enum Opciones
enum Opciones<T> {
    None,
    Some(T),
}

fn identificador_enum_opciones(id: u8) -> Opciones<std::string::String> {
    if id == 1 {
        return Opciones::Some("Id : UNO".to_string());
    }
    return Opciones::None;
}

#[allow(dead_code)]
pub fn option_prueba_concepto_2() {
    let titulo = String::from(" Option - Pruebas de concepto 2 ");
    imprime_titulo(&titulo);

    let identidad = match identificador_enum_opciones(1) {
        Opciones::Some(id) => id,
        Opciones::None => {
            println!("Identidad no existe");
            return;
        }
    };
    println!("Identidad 1 : {}", identidad);
}
