use crate::fahrenheit::fahrenheit_celsius;
mod fahrenheit;

fn main () {
    let valor_celsius = 
        fahrenheit_celsius(234.0);
    println!("A conversao resulta em {}C ", valor_celsius);
}
