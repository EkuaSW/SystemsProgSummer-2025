// Temperature Converter 
fn fahrenheit_to_celcius(f: f64) -> f64
{
    (f - 32.0)/1.8
}

fn celcius_to_fahrenheit(c: f64) -> f64
{
  c * 1.8 + 32.0
}


fn main() {
    let mut tempc : f64 = 30.0;
    println!("{} C = {} F",tempc,  celcius_to_fahrenheit(tempc));

    let mut tempf : f64 = 50.0;
    println!("{} F = {} C", tempf, fahrenheit_to_celcius(tempf));

    let f: f64 = tempc;

    for i in 1..6
    {
        tempf += 1.0;
        println!("F = {}", fahrenheit_to_celcius(tempf));
    }
}
