use std::env;
use presicion::{ Calculator, IdealGasPressure };

fn main() {
    let args: Vec<String> = env::args().collect();

    let ideal_gas_pressure = IdealGasPressure::new(&args);

    ideal_gas_pressure.calculate_and_print();
}
