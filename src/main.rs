use colored::*;
use std::collections::VecDeque;
use std::fs;
use std::fmt::DebugList;
use std::cmp::min;
use std::fs::File;
use std::io::Write;

#[derive(Debug)]
struct Canvas{
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,

    canvas_line: Vec<char>,
    canvas_rows: Vec<Vec<char>>,
}
struct GenInstruction {
    // Horizontal or vertical so 'x' or 'y'
    direction: char,
    // Plus or minus a lenght
    lenght: i16,
}

fn get_input(filename: &str) -> Vec<Vec<GenInstruction>> {
    let input = fs::read_to_string(filename).expect("Filename was not known");
    let mut lines = Vec::new();
    for line in input.split("\n") {
        let mut input_list = Vec::new();
        for instruction in line.split(","){
            let mut dir = 'n';
            let mut len: i16 = 0;
            if instruction.starts_with('R')      {
                dir = 'x';
                len = instruction.replace("R", "").parse::<i16>().unwrap();
            }
            else if instruction.starts_with('L') {
                dir = 'x';
                len = -instruction.replace("L", "").parse::<i16>().unwrap();

            }
            else if instruction.starts_with('U') {
                dir = 'y';
                len = instruction.replace("U", "").parse::<i16>().unwrap();
            }
            else if instruction.starts_with('D') {
                dir = 'y';
                len = -instruction.replace("D", "").parse::<i16>().unwrap();
            }
            else{
                println!("Direction not found")
            }
            let instruct = GenInstruction{
                direction: dir,
                lenght: len,
            };

            println!("{} is on the {} axis with lenght {}", instruction, dir, len);

            input_list.push(instruct);
        }
    lines.push(input_list);
    }
    lines
}
impl Canvas {
    fn to_string(&self) -> String {
        let mut s:String = "".parse().unwrap();
        for line in &self.canvas_rows {
            let line_as_string: String = line.iter().collect();
            s = s + line_as_string.as_ref();
            s.push('\n');
        }
        s
    }

    fn to_file(&self) -> std::io::Result<()>{
        let mut file = File::create("canvas.txt")?;
        file.write_all(self.to_string().as_ref())?;
        Ok(())
    }

    fn draw_parcour(&mut self, instructions:Vec<GenInstruction>) -> &mut Self {
        let mut cur_x= self.min_x.abs() as usize;
        let mut cur_y= self.min_y.abs() as usize;
        for i in instructions{
            if i.direction == 'x'{
                for _ in 0..i.lenght.abs() {
                    if i.lenght > 0 {
                        cur_x += 1;
                    }
                    else{
                        cur_x -= 1;
                    }
                self.canvas_rows[cur_y][cur_x] = ".".red().parse().unwrap();
                }
            }
            else if i.direction == 'y'{
                for _ in 0..i.lenght.abs() {
                    if i.lenght > 0 {
                        cur_y += 1;
                    }
                    else{
                        cur_y -= 1;
                    }
                self.canvas_rows[cur_y][cur_x] = ".".red().parse().unwrap();
                }
            }
        }
        self
    }
}


fn calc_canvas(instructions_lists: Vec<Vec<GenInstruction>>) -> Canvas{
    let (mut min_x, mut max_x) = (0,0);
    let (mut min_y, mut max_y) = (0,0);
    for instructions in instructions_lists {
        let (mut x,mut y) = (0,0);
        for i in instructions {
            if i.direction == 'x' {
                x = x + i.lenght as i32;
                if x < min_x {
                    min_x = x;
                } else if x > max_x {
                    max_x = x;
                }
            } else if i.direction == 'y' {
                y = y + i.lenght as i32;
                if y < min_y {
                    min_y = y;
                } else if y > max_y {
                    max_y = y;
                }
            }
        }
    }
    let canvas_line = vec!['.'; (max_x - min_x) as usize];
    Canvas{
        min_x,
        max_x,
        min_y,
        max_y,
        canvas_line: canvas_line.clone(),
        canvas_rows: vec![canvas_line; (max_y - min_y) as usize]
    }
}

fn main() {
    let lists = get_input("./resources/input_puzzle_3.txt");
    let canvas = calc_canvas(lists);
    println!("First canvas has size: x:{} y:{}", canvas.canvas_line.len(), canvas.canvas_rows.len());
    for list in lists{
        let canvas = canvas.draw_parcour(list);
    }
    canvas.to_file();

}
// fn main(){
//     let input = get_input("./resources/input_puzzle_3.txt");
//     let mut vec_line: VecDeque<ColoredString> = VecDeque::new();
//     let mut vec_2d: VecDeque<VecDeque<ColoredString>> = VecDeque::new();
//     // for _ in 0..99 {
//     //     vec_line.push(".".red());
//     // }
//     // for _ in 0..99 {
//     //     vec_2d.push(vec_line.clone());
//     // }
//     // draw(vec_2d)
//     //
//     let filler = ".".red();
//     for index in 1..5 {
//         let mut line = vec_line.clone();
//         line.insert(index, filler.clone());
//         if vec_2d[x + 1].is_empty() {
//             vec_2d.push_front(line)
//         }
//     }
//     // let input = get_input("./resources/input_puzzle_3.txt");
//     // for line in input{
//     //     instruction_parse(line)
//     // }
// }
// fn instruction_parse(instructions: Vec<String>, filler: ColoredString){
//     /// The start of the grid is left top.
//     /// So a push_front would be above the current point or left from the current point
//     let mut x = 0;
//     let mut y = 0;
//     let mut vec_line: VecDeque<ColoredString> = VecDeque::new();
//     let mut vec_2d: VecDeque<VecDeque<ColoredString>> = VecDeque::new();
//     for mut instruction in instructions{
//         if instruction.starts_with("U"){
//             let argument = instruction.split_off(1);
//             println!("Argument of U : {}", argument);
//             for _ in argument.parse(){
//                 let line = vec_line.clone().insert(x,filler.clone());
//                 if vec_2d[x + 1].is_empty() {
//                     vec_2d.push_front(line)
//                 }
//             }
//         }
//
//     }
// }
//
// fn draw(vec_2d: VecDeque<VecDeque<ColoredString>>){
//     for line in vec_2d{
//         for dot in line{
//             print!("{}", dot);
//         }
//         println!();
//
//     }
// }
// fn get_input(filename: &str) -> Vec<Vec<String>> {
//     let input = fs::read_to_string(filename).expect("Filename was not known");
//     let mut input_vec_2d: Vec<Vec<String>> = Vec::new();
//
//     for line in input.split("\n") {
//         let mut input_line: Vec<String> = Vec::new();
//         for instruction in line.split(","){
//                 input_line.push(instruction.to_string());
//         }
//         input_vec_2d.push(input_line);
//     }
//     input_vec_2d
// }