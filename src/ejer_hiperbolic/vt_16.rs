/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          06-02-2023
    Canal youtube:  Hyperbolic Time Academy
                    https://www.youtube.com/@hyperbolictimeacademy

    Lista Repro:    Rust programming - beginners
                    https://www.youtube.com/playlist?list=PLPCT_BwWXlAPD-NA8SCn_nhMTLVxw99V4

    Video tutorial: Closures - Rust Programming Videos For Beginners - Lesson 16
                    https://www.youtube.com/watch?v=vsVL8CVGFkM

***************************************************************************************/

#[allow(dead_code)]
struct LibrosMicros{
    editorial: &'static str,
    libro: Vec<(&'static str, &'static str, u64, bool)>,
}

impl LibrosMicros{
    fn libros_get<F>(&self, f: F)
    where
        F: Fn(&Vec<(&'static str, &'static str, u64, bool)>) {
            f(&self.libro)
        }
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn vt_16() {
    // Titulo, autor, ISBN10, idioma(bool)
    let libro_intel = vec![
        ("MCS-80", "Intel", 0000000000, false)
        ];

    let libro_macmillan = vec![
        ("The Z80 Microprocessor Architecture, Interfacing, Programming and Design", "Ramesh Gaonkar", 0023404841, false),
        ];

    let libro_prentice_hall = vec![
        ("Microcomputers And Microprocessors: The 8080, 8085 and Z-80 Programming, Interfacing and Troubleshooting", "John E. Uffenbeck", 0135840619, false),
        ];

    let libro_paraninfo = vec![
        ("MICRO PROCESADADORES Arquitectura Programación y Desarrollo De Sistemas", "Jose Mª Angulo Usategui", 8428312370, true),
        ("MICRO PROCESADADORES Fundamentos Diseño y Aplicaciones", "Jose Mª Angulo Usategui", 842831148, true),
        ];

    let libro_marcombo = vec![
        ("Microprocesador Z-80, programación e interfaces (Libro2)", "Peter R. Rony,Elizabeth A. Nichols,Joseph C. Nichols", 9684100124, true),
        ("Programación del Microprocesador Z80 (Libro1)", "Peter R. Rony,Elizabeth A. Nichols,Joseph C. Nichols", 8426704085, false),
        ("Microprocesadores: Del chip al sistema", "Rodnay Zaks", 8426703909, true),
        ];    

    let edit_intel = LibrosMicros{editorial: "Intel", libro: libro_intel };
    let edit_macmillan = LibrosMicros{editorial: "Macmillan", libro: libro_macmillan };
    let dit_prentice_hall = LibrosMicros{editorial: "Prentice-Hall", libro: libro_prentice_hall };
    let edit_paraninfo = LibrosMicros{editorial: "Paraninfo", libro: libro_paraninfo };
    let edit_marcombo = LibrosMicros{editorial: "Marcombo", libro: libro_marcombo };

    edit_marcombo.libros_get(|x_closure| {
                // El tipo del vector de retorno al ser guion bajo --> <_>, la recopilación inferirá automáticamente
                // que este sería un vector de tuplas lo que necesita devolver
        //let ret: Vec<_> = x_closure.into_iter().collect();
        let ret: Vec<_> = x_closure.into_iter().filter(|x_closure|x_closure.3 == false).collect();
        println!("Resultado {:?}\n", ret);
        let ret: Vec<_> = x_closure.into_iter().map(|x_closure|format!("Titulo: {}, Castellano: {}",x_closure.0,x_closure.3)).collect();
        println!("Resultado {:?}\n", ret);
    })

/*
    for i in &libro_marcombo {
        println!("{:?}\n", libro_marcombo);
    }
*/
}