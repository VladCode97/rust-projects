// fn summ(x: i32, y: i32) -> i32 { x + y } // Return expression (Good)
// fn less(x: i32, y: i32) -> i32 {
//     let result:i32 = x - y; // Declaration
//     return result; // 
//  }

// fn is_even(x: i32) -> bool { x%2 == 0 }

fn formule_fahrenheit_to_celcius(temp_fahrenheit: f32) -> f32 
    { ((temp_fahrenheit -32.0)*5.0) / 9.0 }

fn fomule_celcius_to_fahrenheit(temp_celcius: f32) -> f32 
    { (temp_celcius * (9.0/5.0)) + 32.0 }

fn main() {
    let result_celcius: f32 = formule_fahrenheit_to_celcius(300.0);
    let result_fahrenheit: f32 = fomule_celcius_to_fahrenheit(result_celcius);
    println!("{result_celcius} °C");
    println!("{result_fahrenheit} °F");
}
