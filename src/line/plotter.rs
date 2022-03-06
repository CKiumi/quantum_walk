#[cfg(test)]
pub fn plot(prob: &[f64]) -> Result<(), Box<dyn std::error::Error>> {
    use plotters::prelude::*;
    let sum: f64 = prob.iter().sum();
    println!("{}", sum);
    let n = ((prob.len() - 1) / 2) as i32;
    let name = format!("image/stripe_{}.png", n);
    let root = BitMapBackend::new(&name, (1080, 720)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .margin(15)
        .x_label_area_size(20)
        .y_label_area_size(40)
        .build_cartesian_2d(
            -n as i32..n as i32,
            // -0f64..prob.iter().fold(0.0 / 0.0, |m, v| v.max(m)),
            -0f64..prob.iter().fold(f64::NAN, |_, _| 0.05),
        )?;
    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .x_labels(5)
        .label_style(TextStyle::from(("sans-serif", 20).into_font()))
        .draw()?;
    chart
        .draw_series(LineSeries::new(
            (0..2 * n + 1).map(|x| ((x - n), prob[x as usize])),
            &RGBColor(100, 100, 100),
        ))?
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    Ok(())
}
