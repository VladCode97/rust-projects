fn add_temperature(temp_data: &mut  Vec<f32>, temp: f32) {
    temp_data.push(temp);
}

fn view_temperature(temp_data: Vec<f32>) {
    for temp in temp_data.iter() {
        println!("{}", temp);
    }
}

fn main() {
    let mut temperature: Vec<f32> = Vec::new();
    add_temperature(&mut temperature, 3.2);
    add_temperature(&mut temperature, 8.1);
    add_temperature(&mut temperature, 3.94);
    view_temperature(temperature);
}
