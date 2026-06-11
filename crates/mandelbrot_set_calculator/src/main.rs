fn main() {
    let width = 350;
    let height = 100;
    let max_iter = 3000;

    let palette = " .`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";

    for py in 0..height {
        let mut row = String::new();
        for px in 0..width {
            let x0 = (px as f64 / width as f64) * 3.5 - 2.5;
            let y0 = (py as f64 / height as f64) * 2.0 - 1.0;

            let mut x = 0.0;
            let mut y = 0.0;
            let mut iteration = 0;

            while x * x + y * y <= 4.0 && iteration < max_iter {
                let xtemp = x * x - y * y + x0;
                y = 2.0 * x * y + y0;
                x = xtemp;
                iteration += 1;
            }

            if iteration == max_iter {
                row.push(' ');
            } else {
                let char_idx = iteration % palette.len();
                row.push(palette.chars().nth(char_idx).unwrap());
            }
        }
        println!("{}", row);
    }
}
