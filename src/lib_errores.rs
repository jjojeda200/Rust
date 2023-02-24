/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          12-02-2023
    Titulo:         introducción a RUST
    Descripción:    Funciones de prueba de control de errores, result, option, match de Rust
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

//*****************************************************************************
enum Opciones<T> {
    None,
    Some(T),
}

fn identificador(id:u8) -> Opciones<std::string::String>{
    if id == 1 {
        return Opciones::Some("Id : UNO".to_string());
    }
    return Opciones::None;
}

#[allow(dead_code)]
pub fn enum_opciones() {
    let identidad = match identificador(1){
        Opciones::Some(id) => id,
        Opciones::None => {
            println!("Identidad no existe");
            return
        }        
    };
    println!("Identidad 1 : {}", identidad);
}

//*****************************************************************************
// https://www.youtube.com/watch?v=y3wUCb-uS3g&t=495s
// Usados en fn de gestión de errores
use std::fs::File;
use std::io::ErrorKind;

/* Nota: 
El operador ? es un atajo para match que devuelve el error si Result es Err,
o devuelve el valor de Ok si Result es Ok.
*/

#[allow(dead_code)]
pub fn gestion_error(){


//********************************* "panic" si el fichero no existe
/* 
    let titulo = " \"panic\" si fichero no existe ";    
    println!("{:*^80}", titulo);    // sin variable, relleno con *, centrado, longitud de 80 caracteres

    let fd_result = File::open("ErrorLog.txt");

    let fd = match fd_result {
       Result::Ok(fichero) => fichero,
       Result::Err(error) => panic!("Problema abriendo el archivo {:?}", error),
    };
    println!("Resultado: {:?}", fd);
*/

//********************************* Mensaje si fichero no existe o sin permiso
/* Nota: 
Abrir un archivo llamado "errorLog.txt" utilizando la función open de la biblioteca std::fs.
Si la apertura del archivo es exitosa, se guarda el archivo en la variable file.
De lo contrario, se compara el tipo de error con los diferentes valores de ErrorKind
para determinar cómo responder. Se usa una función match para manejar los errores,
lo que permite tener un control detallado de los diferentes tipos de errores
que pueden ocurrir.
*/
/*
let titulo = " Mensaje si fichero no existe o sin permiso ";
    println!("{:*^80}\n", titulo);

    let fd_result = File::open("ErrorLog.txt");

    let fd = match fd_result {
        Result::Ok(fichero) => fichero,
        Result::Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => println!("El archivo no existe"),
                std::io::ErrorKind::PermissionDenied => println!("Permiso denegado para el archivo"),
                _ => println!("Otro error: {:?}", error),
            }
            return;
        }
    };
    println!("Resultado: {:?}", fd);
*/

//********************************* Crea archivo si no existe, \"panic\" para otros errores
/*
    let titulo = " Crea archivo si no existe, \"panic\" para otros errores ";
    println!("{:*^80}\n", titulo);

    let fd_result = File::open("ErrorLog.txt");

    let fd = match fd_result {
        Result::Ok(fichero) => fichero,
        Result::Err(error) => match error.kind()
        {
            ErrorKind::NotFound => match File::create("ErrorLog.txt")
            {
                Ok(fichero_creado) => fichero_creado,
                Err(error) => panic!("Problema creando el archivo {:?}", error),
            },
            ErrorKind::PermissionDenied => match File::create("ErrorLog.txt") {
                Ok(fichero_creado) => fichero_creado,
                Err(error) => panic!("Problema permisos acceso al archivo denegado {:?}", error),
            },
            otros_errores => { panic!("Problema xxxx el archivo {:?}", otros_errores) }
        },
    };
    println!("Resultado: {:?}", fd);
*/

//********************************* Método con IF (Crea archivo si no existe, \"panic\" para otros errores)
/* Nota: 
    Es importante notar que también se pueden utilizar otras estructuras de 
    control de flujo, como if let o while let, para lograr el mismo objetivo.
 */

    let titulo = " Método con IF (Crea archivo si no existe, \"panic\" para otros errores) ";
    println!("{:*^80}\n", titulo);

    let fd_result = File::open("ErrorLog.txt").unwrap_or_else(|error|{
        if error.kind() == ErrorKind::NotFound {
            File::create("ErrorLog.txt").unwrap_or_else(|error|{
                panic!("Problema creando el archivo {:?}", error);
            })
        } else if error.kind() == ErrorKind::PermissionDenied {
            println!("Error: {}", error.kind());
            panic!("Problema permiso acceso al archivo {:?}", error);
        } else {
            println!("Error sin definir ");
            panic!("Problema con el archivo {:?}", error);
        }
    });
    println!("Resultado: {:?}", fd_result);

    // Otra forma de presentar un mensaje de error con panic -->
    // let fd = File::open("Error.txt").expect("Fallo abriendo otro archivo");
//*************************************
}


/*
Leer el contenido del archivo y guardarlo en la variable contents. Si la lectura es ok,
se imprime el contenido del archivo. De lo contrario, se imprime un mensaje de error
que indica que hubo un problema al leer el archivo.

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => println!("El contenido del archivo es: {}", contents),
        Err(error) => println!("Error al leer el archivo: {:?}", error),
    }
*/

//*****************************************************************************