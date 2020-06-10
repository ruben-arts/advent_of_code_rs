fn main() {
    // Answer already filled in to test before committing
    let mut input:Vec<usize> = vec![
        1, 67, 18, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 13, 1, 19, 1, 6, 19, 23, 2, 23, 6, 27,
        1, 5, 27, 31, 1, 10, 31, 35, 2, 6, 35, 39, 1, 39, 13, 43, 1, 43, 9, 47, 2, 47, 10, 51, 1,
        5, 51, 55, 1, 55, 10, 59, 2, 59, 6, 63, 2, 6, 63, 67, 1, 5, 67, 71, 2, 9, 71, 75, 1, 75, 6,
        79, 1, 6, 79, 83, 2, 83, 9, 87, 2, 87, 13, 91, 1, 10, 91, 95, 1, 95, 13, 99, 2, 13, 99,
        103, 1, 103, 10, 107, 2, 107, 10, 111, 1, 111, 9, 115, 1, 115, 2, 119, 1, 9, 119, 0, 99, 2,
        0, 14, 0,
    ];
    // Just as a test
    let output = run_all_instructions(input.clone());
    println!("The value at first run, at position '0' = {}", output);

    // Loop over all possible outcomes until calculation is the same as asked.
    // for noun in 0..99{
    //     input[1] = noun;
    //     for verb in 0..99{
    //         input[2] = verb;
    //         let output = run_all_instructions(input.clone());
    //         if output == 19690720{
    //             println!("Goteem");
    //             println!("noun = {}", noun);
    //             println!("verb = {}", verb);
    //             println!("100 * noun + verb = {}", 100*noun+verb)
    //         }
    //     }
    //
    // }
}
fn run_all_instructions(mut input: Vec<usize>) -> usize {
    let mut index:usize = 0;
    let mut opcode = input[index];

    while opcode != 99 && index + 3 <= input.len() {
        if opcode == 1 {
            let output = input[input[index + 1]] + input[input[index + 2]];
            let o_i = input[index + 3];
            input[o_i] = output;
        } else if opcode == 2 {
            let output = input[input[index + 1]] * input[input[index + 2]];
            let o_i = input[index + 3];
            input[o_i] = output;
        } else {
            panic!("unknown opcode")
        }
        index += 4;
        opcode = input[index];
    }
    input[0]
}


