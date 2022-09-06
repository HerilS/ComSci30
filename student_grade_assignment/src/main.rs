// input output module used for taking input from user
use std::io;
// process module used for exiting program
use std::process;
// library to generate random numbers
use rand::Rng;

struct GradeBook {
    grades: Vec<u16>
}

trait GradeOperations {
    fn gen_grades() -> Vec<u16>; // static function used in randomize() and new()
    fn new() -> GradeBook;
    fn show_all(&self);
    fn honours(&self);
    fn max(&self) -> &u16;
    fn min(&self) -> &u16;
    fn avg(&self) -> u8;
    fn stats(&self);
    fn randomize(&mut self);
}

impl GradeOperations for GradeBook {
    fn gen_grades() -> Vec<u16> {
        let mut rng = rand::thread_rng();

        let mut rand_grades: Vec<u16> = Vec::with_capacity(35);

        for _ in 0..35 {
            rand_grades.push(rng.gen_range(0..101)); // gen_range fn returns [0, 101) -> will not include 101
        }

        rand_grades
    }

    fn new() -> GradeBook {
        GradeBook { grades: Self::gen_grades() }
    }

    fn show_all(&self) {
        println!("ALL GRADES");
        for i in &self.grades {
            println!("{}%", i)
        }
    }

    fn honours(&self) {
        println!("HONOURS");
        let mut count = 0;
        for i in &self.grades {
            if i >= &80 {
                println!("{}%", i);
                count += 1;
            }
        }
        println!("Number of Honours: {}", count);
    }

    fn max(&self) -> &u16 {
        self.grades.iter().max().unwrap()
    }

    fn min(&self) -> &u16 {
        self.grades.iter().min().unwrap()
    }

    fn avg(&self) -> u8 {
        let sum: u16 = self.grades.iter().sum();
        let mean: f32 = sum as f32 / self.grades.len() as f32;
        mean.round() as u8
    }

    fn stats(&self) {
        println!("STATS");
        println!("Highest Grade: {}%", self.max());
        println!("Lowest Grade: {}%", self.min());
        println!("Average Grade: {}%", self.avg());
    }

    fn randomize(&mut self) {
        let new_grades = Self::gen_grades();
        self.grades = new_grades;
        println!("GRADES HAVE BEEN RANDOMIZED");
    }
}

fn display_menu(grade_book: &mut GradeBook) {
    println!(r#"
MAIN MENU
    1. Display All Grades
    2. Display Honours
    3. Stats
    4. Randomize Grades
    5. Exit
    "#);

    let mut input = String::new();

    // make "input" variable = user input from terminal
    io::stdin().read_line(&mut input).expect("Failed to read line");

    println!("\n");
    // remove trailing whitespace from input
    // (input from keyboard will always contain trailing newline character)
    match input.trim_end() {
        "1" => grade_book.show_all(),
        "2" => grade_book.honours(),
        "3" => grade_book.stats(),
        "4" => grade_book.randomize(),
        "5" => process::exit(1),
        _ => println!("Invalid choice"),
    }
}

fn main() {
    let mut grade_book = GradeBook::new();

    loop {
        display_menu(&mut grade_book);
    }
}
