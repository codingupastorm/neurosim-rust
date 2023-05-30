use plotters::prelude::*;

pub fn print_chart(t: &Vec<f64>, data: Vec<f64>, filename: &str) {
    let root = BitMapBackend::new(filename, (1200, 800)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let tmax = t[t.len() - 1];

    let min_range = data
            .iter()
            .cloned()
            .fold(0.0 / 0.0, f64::min);
        let max_range = data
            .iter()
            .cloned()
            .fold(0.0 / 0.0, f64::max);

        let mut chart = ChartBuilder::on(&root)
            .caption(filename, ("Arial", 20).into_font())
            .margin(5)
            .set_all_label_area_size(50)
            .build_cartesian_2d(0f64..tmax, min_range..max_range)
            .unwrap();

        chart.configure_mesh().draw().unwrap();

        chart
            .draw_series(LineSeries::new(
                t.iter().cloned().zip(data.iter().cloned()),
                &BLUE,
            ))
            .unwrap();
}