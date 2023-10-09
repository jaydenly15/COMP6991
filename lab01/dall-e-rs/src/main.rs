use bmp::Image;

fn main() {
    let path = std::env::args().nth(1).expect("You must provide a path.");
    let n: u32 = 200;
    let mut img = Image::new(n, n);

    let radius: u32 = 25;

    for i in 0..n {
        for j in 0..n {
            if distance_from_center(i, j, n) < radius as f64 {
                img.set_pixel(i, j, bmp::consts::YELLOW);
            } else if j < n / 2 {
                img.set_pixel(i, j, bmp::consts::BLACK);
            } else if j >= n / 2 {
                img.set_pixel(i, j, bmp::consts::RED);
            }
        }
    }
    img.save(path).expect("This should save correctly.");
}

fn distance_from_center(x: u32, y: u32, n: u32) -> f64 {
    let center = (n - 1) as f64 / 2f64;

    ((center - x as f64).powi(2) + (center - y as f64).powi(2)).sqrt()
}
