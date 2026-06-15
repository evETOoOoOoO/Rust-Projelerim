fn main() {
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

    if *first_element < 5 {
        panic!("How can the first element be less than 5 when its value in the vector is 30?");
    }
}

#[allow(dead_code)]
fn variablesproject() {
    //Bu fonksiyon tüm öğrenim fonkisyonlarındaki bilgilerin(bi kısmının)
    //birleştirilmesinden oluşan bir projedir
    println!("Wait");
}
