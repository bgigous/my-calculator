struct MyCalculator {
    result: f64,
}

impl MyCalculator {
    fn add(&mut self, a: & f64) -> f64{
        self.result += a;
        self.result
    }
}

fn main() {
    let mut calc: MyCalculator = MyCalculator { result: (0.0) };

    let a: f64 = 5.5;
    println!("Add {}", a);
    calc.add(&a);

    println!("Result is {}", calc.result);
}
