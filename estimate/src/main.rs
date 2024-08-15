use std::io;

struct LinearModel {
    theta0: f64,
    theta1: f64,
}

impl LinearModel {
    fn new() -> LinearModel {
        LinearModel {
            theta0: 0.0,
            theta1: 0.0,
        }
    }

    fn predict(&self, mileage: f64) -> f64 {
        self.theta0 + self.theta1 * mileage
    }
}

fn main() {
    let model = LinearModel::new();

    println!("Insert the car's mileage: ");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error in input reading");
    
    let mileage: f64 = input.trim().parse()
        .expect("Enter a valid mileage number");
    
    let estimated_price = model.predict(mileage);

    println!("Il prezzo stimato per un'auto con {} km è: {:.2} €", mileage, estimated_price);
}
