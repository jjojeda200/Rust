/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          04-03-2023
    Titulo:         introducción a RUST
    Descripción:    Ejemplo avanzado de un trait que usa genéricos y manejo de memoria
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

//***************************************************************************** Trait con genéricos <T>
/* Descripción: trait Search              
Este código en Rust define un trait llamado Search con un parámetro genérico T
que describe la capacidad de buscar un valor de tipo T en algún conjunto de datos.
Luego define una estructura llamada LinearSearcher que implementa el trait Search
para realizar una búsqueda lineal en una lista de elementos de tipo T.
En resumen define un método search que toma una referencia a un valor de tipo T
y devuelve una opción que representa la posición del valor en algún tipo de
estructura de datos.
*/
trait Search<T> {
	fn search(&self, value: &T) -> Option<usize>;
}

struct LinearSearcher<T> {
	data: Vec<T>,
}

/* Descripción: trait Search                
La implementación del trait Search para un LinearSearcher, que es una estructura
que contiene un vector de valores de tipo T. La implementación de search utiliza
un ciclo for para iterar sobre el vector y encontrar la posición del valor dado.

En resumen se busca el valor de entrada value en la lista de elementos data y se
devuelve el índice de la primera aparición del valor si se encuentra, o None si
no se encuentra.
*/
impl<T: PartialEq> Search<T> for LinearSearcher<T> {
	fn search(&self, value: &T) -> Option<usize> {
    	for (i, v) in self.data.iter().enumerate() {
        	if v == value {
            	return Some(i);
        	}
    	}
    	None
	}
}

/* Descripción: ejemplo_generico_0        
La función ejemplo_generico_0 crea una instancia de LinearSearcher con una lista
de enteros del 1 al 5, y luego realiza una búsqueda en esa instancia de un valor
de 3. Si se encuentra, se imprime el índice de la primera aparición del valor en
la lista. En este caso, debería imprimir "Index of 3: 2".

Es importante destacar que Rust utiliza el concepto de ownership y borrowing
para gestionar la memoria. En este ejemplo, la propiedad del vector se pasa al
LinearSearcher en la definición de la estructura. Luego, el método search toma
una referencia al valor que se está buscando en el vector. De esta manera, Rust
asegura que la memoria se maneje de forma segura y eficiente en todo momento.
*/
#[allow(dead_code)]
pub fn ejemplo_generico_0() {
      let titulo = String::from(" Trait con genéricos 0 ");
      imprime_titulo(&titulo);

	let searcher = LinearSearcher { data: vec![1, 2, 3, 4, 5] };

      let buscar: LinearSearcher<u8> = LinearSearcher {data: vec![1, 2, 4, 8, 16, 32, 64, 128]};
      /* let buscar: LinearSearcher<u8>
      Define una variable llamada buscar que es una instancia de la estructura
      LinearSearcher con el tipo de datos u8, lo que significa que la lista de
      elementos data dentro de LinearSearcher es una lista de números enteros
      sin signo de 8 bits.
      La lista de elementos data en buscar contiene los números 1, 2, 4, 8, 16,
      32, 64 y 128. Esto significa que se puede buscar cualquier valor dentro de
      esta lista utilizando el método search definido en la implementación de
      Search para LinearSearcher.
      Por ejemplo, se podría llamar a buscar.search(&4) para buscar el valor 4 en
      la lista. Si se encuentra, search devolverá Some(2), lo que indica que 4 se
      encuentra en la posición 2 de la lista. Si el valor buscado no se encuentra
      en la lista, search devolverá None.
      */
	let mut index = searcher.search(&3).unwrap();
	println!("Index of 3: {}", index);
	index = buscar.search(&8).unwrap();
	println!("Index of 8: {}", index);
}

//***************************************************************************** Trait con genéricos y memoria
/* Descripción: trait MyTrait             
El trait MyTrait tiene un parámetro de tipo genérico T. El método do_something
toma una referencia de T y devuelve un valor booleano.
*/
trait MyTrait<T> {
      fn do_something(&self, data: &T) -> bool;
}

/* Descripción: struct MyStruct<'a>       
La estructura MyStruct tiene un campo de referencia de cadena (&str). También se
usa el manejo de memoria de Rust a través de la declaración de una duración de vida
('a) para el campo de referencia data en la estructura MyStruct. Esto indica que la
referencia debe tener una duración de vida al menos igual a la de la estructura.
*/
struct MyStruct<'a> {
      data: &'a str,      
}

/* Descripción: impl<'a> MyTrait<&'a str> 
La declaración impl<'a> MyTrait<&'a str> for MyStruct<'a> define una implementación
genérica del trait MyTrait para una estructura MyStruct que tiene un lifetime 'a.
La cláusula 'a en la definición de impl indica que esta implementación es genérica
sobre un lifetime arbitrario, lo que significa que el lifetime de la referencia &'a
str que se utiliza en la definición de MyTrait puede ser diferente para cada instancia
de MyStruct. Esta flexibilidad es útil cuando se trabaja con tipos que tienen referencias
a datos con diferentes vidas útiles.
La función do_something implementa el método requerido por el trait MyTrait. Toma una
referencia a una referencia de &'a str como argumento, que es una forma de indicar que
la referencia interna tiene un lifetime igual al lifetime de MyStruct. La función
compara self.data (que es una referencia a &'a str almacenada en MyStruct) con la
referencia que se pasa como argumento (*data). Devuelve true si las dos referencias
apuntan a la misma ubicación de memoria, lo que significa que contienen el mismo valor.
En resumen, esta implementación genérica del trait MyTrait para MyStruct permite trabajar
con referencias a datos con diferentes vidas útiles y define un método do_something que
compara dos referencias de &'a str.
*/
impl<'a> MyTrait<&'a str> for MyStruct<'a> {
      /*
      En Rust, una referencia a un valor se denota con el símbolo &, y un tipo de
      referencia a un valor de tipo T se escribe como &T. Además, se pueden tener
      referencias a otras referencias, lo que se conoce como referencias anidadas
      o punteros dobles.
      En la declaración fn do_something(&self, data: &&'a str) -> bool, el parámetro
      data es una referencia anidada a un &'a str. Esto significa que data es una
      referencia que apunta a otra referencia que apunta a un valor de tipo str, y el
      lifetime de la referencia más interna (la referencia a str) está especificado
      por el lifetime 'a.
      El propósito de la referencia doble es permitir que la función compare la referencia
      interna con otra referencia externa que tenga el mismo lifetime, como la referencia
      self.data de MyStruct. En otras palabras, data es una referencia que apunta a la misma
      ubicación de memoria que self.data, pero con un nivel de indirección adicional.
      */
      fn do_something(&self, data: &&'a str) -> bool {
            self.data == *data
      }
}

/* Descripción: ejemplo_generico_memoria  
En la función ejemplo_generico_memoria, se crean dos cadenas s1 y s2, y se crea una
instancia de MyStruct que tiene s1 como su campo data. Luego se realizan dos
comprobaciones usando el método do_something de MyTrait: la primera con una referencia
a s1 y la segunda con una referencia a s2. La primera comprobación devuelve true,
mientras que la segunda devuelve false.
*/
#[allow(dead_code)]
pub fn ejemplo_generico_memoria() {
      let titulo = String::from(" Trait con genéricos y acceso a memoria ");
      imprime_titulo(&titulo);

      let s1 = "hello";
      let s2 = "world";
      let my_struct = MyStruct { data: s1 };
      println!("{}", my_struct.do_something(&s1));
      assert!(my_struct.do_something(&s1));
      println!("{}", my_struct.do_something(&s2));
      assert!(!my_struct.do_something(&s2));
}

