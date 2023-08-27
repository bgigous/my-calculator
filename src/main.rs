struct MyCalculator {
    result: f64,
}

impl MyCalculator {
    fn add(&mut self, a: & f64) -> &mut Self {
        self.result += a;
        self
    }

    fn sub(&mut self, a: & f64) -> &mut Self {
        self.result -= a;
        self
    }

    fn div(&mut self, a: & f64) -> &mut Self {
        self.result /= a;
        self
    }

    fn mult(&mut self, a: & f64) -> &mut Self {
        self.result *= a;
        self
    }

    fn clear(&mut self) {
        self.result = 0.0;
    }
}

fn main() {
    let mut calc: MyCalculator = MyCalculator { result: (0.0) };
    println!("Start with {}", calc.result);

    let a: f64 = 5.5;
    let b: f64 = 1.5;
    let c: f64 = 2.0;
    let d: f64 = 4.0;

    println!("Add {}", a);
    calc.add(&a);
    println!("Subtract {}", b);
    calc.sub(&b);
    println!("Multiply {}", c);
    calc.mult(&c);
    println!("Divide {}", d);
    calc.div(&d);

    println!("Result is {}", calc.result);

    println!("Clear result");
    calc.clear();

    calc.add(&a).sub(&b).mult(&c).div(&d);
    println!("Daisy-chained result: {}", calc.result);

    println!("EL FIN");
}
