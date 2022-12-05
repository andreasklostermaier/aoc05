use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {

    let mut preparation_mode                  = true;
    let mut stack_parse_cache: Vec<String>    = Vec::new();
    let mut local_stack      : Vec<char>      = Vec::new();
    let mut all_stacks_9000  : Vec<Vec<char>> = Vec::new();
    let mut all_stacks_9001  : Vec<Vec<char>> = Vec::new();

    if let Ok(lines) = read_lines("./data/input.txt") {

        for line in lines.flatten() {

            if line.is_empty() {
                // build the stack vectors and then end preparation mode
                let stack_height = stack_parse_cache.len() - 1;
                let nr_of_stacks = stack_parse_cache.pop().unwrap().as_str()
                                                    .split_ascii_whitespace().last().unwrap()
                                                    .parse::<usize>().unwrap();
                for stack in 1..=nr_of_stacks {
                    let stack_offset = (stack * 4)-3;
                    for level in (0..stack_height).rev() {
                        let crate_label = stack_parse_cache[level].chars().nth(stack_offset).unwrap();
                        if crate_label != ' ' {
                            local_stack.push(crate_label);
                        }
                    }
                    all_stacks_9000.push(local_stack.to_owned());
                    local_stack.clear();
                }
                all_stacks_9001 = all_stacks_9000.to_owned();
                preparation_mode = false;

            } else if preparation_mode {
                // cache the description of the initial stack layouts
                stack_parse_cache.push(line);
                continue;

            } else {
                let move_instruction: Vec<&str> = line.split(' ').collect();
                let nr_of_moves  = move_instruction[1].parse::<u32>().unwrap();
                let source_stack = move_instruction[3].parse::<usize>().unwrap();
                let target_stack = move_instruction[5].parse::<usize>().unwrap();
                // move the crates with CrateMover 9000 (Part 1)
                for _moves in 0..nr_of_moves {
                    let stack_to_move = all_stacks_9000[source_stack - 1].pop().unwrap();
                    all_stacks_9000[target_stack -1].push(stack_to_move);
                }
                // move the crates with CrateMover 9001 (Part 2)
                let split_offset = all_stacks_9001[source_stack - 1].len() - nr_of_moves as usize;
                let mut stacks_to_move = all_stacks_9001[source_stack - 1].split_off(split_offset);
                all_stacks_9001[target_stack -1].append(&mut stacks_to_move);

            }
        }
    }

    // collect the top crates of CrateMover9000
    let mut result_part1: String = String::new();
    for stack_iter in all_stacks_9000.iter() {
        result_part1.push(*stack_iter.last().unwrap());
    }
    // collect the top crates of CrateMover9001
    let mut result_part2: String = String::new();
    for stack_iter in all_stacks_9001.iter() {
        result_part2.push(*stack_iter.last().unwrap());
    }

    // Part 1
    println!("Part 1");
    println!("Top crates after all moves with CrateMover9000: {:?}", result_part1);

    // Part 2
    println!("Part 2");
    println!("Top crates after all moves with CrateMover9001: {:?}", result_part2);
 
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
