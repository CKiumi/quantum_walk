#![cfg(not(target_arch = "wasm32"))]
use plotters::prelude::*;
pub fn plot_1d(prob: &[f64], dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let n = ((prob.len() - 1) / 2) as i32;
    let root = BitMapBackend::new(dir, (3596, 2360)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .margin(170)
        .x_label_area_size(20)
        .y_label_area_size(40)
        .build_cartesian_2d(
            -n as i32..n as i32,
            -0f64..prob.iter().fold(f64::NAN, |m, v| v.max(m)),
        )?;
    chart
        .configure_mesh()
        .x_labels(5)
        .y_label_offset(-20)
        .label_style(TextStyle::from(("sans-serif", 70).into_font()))
        .draw()?;
    chart
        .draw_series(LineSeries::new(
            (0..2 * n + 1).map(|x| ((x - n), prob[x as usize])),
            RGBColor(102, 170, 187).stroke_width(10),
        ))?
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    Ok(())
}

pub fn svg_plot_1d(prob: &[f64], dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let n = ((prob.len() - 1) / 2) as i32;
    let root = SVGBackend::new(dir, (2000, 1200)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .margin_left(140)
        .margin_right(70)
        .margin_top(50)
        .margin_bottom(50)
        .x_label_area_size(15)
        .y_label_area_size(10)
        .build_cartesian_2d(
            -n as i32..n as i32,
            -0f64..prob.iter().fold(f64::NAN, |m, v| v.max(m)),
        )?;
    chart
        .configure_mesh()
        .x_labels(5)
        .y_label_offset(-20)
        .label_style(TextStyle::from(("sans-serif", 70).into_font()))
        .draw()?;
    chart
        .draw_series(LineSeries::new(
            (0..2 * n + 1).map(|x| ((x - n), prob[x as usize])),
            RGBColor(102, 170, 187).stroke_width(10),
        ))?
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    Ok(())
}

#[test]
fn unit_test() {
    plot_1d(&[1.322, 1., 1., 1., 1.322], "image/test.png").unwrap();
}
