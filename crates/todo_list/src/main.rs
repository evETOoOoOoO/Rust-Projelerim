use std::io;

struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

fn main() {
    let mut tasks: Vec<Todo> = Vec::new();

    loop {
        println!("\n=== Todo List ===");
        println!("1. Görev ekle");
        println!("2. Görevleri listele");
        println!("3. Görevi tamamlandı işaretle");
        println!("4. Görev ara");
        println!("5. Çıkış");
        println!("Seçiminiz:");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice = choice.trim();

        match choice {
            "1" => {
                println!("Görev başlığını girin:");
                let mut title = String::new();
                io::stdin()
                    .read_line(&mut title)
                    .expect("Failed to read line");
                let title = title.trim().to_string();

                let new_task = Todo {
                    id: tasks.len() as u32 + 1,
                    title: title,
                    completed: false,
                };

                tasks.push(new_task);
                println!("Görev eklendi!");
            }
            "2" => {
                if tasks.is_empty() {
                    println!("Henüz görev yok");
                } else {
                    for task in &tasks {
                        let durum = if task.completed { "✓" } else { " " };
                        println!("[{}] {} - {}", durum, task.id, task.title);
                    }
                }
            }
            "3" => {
                println!("Hangi görevin id'sini tamamlandı işaretlemek istiyorsunuz?");
                let mut id_input = String::new();
                io::stdin()
                    .read_line(&mut id_input)
                    .expect("Failed to read line");
                let id_input: u32 = match id_input.trim().parse() {
                    Ok(id) => id,
                    Err(_) => {
                        println!("Geçersiz sayı. ");
                        continue;
                    }
                };

                let mut found = false;

                for task in &mut tasks {
                    if task.id == id_input {
                        task.completed = true;
                        found = true;
                        println!("Görev tamamlandı olarak işaretlendi!");
                    }
                }
                if !found {
                    println!("Görev Bulunamadı");
                }
            }
            "4" => {
                println!("Aranacak görev id'sini girin:");
                let mut search_id = String::new();
                io::stdin()
                    .read_line(&mut search_id)
                    .expect("Failed to read line");
                let search_id: u32 = match search_id.trim().parse() {
                    Ok(id) => id,
                    Err(_) => {
                        println!("Geçersiz sayı");
                        continue;
                    }
                };

                let result = find_task(&tasks, search_id);
                match result {
                    Some(task) => {
                        println!("Bulundu: {} (tamamlandı: {})", task.title, task.completed)
                    }
                    None => println!("Bulunamadı"),
                }
            }
            "5" => {
                println!("Görüşürüz!");
                break;
            }
            _ => {
                println!("Geçersiz seçim, tekrar deneyin.");
            }
        }
    }
}

fn find_task(tasks: &Vec<Todo>, id: u32) -> Option<&Todo> {
    for task in tasks {
        if task.id == id {
            return Some(task);
        }
    }
    None
}
