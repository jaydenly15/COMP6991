use bmp::Image;
use std::path::Path;
fn main() {
    let mut args: Vec<_> = std::env::args().collect();
    args.remove(0);
    for s in args {
        println!("===== {s} =====");
        // let filename = &s[8..s.len()];
        let filename = &s;
        // let filename = &format!("{}{}", "../", &s[8..s.len()]);
        // println!("{filename}");
        let path = Path::new(filename);
        let img = match bmp::open(path) {
            Ok(img) => {
                // for (x, y) in img.coordinates() {
                //     let pixel = img.get_pixel(x, y);
                //     if pixel == bmp::consts::RED {
                //         print!("R")
                //     } else if pixel == bmp::consts::BLUE {
                //         print!("B")
                //     } else if pixel == bmp::consts::WHITE {
                //         print!("W")
                //     } else if pixel == bmp::consts::LIME {
                //         print!("G")
                //     }
                //     if x == img.get_width() - 1 {
                //         println!("")
                //     } else {
                //         print!(" ")
                //     }
                // }
                for row in 0..img.get_height() {
                    for col in 0..img.get_width() {
                        let pixel = img.get_pixel(col, row);
                        if pixel == bmp::consts::RED {
                            print!("R")
                        } else if pixel == bmp::consts::BLUE {
                            print!("B")
                        } else if pixel == bmp::consts::WHITE {
                            print!("W")
                        } else if pixel == bmp::consts::LIME {
                            print!("G")
                        }

                        if col == img.get_width() - 1 {
                            println!("")
                        } else {
                            print!(" ")
                        }
                    }
                }
            }
            Err(err) => {
                println!("Error! {:?}", err);
            }
        };
    }
}
