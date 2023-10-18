pub struct IdealGasPressure {
    pub n: f64,
    pub r: f64,
    pub t: f64,
    pub v: f64,
}

impl IdealGasPressure {
    pub fn new(args: &[String]) -> IdealGasPressure {
        let n = args[1].clone().parse::<f64>().unwrap();
        let r = args[2].clone().parse::<f64>().unwrap();
        let t = args[3].clone().parse::<f64>().unwrap();
        let v = args[4].clone().parse::<f64>().unwrap();

        IdealGasPressure { n, r, t, v }
    }
}

impl Calculator for IdealGasPressure {
    fn calculate(&self) -> f64 {
        self.n * self.r * self.t / self.v
    }
}

impl Printer for IdealGasPressure {
    fn println(&self, result: f64) {
        println!("Amount of substance (moles): {}", self.n);
        println!("Gas constant: {}", self.r);
        println!("Temperature (Kelvin): {}", self.t);
        println!("Volume (liters): {}", self.v);
        println!("Ideal gas pressure: {}", result);
    }
}

pub trait Calculator: Printer {
    fn calculate(&self) -> f64;

    fn calculate_and_print(&self) {
        let result = self.calculate();
        self.println(result);
    }
}

pub trait Printer {
    fn println(&self, result: f64);
}