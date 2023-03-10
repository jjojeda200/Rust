/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          04-03-2023
    Titulo:         introducción a RUST
    Descripción:    Ejemplo básico de implementación de objeto en Rust
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

fn imprime_titulo(titulo: &String) {
    println!("\n{:*^80}", titulo);
}

//*****************************************************************************
/* Descripción:
Este código de Rust define tres estructuras: Monstruo, Mago y Cazador. Además,
define dos rasgos (traits) de lucha: LuchaCercana y LuchaADistancia.

La estructura Monstruo tiene un solo campo, salud, que es un número entero con
signo de 32 bits (i32). Las estructuras Mago y Cazador también tienen un campo
de salud, pero no tienen más campos.
*/
struct Monstruo {
    salud: i32,
}
#[derive(Debug)]
struct Mago {
    salud: i32,
}
#[derive(Debug)]
struct Cazador {
    salud: i32,
}

/* Descripción: LuchaCercana              
El rasgo LuchaCercana define dos métodos: atacar_con_espada y atacar_con_la_mano.
Ambos métodos toman una referencia mutable a un Monstruo como parámetro y reducen
su salud en una cantidad específica. Además, ambos métodos imprimen un mensaje por
la consola que indica el resultado del ataque.
*/
trait LuchaCercana: std::fmt::Debug {
    // Ahora el tipo necesita tener Debug para poder implementar LuchaCercana
    fn atacar_con_espada(&self, oponente: &mut Monstruo) {
        oponente.salud -= 10;
        println!(
            "Atacaste con espada. Tu oponente tiene ahora {} de salud. Ahora tienes {:?}",
            // se puede usar {:?} porque se dispone de Debug
            oponente.salud,
            &self
        );
    }
    fn atacar_con_la_mano(&self, oponente: &mut Monstruo) {
        oponente.salud -= 2;
        println!(
            "Atacaste con la mano. tu oponente tiene ahora {} de salud. Ahora tienes {:?}",
            oponente.salud, &self
        );
    }
}
impl LuchaCercana for Mago {}
impl LuchaCercana for Cazador {}

/* Descripción: LuchaADistancia           
El rasgo LuchaADistancia define dos métodos: atacar_con_arco y atacar con_piedra.
Ambos métodos toman una referencia mutable a un Monstruo y un número entero sin
signo (u32) que representa la distancia entre el cazador y el monstruo. Si la
distancia es menor que un valor específico, el método reduce la salud del monstruo
y emite un mensaje en la consola.
*/
trait LuchaADistancia: std::fmt::Debug {
    // también se podría haber escrito LuchaADistancia: LuchaCercana
    // porque LuchaCercana implementa ya Debug
    fn atacar_con_arco(&self, oponente: &mut Monstruo, distancia: u32) {
        if distancia < 10 {
            oponente.salud -= 10;
            println!(
                "Atacaste con el arco. Tu oponente tiene ahora {} de salud. Ahora tienes {:?}",
                oponente.salud, &self
            );
        }
    }
    fn atacar_con_piedra(&self, oponente: &mut Monstruo, distancia: u32) {
        if distancia < 3 {
            oponente.salud -= 4;
        }
        println!(
            "Atacaste con una piedra. Tu oponente tiene ahora {} de salud. Ahora tienes {:?}",
            oponente.salud, &self
        );
    }
}
impl LuchaADistancia for Cazador {}

/* Descripción: ejemplo_poo_0()           
Crea un Mago y un Cazador con una cierta cantidad de salud y un Monstruo con
una cantidad inicial de salud. La función main llama a los métodos de lucha
en los objetos Mago y Cazador y pasa el objeto Monstruo como parámetro.
*/
pub fn ejemplo_poo_0() {
    let titulo = String::from(" Ejemplo de POO 0 ");
    imprime_titulo(&titulo);

    let radagast = Mago { salud: 60 };
    let aragorn = Cazador { salud: 80 };
    let mut uruk_hai = Monstruo { salud: 40 };

    radagast.atacar_con_espada(&mut uruk_hai);
    aragorn.atacar_con_arco(&mut uruk_hai, 8);
}

//*****************************************************************************
use std::fmt::Debug;    // para no tener que escribir todo el camino al rasgo

struct Monstruo1 {
    salud: i32,
}
#[derive(Debug)]
struct Mago1 {
    salud: i32,
}
#[derive(Debug)]
struct Cazador1 {
    salud: i32,
}

trait Magia1{}         // todos los traits sin métodos. Son límites de rasgo

trait LuchaCercana1 {}
trait LuchaADistancia1 {}

impl LuchaCercana1 for Cazador1{}       // Cada tipo tiene LuchaCercana,
impl LuchaCercana1 for Mago1 {}
impl LuchaADistancia1 for Cazador1{}    // Pero solo el cazador lucha a distancia
impl Magia1 for Mago1{}                 // y solo el mago hace magia

/* Descripción: atacar_con_flecha         
La función atacar_con_flecha acepta un tipo de personaje que implementa el rasgo
LuchaADistancia y tiene una distancia de ataque menor a 10. Esta función reduce la
salud del oponente en 10 y muestra un mensaje indicando la salud del oponente y el
personaje que realizó el ataque.
*/
fn atacar_con_flecha1<T: LuchaADistancia1 + Debug>(personaje: &T, oponente: &mut Monstruo1, distancia: u32) {
    if distancia < 10 {
        oponente.salud -= 10;
        println!(
            "Atacaste con el arco. Tu oponente tiene ahora {} de salud. Ahora tienes {:?}",
            oponente.salud, personaje
        );
    }
}

/* Descripción: atacar_con_espada         
La función atacar_con_espada acepta un tipo de personaje que implementa el rasgo
LuchaCercana. Esta función reduce la salud del oponente en 10 y muestra un mensaje
indicando la salud del oponente y el personaje que realizó el ataque.
*/
fn atacar_con_espada1<T: LuchaCercana1 + Debug>(personaje: &T, oponente: &mut Monstruo1) {
    oponente.salud -= 10;
    println!(
        "Atacaste con la espada. Tu oponente tiene ahora {} de salud. Ahora tienes {:?}",
        oponente.salud, personaje
    );
}

/* Descripción: atacar_con_bola_de_fuego  
La función atacar_con_bola_de_fuego acepta un tipo de personaje que implementa el
rasgo Magia y tiene una distancia de ataque menor a 15. Esta función reduce la salud
del oponente en 20 y muestra un mensaje indicando la salud del oponente y el personaje
que realizó el ataque.
*/
fn atacar_con_bola_de_fuego1<T: Magia1 + Debug>(personaje: &T, oponente: &mut Monstruo1, distancia: u32) {
    if distancia < 15 {
        oponente.salud -= 20;
        println!("¡Levantas las manos y conjuras una bola de fuego! Tu oponente tiene ahora {} de salud. Ahora tienes: {:?}",
    oponente.salud, personaje);
    }
}
/* Resumen:                               
Cada función tiene un rasgo diferente como límite de tipo y/o una distancia de ataque
diferente, lo que les permite aceptar diferentes tipos de personajes y realizar
diferentes tipos de ataques.
*/

pub fn ejemplo_poo_1() {
    let titulo = String::from(" Ejemplo de POO 1 ");
    imprime_titulo(&titulo);

    let radagast = Mago1 { salud: 60 };
    let aragorn = Cazador1 { salud: 80 };

    let mut uruk_hai = Monstruo1 { salud: 40 };

    atacar_con_espada1(&radagast, &mut uruk_hai);
    atacar_con_flecha1(&aragorn, &mut uruk_hai, 8);
    atacar_con_bola_de_fuego1(&radagast, &mut uruk_hai, 8);
}

//*****************************************************************************