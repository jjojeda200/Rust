use plotters::prelude::*;

fn draw_pin_timings(pins: &[u8], timings: &[u16], file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(file_path, (640, 480)).into_drawing_area();

    root.fill(&WHITE)?;

    let pins_str: Vec<String> = pins.iter().map(|p| format!("{:02X}", p)).collect();
    let timings_f64: Vec<f64> = timings.iter().map(|t| *t as f64).collect();

    let mut chart = ChartBuilder::on(&root)
        .caption("Timing diagram", ("Arial", 20).into_font())
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0.0..timings_f64[timings_f64.len() - 1] as f32, 0.0..pins.len() as f32)?;

    chart.configure_mesh().draw()?;

    for i in 0..pins.len() {
        let pin_str = &pins_str[i];
        let timings_slice: Vec<f64> = timings_f64.iter().map(|t| *t as f64).collect();
        let mut timings_points: Vec<(f32, f32)> = Vec::new();

        for (j, timing) in timings_slice.iter().enumerate() {
            if j == 0 {
                timings_points.push((0.0, i as f32));
            } else {
                timings_points.push((timing as f32, i as f32));
                timings_points.push((timing as f32, i as f32 + 1.0));
                timings_points.push((timings_slice[j - 1] as f32, i as f32 + 1.0));
                timings_points.push((timings_slice[j - 1] as f32, i as f32));
            }
        }

        chart.draw_series(std::iter::once(Polygon::new(
            timings_points,
            colors::Color::from_rgb(0, 0, 255).mix(0.2),
        )))?
        .label(pin_str)
        .legend(move |(x, y)| {
            Rectangle::new([(x - 5, y - 5), (x + 5, y + 5)], colors::Color::from_rgb(0, 0, 255).mix(0.2))
        });
    }

    chart.configure_series_labels().border_style(&BLACK).draw()?;

    Ok(())
}



fn draw_pin_timings1(pins: &[u8], timings: &[u16]) {
    let mut last_timing = timings[0];
    for i in 0..pins.len() {
        let timing = timings[i + 1];
        let duration = timing - last_timing;
        last_timing = timing;

        let pin_str = format!("{:02X}", pins[i]);
        let timing_str = format!("{:04X}", duration);

        println!("Pin {} changed after {} clock cycles", pin_str, timing_str);
    }
}


pub fn pru_ploter() {
    // Simulamos algunos cambios en los pines y los tiempos en ciclos de reloj
    let pins = [0x01, 0x03, 0x07, 0x0F];
    let timings = [0x0000, 0x0012, 0x0025, 0x003A, 0x0051];

    // Dibujamos los tiempos de los pines
    draw_pin_timings(&pins, &timings);
}

//*****************************************************************************