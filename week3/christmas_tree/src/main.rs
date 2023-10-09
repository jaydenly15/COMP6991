use serde::Deserialize;
use std::collections::VecDeque;
use std::io;

#[derive(Debug, Deserialize)]
enum Instruction {
    Set(i32),
    Left,
    Right,
    Reset,
}

#[derive(Debug)]
struct Light {
    // TODO: change me!
    left: Option<Box<Light>>,
    right: Option<Box<Light>>,
    brightness: i32,
}

fn get_instructions_from_stdin() -> VecDeque<Instruction> {
    let mut instructions = String::new();
    io::stdin().read_line(&mut instructions).unwrap();
    ron::from_str(&instructions).unwrap()
}

fn create_node() -> Box<Light> {
    Box::new(Light {
        left: None,
        right: None,
        brightness: 0,
    })
}

// fn add_right(curr: &mut Box<Light>) {
//     // (*curr).right = Some(create_node());
//     // *curr = (*curr).right.unwrap();
// }

fn main() {
    let instructions = get_instructions_from_stdin();
    let mut root = create_node();
    let mut curr = &mut root;
    let mut sum = 0;
    let mut num = 1;

    for cmd in instructions {
        match cmd {
            Instruction::Set(value) => {
                let diff = value - curr.brightness;
                curr.brightness = value;
                sum += diff;
            }
            Instruction::Left => {
                curr.left = Some(create_node());
                curr = curr.left.as_mut().unwrap();
                num += 1;
            }
            Instruction::Right => {
                curr.right = Some(create_node());
                curr = curr.right.as_mut().unwrap();
                num += 1;
            }
            Instruction::Reset => {
                curr = &mut root;
            }
        }
    }
    println!("{}", sum/num);
    // let light = Light { left: (), right: (), brightness: 0};
    // println!("{instructions:?}");
    // println!("{light:?}");
    // TODO: your implementation here
}
