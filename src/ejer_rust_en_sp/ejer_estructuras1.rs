/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          28-03-2023
    Titulo:         introducción a RUST
    Descripción:    Funciones para structs y enums
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

fn imprime_titulo(titulo: &String) {
    println!("\n{:*^80}", titulo);
}

//***************************************************************************** Método Self y self
/* Notas Métodos:       
Los struct y enum permiten funciones asociadas a los tipos definidos,  utiliza
el bloque impl. A estas funciones se las llama métodos.
En un bloque impl se pueden definir dos tipos diferentes de métodos:
- Métodos: toman como primer parámetro uno denominado self (o &self o &mut self).
  Para utilizarlos, usan . (un punto) sobre una variable del tipo struct o enum.
  Por ejemplo, x.clone() es un método del tipo de la variable x.

- Funciones asociadas (al tipo). No tienen el primer parámetros self. Son funciones
  "relacionadas con el tipo de datos". Se llaman utilizando ::.
  Por ejemplo: String::from() y Vec::new() son llamadas a funciones asociada.
  Normalmente se utilizan para crear valores de variables del tipo correspondiente.
*/
/* Notas Self/self:     
Self y self son dos identificadores diferentes que se utilizan en diferentes contextos.

Self ese utiliza para hacer referencia al tipo de la estructura o el objeto en el
que se encuentra el método. Se utiliza para especificar el tipo de retorno de un
método (normalmente una instancia) o para hacer referencia al tipo de la estructura o el objeto implementado
de un trait.

self se utiliza para hacer referencia al objeto o estructura en la que se está
ejecutando un método. Se utiliza como el primer parámetro de los métodos de la
estructura o el objeto. Por ejemplo, en el siguiente código, self se utiliza para
referirse al objeto en el que se está ejecutando el método some_method():
*/

#[derive(Debug)]
struct Estructura {
    tipo_x: u8,
}

/*
Aquí, new es un método de Foo que devuelve una instancia de Foo,
y get_x es un método que devuelve el valor de x, y add_x modifica el valor de x
**** Self se refiere al tipo struct "Estructura"
**** self se refiere al objeto "objeto_x"
*/
impl Estructura {
    // fn new(x: u8) -> Self { Self { x } }
    // vemos como sustituye Self a Estructura
    fn new(tipo_x: u8) -> Estructura { Estructura { tipo_x } }  
        
    fn get_tipo_x(&self) -> u8 { self.tipo_x }

    fn add_tipo_x(&mut self, var_y: u8) { self.tipo_x += var_y; }
}

pub fn metodo_0() {
    let titulo = String::from(" ¡Self / self! ");
    imprime_titulo(&titulo);

    let mut objeto_x = Estructura::new(10);
    println!("objeto_x.tipo_x contiene: {:?}", objeto_x.tipo_x);
    let var_y = Estructura::get_tipo_x(&objeto_x);
    println!("var_y contiene: {:?}", var_y);
    Estructura::add_tipo_x(&mut objeto_x, var_y);
    println!("objeto_x.tipo_x contiene: {:?}", objeto_x.tipo_x);
}


//***************************************************************************** Método/función con enums
enum Estado {
    Bueno,
    Malo,
    Somnoliento,
}

impl Estado {
    fn consultar(&self) {
        match self {
            Estado::Bueno => println!("¡Me siento bien!"),
            Estado::Malo => println!("Eh, no me siento tan bien"),
            Estado::Somnoliento => println!("Necesito dormir AHORA"),
        }
    }
}

pub fn metodo_1() {
    let titulo = String::from(" Método con enum ");
    imprime_titulo(&titulo);

    let mi_estado = Estado::Somnoliento;
    mi_estado.consultar();
}

//***************************************************************************** Detalles de Self y self 
/* Nota Importante:     
Self (el tipo Self) y self (la variable self) funcionan como abreviaturas
del tipo que sea en cada momento.

En este ejemplo, tanto self como Self se utilizan en los métodos doble_tamaño,
invertir y nuevo. self se utiliza para referirse a una instancia a la estructura
actual (Rectangulo) y Self se utiliza como un tipo de retorno implícito para que
se pueda devolver, un nuevo objeto (instancia) de la estructura con los valores
calculados en el método.
Indistintamente se puede usar Self o el nombre de la estructura, en este caso
Rectangulo.
*/

#[derive(Debug)]
struct Rectangulo {
    ancho: u32,
    alto: u32,
}

impl Rectangulo {
    // Devuelve un nuevo Rectangulo, indistintamente se puede usar Self o Rectangulo
    fn doble_tamaño(&self) -> Rectangulo {
        Rectangulo {
            ancho: self.ancho * 2,
            alto: self.alto * 2,
        }
    }
    
    // Devuelve un nuevo Rectangulo, indistintamente se puede usar Self o Rectangulo
    fn invertir(&self) -> Self {
        Self {
            ancho: self.alto,
            alto: self.ancho,
        }
    }
    
    fn area(&self) -> u32 {
        self.ancho * self.alto
    }
    
    // Devuelve el área del Rectangulo,  se puede utilizar Self en lugar de Option<u32>
    // ¡¡¡En las pruebas no funciona!!! 
    // fn area_1 (&self ) -> Self {
    fn area_1 (&self) -> Option<u32> {
        if self.ancho == 0 || self.alto == 0 {
            // Self::None
            None
        } else {
            // Self::Some(self.ancho * self.alto)::Option<u32>
            Some(self.ancho * self.alto)
        }
    }

    // Método que crea un nuevo Rectangulo con el ancho y alto dados
    fn nuevo(ancho: u32, alto: u32) -> Self /*o Rectangulo*/ {
        Rectangulo /*o Self*/ {
            ancho,
            alto,
        }
    }
}

pub fn metodo_2() {
    let titulo = String::from(" Detalles de Self y self ");
    imprime_titulo(&titulo);

    let r = Rectangulo::nuevo(10, 20);
    println!("Rectangulo original: {} x {} = {:?}", r.ancho, r.alto, r.area());
    
    let r_doble = r.doble_tamaño();
    println!("Rectangulo doble tamaño: {} x {} = {:?}", r_doble.ancho, r_doble.alto, r_doble.area());
    
    let r_invertido = r.invertir();
    println!("Rectangulo invertido: {} x {} = {:?}", r_invertido.ancho, r_invertido.alto, r_invertido.area());

    let area = r.area_1();
    match area {
        Some(valor) => println!("El área del rectángulo es {}", valor),
        None => println!("El rectángulo no tiene área"),
    }
}

//*****************************************************************************