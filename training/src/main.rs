use plotters::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("src/data.csv");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut mileage = Vec::new();
    let mut price = Vec::new();

    // Leggi i dati dal file CSV
    for line in reader.lines() {
        let line = line?;
        let values: Vec<f64> = line
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        if values.len() == 2 {
            mileage.push(values[0]);
            price.push(values[1]);
        }
    }

    let m = mileage.len() as f64;
    if m == 0.0 {
        println!("No valid data points found.");
        return Ok(());
    }

    // Normalizza i dati
    let (mileage_norm, mileage_mean, mileage_std) = normalize(&mileage);
    let (price_norm, price_mean, price_std) = normalize(&price);

    let mut theta0 = 0.0;
    let mut theta1 = 0.0;
    let learning_rate = 0.01;
    let num_iterations = 1000;

    // Esegui la regressione lineare
    for _ in 0..num_iterations {
        let mut tmp_theta0 = 0.0;
        let mut tmp_theta1 = 0.0;

        for i in 0..mileage_norm.len() {
            let estimate = estimate_price(mileage_norm[i], theta0, theta1);
            let error = estimate - price_norm[i];
            tmp_theta0 += error;
            tmp_theta1 += error * mileage_norm[i];
        }

        theta0 -= learning_rate * (1.0 / m) * tmp_theta0;
        theta1 -= learning_rate * (1.0 / m) * tmp_theta1;
    }

    // De-normalizza theta0 e theta1
    let theta1_denorm = theta1 * (price_std / mileage_std);
    let theta0_denorm = (theta0 * price_std) + price_mean - (theta1_denorm * mileage_mean);

    println!("Theta0: {}", theta0_denorm);
    println!("Theta1: {}", theta1_denorm);

    plot_data(&mileage, &price, theta0_denorm, theta1_denorm)?;

    Ok(())
}

fn estimate_price(mileage: f64, theta0: f64, theta1: f64) -> f64 {
    theta0 + theta1 * mileage
}

fn normalize(data: &[f64]) -> (Vec<f64>, f64, f64) {
    let mean = data.iter().sum::<f64>() / data.len() as f64;
    let variance = data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / data.len() as f64;
    let std_dev = variance.sqrt();
    let normalized = data.iter().map(|&x| (x - mean) / std_dev).collect();
    (normalized, mean, std_dev)
}

fn plot_data(mileage: &[f64], price: &[f64], theta0: f64, theta1: f64) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("plot.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let min_x = mileage.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_x = mileage.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let min_y = price.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_y = price.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    let mut chart = ChartBuilder::on(&root)
        .caption("Linear Regression", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(min_x..max_x, min_y..max_y)?;

    chart.configure_mesh().draw()?;

    // Disegna i punti dati
    chart.draw_series(mileage.iter().zip(price.iter()).map(|(&x, &y)| {
        Circle::new((x, y), 3, &RED)
    }))?;

    // Disegna la linea di regressione
    chart.draw_series(LineSeries::new(
        (0..100).map(|i| {
            let x = min_x + (max_x - min_x) * i as f64 / 100.0;
            (x, theta0 + theta1 * x)
        }),
        &BLUE,
    ))?;

    root.present()?;

    println!("Plot has been saved to plot.png");

    Ok(())
}