/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          02-03-2023
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

    Dependencias:

    Compilar:

    Licencia:

***************************************************************************************/

fn imprime_titulo(titulo: &String) {
      println!("\n{:*^80}", titulo);
  }

/* Definición objeto versus Instancia:    
En Rust, una estructura (struct) y un objeto (object) se refieren al mismo
concepto: un tipo de datos que puede contener varios campos con diferentes
tipos de datos. En otras palabras, una estructura u objeto es una plantilla
para crear instancias específicas de ese tipo de datos.

Una instancia (instance), por otro lado, se refiere a un objeto concreto creado
a partir de una estructura u objeto en particular. Cada instancia tiene sus
propios valores para los campos definidos en la estructura u objeto.

En resumen, una estructura u objeto en Rust es una descripción abstracta de un
tipo de datos, mientras que una instancia es una realización concreta de esa
descripción que tiene valores específicos para cada campo.
*/

/* Definición Métodos:                    
Los métodos son funciones asociadas a una estructura o un tipo de datos (Objeto) en particular.
Los métodos se dividen en dos tipos:
      * Métodos de instancia: son métodos que se aplican a una instancia particular
        de una estructura u objeto. Se llaman usando el operador ".".  Estos métodos
        se definen dentro del bloque de implementación de la estructura o el objeto
        y utilizan la palabra clave self para hacer referencia a la instancia en la
        que se están aplicando.

      * Métodos estáticos:    son métodos que se aplican a la estructura u objeto en
        sí misma, en lugar de a una instancia particular. Se llaman usando el operador "::".
        Estos métodos se definen también dentro del bloque de implementación, pero
        utilizan la palabra clave Self en lugar de self para hacer referencia a la
        estructura u objeto.

En resumen, los métodos de instancia se aplican a una instancia particular de una
estructura u objeto, mientras que los métodos estáticos se aplican a la estructura u
objeto en sí misma. Ambos tipos de métodos se definen dentro del bloque de implementación
y utilizan palabras clave diferentes para hacer referencia a la estructura u objeto.
*/

//*****************************************************************************
/* Ejemplos métodos de instancia:         
Se definen usando la palabra clave "impl" seguida del nombre de la estructura o
tipo de datos y luego el método.

En este ejemplo, se define una estructura Persona con un método de instancia
presentarse. El método toma una referencia a self como parámetro, lo que permite
acceder a los campos de la estructura.
*/
struct Persona {
    nombre: String,
    edad: u8,
}

impl Persona {
    fn presentarse(&self) {
        println!("Hola, mi nombre es {} y tengo {} años.", self.nombre, self.edad);
    }
}

#[allow(dead_code)]
pub fn ejemplo_instancia_0() {
      let titulo = String::from(" Ejemplo método de instancia 0 ");
      imprime_titulo(&titulo);
  
    let persona = Persona {
        nombre: String::from("Juan"),
        edad: 25,
    };
    persona.presentarse();    // Salida: Hola, mi nombre es Juan y tengo 25 años.
}
/*
En este ejemplo, area() es un método de instancia que se define dentro del bloque
de implementación de la estructura Rectangle. El método toma una referencia a self
y devuelve el área del rectángulo.
*/
struct Rectangle {
      width: u32,
      height: u32,
}

impl Rectangle {
      fn area(&self) -> u32 {
            self.width * self.height
      }
}

#[allow(dead_code)]
pub fn ejemplo_instancia_1() {
      let titulo = String::from(" Ejemplo método de instancia 1 ");
      imprime_titulo(&titulo);

      let rect = Rectangle { width: 10, height: 20 };
      println!("Area: {}", rect.area());
}
  

//*****************************************************************************
/* Ejemplos métodos estáticos:            
Los métodos estáticos se definen usando la palabra clave "impl" seguida del nombre
de la estructura o tipo de datos y luego la palabra clave "fn" con el nombre del
método.

En este ejemplo, se define una estructura Calculadora con un método estático sumar.
El método no toma una referencia a self como parámetro, sino que toma los parámetros
a y b directamente. El método se llama usando el nombre de la estructura seguido del
doble dos puntos (::) y luego el nombre del método.
*/
struct Calculadora;

impl Calculadora {
    fn sumar(a: i32, b: i32) -> i32 {
        a + b
    }
}

#[allow(dead_code)]
pub fn ejemplo_estatico_0() {
      let titulo = String::from(" Ejemplo método estático 0 ");
      imprime_titulo(&titulo);

    let resultado = Calculadora::sumar(2, 3);
    println!("El resultado es {}", resultado); // Output: El resultado es 5
}

/*
En este ejemplo, pi() es un método estático que devuelve el valor de pi. area() es
un método de instancia que utiliza el método estático pi() para calcular el área del
círculo. Para hacer referencia al método estático desde un método de instancia, se
utiliza Self:: en lugar de self.
*/
struct Circle {
      radius: f64,
  }
  
  impl Circle {
      fn pi() -> f64 {
          3.14159265359
      }
  
      fn area(&self) -> f64 {
          Self::pi() * (self.radius * self.radius)
      }
  }
  
#[allow(dead_code)]
  pub fn ejemplo_estatico_1() {
      let titulo = String::from(" Ejemplo método estático 1 ");
      imprime_titulo(&titulo);

      let circle = Circle { radius: 10.0 };
      println!("Area: {}", circle.area());
}

