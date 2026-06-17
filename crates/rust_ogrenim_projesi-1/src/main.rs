use std::io;

fn main() {
    // Bu fonksiyon değişkenler ve veri tipleri ile ilgili bir proje örneği sunar.
    println!("Please say your name: ");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    println!("Hello, {name}!");

    let name = name.trim();

    println!("Are you wan't to do a math operation? (Y/N): ");
    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");
    let answer = answer.trim().to_uppercase();
    if answer == "Y" {
        println!(
            "Which mathematical operation would you like to perform (addition, subtraction, multiplication, division, square root ( A / D / S / M / SR )?"
        );

        let multiplier = 10000.0;

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice = choice.trim().to_uppercase();

        if choice == "A" {
            println!("Please enter the first number: ");
            let mut num1_str = String::new();
            io::stdin()
                .read_line(&mut num1_str)
                .expect("Failed to read line");
            let num1_f64: f64 = num1_str
                .trim()
                .parse()
                .expect("Please enter a valid number");
            let num1_scaled = (num1_f64 * multiplier).round() as i64;

            println!("Please enter the second number: ");
            let mut num2_str = String::new();
            io::stdin()
                .read_line(&mut num2_str)
                .expect("Failed to read line");
            let num2_f64: f64 = num2_str
                .trim()
                .parse()
                .expect("Please enter a valid number");
            let num2_scaled = (num2_f64 * multiplier).round() as i64;
            let whole_number_scaled = num1_scaled + num2_scaled;

            let final_result = whole_number_scaled as f64 / multiplier;
            println!("The whole number is: {}", final_result);
        }
    } else {
        println!("Goodbye, {name}!");
    }
}

#[allow(dead_code)]
fn rust_ogrenim_aşaması() {
    println!();
    let name = String::from("Ahmet Asaf Güleç");
    println!("{}", name);
    println!();

    let mut player_score = 51;
    player_score += 1;
    println!("Player score: {player_score}");
    println!();

    let _delta_time = 1.25;
    let _delta_time: f32 = 1.25;
    let _delta_time = 1.25_f32;
    let delta_time = 1.25f32;
    println!("Current delta time {delta_time}");
    println!();

    let total_points: u8 = 1 + 2 + 5;
    println!("{total_points}");
    println!();

    let color_in_hex = 0xFF0033;
    println!("Background color is: {color_in_hex}");
    println!();

    let dir_permission: i32 = 0o755;
    println!("Directory permission is {dir_permission:o} / {dir_permission}");
    println!();

    let gate_flag: u8 = 0b1010_0100;
    println!("Gate flag is {gate_flag:b} / {gate_flag}");
    println!();

    let is_active = true;
    println!("Is active {is_active}");
    println!();

    let first_char = 'a';
    println!("The first char is {first_char}");
    println!();

    println!(
        "All variables above {name}, {player_score}, {delta_time}, {total_points}, {color_in_hex}, {dir_permission}, {gate_flag}, {is_active}, {first_char}"
    );
    println!();

    let config = (640, 400, "Hello World".to_string(), false);
    println!("The config  is {config:?}");
    println!();
    let width = config.0;
    let height = config.1;
    let (w, h) = (width, height);
    println!("The screen resolution is {w}:{h}");
    println!();

    let mut scores: [u8; 5] = [56, 10, 90, 100, 48];
    println!("The Scores are {scores:?}");
    println!();
    println!("The first score {}, Length is {}", scores[0], scores.len());
    println!();
    scores[1] += 50;
    println!("The first score {}", scores[0]);
    println!();

    let mut vek: Vec<i32> = vec![30, 10, 20, 40, 90, 12, 38, 1203, 101];
    vek.push(92);
    let first_element = &vek[0];
    println!("First element: {}", first_element);
    println!();
    println!("Push element: {}", vek[vek.len() - 1]);
    println!();

    if *first_element < 5 {
        panic!("How can the first element be less than 5 when its value in the vector is 30?");
    }

    let mut colors = Vec::new();
    colors.push(String::from("red"));
    colors.push(String::from("blue"));
    colors.push(String::from("green"));
    println!("Vec list {:?}", colors);
    println!();

    let codes: Vec<u8> = (0..=255).collect();
    println!("Codes: {:?}", codes);
    println!();
    let hero_name = "Ahmet Asaf Güleç".to_string();
    println!("Hero name: {}", hero_name);
    let short_name = hero_name.replace("Ahmet Asaf Güleç", "Asaf");
    println!("Short name: {}", short_name);
    println!();
}
