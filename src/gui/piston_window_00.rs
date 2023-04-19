/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          19-04-2023
    Titulo:         pruebas de piston_window
    Descripción:    
    Referencias:    https://github.com/pistondevelopers/piston_window

***************************************************************************************/
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

//extern crate piston_window;

use piston_window::*;

pub fn piston_win_00() {
 
    let title = "Hello Piston! (press any key to enter inner loop)";
    let mut window: PistonWindow = WindowSettings::new(title, [200, 200])
        .resizable(false)
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    window.set_lazy(true);
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear([0.5, 1.0, 0.5, 1.0], g);
            rectangle(
                [1.0, 0.0, 0.0, 1.0],
                [50.0, 50.0, 100.0, 100.0],
                c.transform,
                g,
            );
        });

        if e.press_args().is_some() {
            InnerApp {
                title: "Inner loop (press X to exit inner loop)",
                exit_button: Button::Keyboard(Key::X),
            }
            .run(&mut window);
            window.set_title(title.into());
        }
    }
}

/// Stores application state of inner event loop.
pub struct InnerApp {
    pub title: &'static str,
    pub exit_button: Button,
}

impl InnerApp {
    pub fn run(&mut self, window: &mut PistonWindow) {
        window.set_title(self.title.into());
        while let Some(e) = window.next() {
            window.draw_2d(&e, |c, g, _| {
                clear([0.5, 0.5, 1.0, 1.0], g);
                ellipse(
                    [1.0, 0.0, 0.0, 1.0],
                    [50.0, 50.0, 100.0, 100.0],
                    c.transform,
                    g,
                );
            });
            if let Some(button) = e.press_args() {
                if button == self.exit_button {
                    break;
                }
            }
        }
    }
}