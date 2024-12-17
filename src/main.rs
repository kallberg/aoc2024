mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

#[allow(dead_code)]
mod input;
#[allow(dead_code)]
mod output;

fn print_result(result: &str, day: u8, one: bool) {
    println!(
        "Day={}, Part={}, Result={}",
        day,
        if one { 1 } else { 2 },
        result
    );
}

fn main() {
    print_result(&day01::part_1(input::DAY_01), 1, true);
    print_result(&day01::part_2(input::DAY_01), 1, false);
    print_result(&day02::part_1(input::DAY_02), 2, true);
    print_result(&day02::part_2(input::DAY_02), 2, false);
    print_result(&day03::part_1(input::DAY_03), 3, true);
    print_result(&day03::part_2(input::DAY_03), 3, false);
    print_result(&day04::part_1(input::DAY_04), 4, true);
    print_result(&day04::part_2(input::DAY_04), 4, false);
    print_result(&day05::part_1(input::DAY_05), 5, true);
    print_result(&day05::part_2(input::DAY_05), 5, false);
    print_result(&day06::part_1(input::DAY_06), 6, true);
    print_result(&day06::part_2(input::DAY_06), 6, false);
    print_result(&day07::part_1(input::DAY_07), 7, true);
    print_result(&day07::part_2(input::DAY_07), 7, false);
    print_result(&day08::part_1(input::DAY_08), 8, true);
    print_result(&day08::part_2(input::DAY_08), 8, false);
    print_result(&day09::part_1(input::DAY_09), 9, true);
    print_result(&day09::part_2(input::DAY_09), 9, false);
}
