/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          24-02-2023
    Titulo:         The Rust Standard Library
    Descripción:    The ? operator and try {} blocks.
    Referencias:    https://doc.rust-lang.org/std/ops/trait.Try.html

***************************************************************************************/

fn imprime_titulo(titulo: &String) {
    println!("\n{:*^80}", titulo);
}

//*****************************************************************************
/* Descripción:     
En este ejemplo, numbers.iter() crea un iterador que produce los elementos de numbers,
y pasamos este iterador a simple_fold junto con un valor inicial de 0 y una clausura
que suma los elementos. La función devuelve la suma total de los elementos.
*/
#[allow(dead_code)]
pub fn fn_std_ops_try_0() {
    let titulo = String::from(" Using Try in Generic Code ");
    imprime_titulo(&titulo);

    let numbers = vec![1, 2, 3, 4, 5];
    let sum = simple_fold(numbers.iter(), 0, |acc, x| acc + x);
    println!("Suma : {}", sum);       // Imprime "Sum: 15"
}

/* Descripción:     
Este código define una función llamada simple_fold que implementa una operación de
plegado (fold) simple para iteradores en Rust.

La función toma tres argumentos:
- iter: un iterador que produce elementos de tipo T.
- accum: un valor inicial de tipo A que se utilizará como acumulador.
- f: una clausura que toma un valor de tipo A y un valor de tipo T, y devuelve un
nuevo valor de tipo A. Esta clausura se aplicará sucesivamente a los elementos del
iterador, usando el valor actual del acumulador como primer argumento.

La función recorre el iterador y, en cada paso, aplica la clausura f al valor actual
del acumulador y al siguiente elemento del iterador. El resultado de la clausura se
utiliza como nuevo valor del acumulador. Al final, la función devuelve el valor final
del acumulador.
*/
fn simple_fold<A, T>(
    iter: impl Iterator<Item = T>,
    mut accum: A,
    mut f: impl FnMut(A, T) -> A,
) -> A {
    for x in iter {
        accum = f(accum, x);
    }
    accum
}

//*****************************************************************************
