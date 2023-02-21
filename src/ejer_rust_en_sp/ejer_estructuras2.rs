/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          22-02-2023
    Titulo:         introducción a RUST
    Descripción:    Funciones para HashMap y BTreeMap
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
método o para hacer referencia al tipo de la estructura o el objeto implementado
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

#[allow(dead_code)]
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
#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn metodo_1() {
    let titulo = String::from(" Método con enum ");
    imprime_titulo(&titulo);

    let mi_estado = Estado::Somnoliento;
    mi_estado.consultar();
}

//***************************************************************************** Método/función con structs y enums
#[allow(dead_code)]
#[derive(Debug)]
struct Animal {
    edad: u8,
    tipo_animal: TipoAnimal,
}

#[derive(Debug)]
enum TipoAnimal {
    Gato,
    Perro,
}

impl Animal {
    fn new() -> Self {
        // Self, aquí, significa Animal. Se podría haber usado Animal en lugar de Self

        Self {
            // Cuando se escriba Animal::new(), se obtendrá siempre un gato de 10 años
            edad: 10,
            tipo_animal: TipoAnimal::Gato,
        }
    }

    fn cambiar_a_perro(&mut self) { // como está dentro de Animal, &mut self significa &mut Animal
                                    // usa .cambiar_a_perro() para convertir el gato en un perro
                                    // con &mut self se puede modificar
        println!("¡Cambiando el animal a perro! {:?}", self.edad = 12);
        self.tipo_animal = TipoAnimal::Perro;
        //self.edad = 14;
    }

    fn cambiar_a_gato(&mut self) {
        // usa .cambiar_a_gato() para cambiar el perro a gato
        println!("¡Cambiando el animal a gato!");
        self.tipo_animal = TipoAnimal::Gato;
        self.edad = 10;
    }

    fn comprobar_tipo(&self) {
        // se lee a sí mismo self
        match self.tipo_animal {
            TipoAnimal::Perro => println!("El animal es un perro"),
            TipoAnimal::Gato => println!("El animal es un gato"),
        }
    }
}
/* Nota Importante:     
Self (el tipo Self) y self (la variable self) funcionan como abreviaturas
del tipo que sea en cada momento.

En el código anterior, Self es igual a Animal. Y en fn cambiar_a_perro(&mut self)
significa que el parámetro primero es un Animal. Este parámetro es la variable
animal_nuevo cuando se llama de la siguiente forma animal_nuevo.cambiar_a_perro().
*/


#[allow(dead_code)]
#[allow(unused_variables)]
pub fn metodo_2() {
    let titulo = String::from(" Métodos con struct y enum ");
    imprime_titulo(&titulo);

    // Función asociada para crear una variable Animal
    let mut animal_nuevo = Animal::new();   // Es un gato de 10 años

    animal_nuevo.comprobar_tipo();
    animal_nuevo.cambiar_a_perro();
    println!("Edad {}", animal_nuevo.edad);
    animal_nuevo.comprobar_tipo();
    animal_nuevo.cambiar_a_gato();
    animal_nuevo.comprobar_tipo();
    println!("Edad {}", animal_nuevo.edad);
}

//*****************************************************************************