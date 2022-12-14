use aoc_2022::day_10::{parsing, part_1, part_2};
use aoc_2022::utils;

fn main() {
    let lines = utils::read_lines("inputs/day_10").unwrap();
    let instructions = parsing(&lines);
    println!("part 1: {}", part_1(&instructions));
    part_2(&instructions);
}


"####--##----##--##--###----##-###--####-"
"#----#--#----#-#--#-#--#----#-#--#-#----"
"###--#-------#-#--#-#--#----#-#--#-###--"
"#----#-------#-####-###-----#-###--#----"
"#----#--#-#--#-#--#-#----#--#-#-#--#----"
"#-----##---##--#--#-#-----##--#--#-####-"