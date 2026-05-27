fn main()
{
    //Bu fonksiyon tüm öğrenim fonkisyonlarındaki bilgilerin(bi kısmının)
    //birleştirilmesinden oluşan bir projedir
}

fn öğrenim1()
{
    println!("");
    let name = String::from("Ahmet Asaf Güleç");
    println!("{}", name);
    println!("");
    
    let mut player_score = 51;
    player_score += 1;
    println!("Player score: {player_score}");
    println!("");
    
    let _delta_time = 1.25;
    let _delta_time: f32 = 1.25;
    let _delta_time = 1.25_f32;
    let delta_time = 1.25f32;
    println!("Current delta time {delta_time}");
    println!("");
    
    let total_points: u8 = 1 + 2 + 5;
    println!("{total_points}");
    println!("");
    
    let color_in_hex = 0xFF0033;
    println!("Background color is: {color_in_hex}");
    println!("");
    
    let dir_permission: i32 = 0o755;
    println!("Directory permission is {dir_permission:o} / {dir_permission}");
    println!("");
    
    let gate_flag: u8 = 0b1010_0100;
    println!("Gate flag is {gate_flag:b} / {gate_flag}");
    println!("");
    
    let is_active = true;
    println!("Is active {is_active}");
    println!("");
    
    let first_char = "a";
    println!("The first char is {first_char}");
    println!("");
    
    println!("All variables are {name}, {player_score}, {delta_time}, {total_points}, {color_in_hex}, {dir_permission}, {gate_flag}, {is_active}, {first_char}")
}

fn öğrenim2()
{
 //BOŞ
}
