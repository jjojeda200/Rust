/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          31-01-2023
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

fn imprime_titulo(titulo: &String) {
      println!("\n{:*^80}", titulo);
  }
  
/*

*/


// Objeto(Clase) y sus Atributos
struct Triangulo
{
      base: f64,
      altura: f64
}
// Métodos(Acciones)
impl Triangulo
{
      // Debe ser referencia mutable
      fn set_base(&mut self, nueva_base: f64)
      {
            self.base = nueva_base;
      }
      fn set_altura(&mut self, nueva_altura: f64)
      {
            self.altura = nueva_altura;
      }
}

// Métodos(Acciones). El cómo se puede hacer del el trait
impl Figura for Triangulo
{
      fn area(&self) -> f64
      {
            (self.base * self.altura) / 2.0
      }
}

// Equivalente/sustituto al Constructor
// Que se puede hacer, no el cómo (Interfaz en otros lenguajes)
trait Figura {
    fn area(&self) -> f64;
}

#[allow(dead_code)]
pub fn ejemplo_poo_0() {
      let titulo = String::from(" Ejemplo de POO ");
      imprime_titulo(&titulo);

      // Instanciar objeto
      let mut tri = Triangulo
      {
            base: 10.0,
            altura: 20.0
      };

      println!("Base = {}, Altura = {}", tri.base, tri.altura);
      println!("El área del triangulo es       = {}", tri.area());
      tri.base = 15.0;
      println!("El área nueva del triangulo es = {}", tri.area());
      tri.set_base(20.0);
      println!("El área nueva del triangulo es = {}", tri.area());
      tri.set_altura(30.0);
      println!("El área nueva del triangulo es = {}", tri.area());
}