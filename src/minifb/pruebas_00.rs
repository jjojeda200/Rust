/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          11-04-2023
    Titulo:         pruebas de minifb
    Descripción:    
    Referencias:    https://github.com/emoon/rust_minifb

***************************************************************************************/
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_imports)]

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

pub fn inicio() {
      let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

      let mut window = Window::new(
          "Test - ESC to exit",
          WIDTH,
          HEIGHT,
          WindowOptions::default(),
      )
      .unwrap_or_else(|e| {
          panic!("{}", e);
      });
  
      while window.is_open() && !window.is_key_down(Key::Escape) {
          for i in buffer.iter_mut() {
              *i = 4; // color negro
          }
  
          // Aquí puedes dibujar en el buffer
  
          window
              .update_with_buffer(&buffer, WIDTH, HEIGHT)
              .unwrap();
      }
}

pub fn flags() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "8080 Flags Calculator - Rust + Minifb",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let a: u8 = 0b00001111; // Primer operando
    let b: u8 = 0b00000001; // Segundo operando
    let carry: bool = false; // Bandera de acarreo inicial
    let result: u8 = add(a, b, carry);

    let zero: bool = result == 0;
    let sign: bool = result & 0b1000_0000 != 0;
    let parity: bool = result.count_ones() % 2 == 0;
    let carry: bool = (a as u16 + b as u16 + carry as u16) > 0xFF;
    let auxiliary_carry: bool = (a & 0x0F) + (b & 0x0F) + carry as u8 > 0x0F;

    println!("Zero: {}", zero);
    println!("Sign: {}", sign);
    println!("Parity: {}", parity);
    println!("Carry: {}", carry);
    println!("Auxiliary carry: {}", auxiliary_carry);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in 0..buffer.len() {
            buffer[i] = 0; // Limpiamos el buffer
        }

        // Dibujamos los resultados de los flags en la ventana
        draw_flag(&mut buffer, zero, 50, 50);
        draw_flag(&mut buffer, sign, 150, 50);
        draw_flag(&mut buffer, parity, 250, 50);
        draw_flag(&mut buffer, carry, 350, 50);
        draw_flag(&mut buffer, auxiliary_carry, 450, 50);

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

// Función que realiza la operación add y actualiza los flags correspondientes
fn add(a: u8, b: u8, carry: bool) -> u8 {
    let result: u16 = a as u16 + b as u16 + carry as u16;

    let zero: bool = result as u8 == 0;
    let sign: bool = result as u8 & 0b1000_0000 != 0;
    let parity: bool = result.count_ones() % 2 == 0;
    let carry: bool = result > 0xFF;
    let auxiliary_carry: bool = (a & 0x0F) + (b & 0x0F) + carry as u8 > 0x0F;

    println!("a: {:08b}, b: {:08b}", a, b);
    println!("carry: {}", carry);

    println!("result: {:08b}", result as u8);
    println!("Zero: {}", zero);
    println!("Sign: {}", sign);
    println!("Parity: {}", parity);
    println!("Carry: {}", carry);
    println!("Auxiliary carry: {}", auxiliary_carry);

    // Devolvemos el resultado de la operación add
    result as u8
}

// Función que dibuja un flag en la posición (x, y) del buffer
fn draw_flag(buffer: &mut Vec<u32>, value: bool, x: usize, y: usize) {
    let color = if value { 0xFFFFFF } else { 0x000000 };

    for i in x..x + 50 {
        for j in y..y + 50 {
            buffer[i + j * 640] = color;
        }
    }
}
