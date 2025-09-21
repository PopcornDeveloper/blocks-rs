use rand::Rng;
use std::any::type_name;
use std::env;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
fn main() {
    let types: [&str; 5] = ["thicc", "thin", "ultrapromaxthicc", "russia", "sus"];

    let mut current_type: String = "russia".to_string();

    let args: Vec<String> = env::args().collect();

    let mut delay = 10; // in ms

    for (index, arg) in args.iter().enumerate() {
        if arg == "-h" || arg == "--help" {
            println!(
                "-h, --help - Prints this text and quits\n\
                -d, --delay - The amount of time delayed / frame in ms (milliseconds), 0  for unlimited. By default is 10 ms. \n\
                -t, --type - The look of the tetris blocks. The types are '``thicc', 'thin', 'ultrapromaxthicc', 'russia', and 'sus'. By default is 'russia' \n\

                This version was not built with the TETRIS feature. To get it, clone a github repo that doesn't exist right now. :) \n\n "

            );
            std::process::exit(0);
        } else if arg == "-d" || arg == "--delay" {
            // Ensure the next argument exists and parse it
            if index + 1 < args.len() {
                let idk_what_to_call_this: i32 = args[index + 1]
                    .parse()
                    .expect("Failed to parse string to integer");
                if type_of(idk_what_to_call_this) != "int32" && idk_what_to_call_this < 0 {
                    println!("Must be a positive integer");
                    std::process::exit(0);
                }
                delay = idk_what_to_call_this as u64;
            }
        } else if arg == "-t" || arg == "--type" {
            let idk_what_to_call_this = &args[index + 1];
            current_type = idk_what_to_call_this.to_string();
        }
    }
    let columns = 10;
    let mut tetrisArray = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 20 (Ignore)
    ];
    /*
     * Alright so we got the array here, right?
     * In a nutshell, 1 is the floor, it's where the blocks land at the start.
     * Value 2 are the blocks that hit the ground and are just sorta sitting there.
     * Value 3 are the falling tetrominos.*/
    loop {
        let mut revArray = tetrisArray;
        revArray.reverse();
        for row_rev in 0..(revArray.len() / columns) {
            for col_rev in 0..columns {
                let index = row_rev * columns + col_rev;
                if revArray[index] == 3 {
                    // Check if it's at the bottom or if the block below is occupied by a stationary block
                    if index < 10 || (revArray[index - 10] != 0) {
                        // Stop the tetromino from moving down
                        revArray[index] = 2; // Change to a stationary block
                    } else {
                        // Move the tetromino down
                        revArray[index] = 0;
                        revArray[index - 10] = 3;
                    }
                }
            }
        }

        revArray.reverse();
        tetrisArray = revArray;

        let point = rand::thread_rng().gen_range(0..10);
        tetrisArray[point] = 3;
        for row in 0..(tetrisArray.len() / columns) {
            for col in 0..columns {
                let index = row * columns + col;

                if tetrisArray[index] == 0 {
                    if current_type == "thicc" {
                        print!(" . ");
                    } else if current_type == "thin" {
                        print!(".")
                    } else if current_type == "sus" {
                        print!(" . ")
                    } else if current_type == "ultrapromaxthicc" {
                        print!("   .   ")
                    } else if current_type == "russia" {
                        print!(" .")
                    }
                } else if tetrisArray[index] == 2 {
                    if index < 10 {
                        tetrisArray = [
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 1
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 2
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 3
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 4
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 5
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 6
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 7
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 8
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 9
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 10
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 11
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 12
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 13
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 14
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 15
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 16
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 17
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 18
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 19
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 20 (Ignore)
                        ];
                    } else {
                        if current_type == "thicc" {
                            print!("[*]")
                        } else if current_type == "thin" {
                            print!("#")
                        } else if current_type == "sus" {
                            print!("[ඞ]")
                        } else if current_type == "ultrapromaxthicc" {
                            print!("[THICC]")
                        } else if current_type == "russia" {
                            print!("[]")
                        }
                    }
                } else if tetrisArray[index] == 3 || tetrisArray[index] == 2 {
                    if current_type == "thicc" {
                        print!("[*]")
                    } else if current_type == "thin" {
                        print!("#")
                    } else if current_type == "sus" {
                        print!("[ඞ]")
                    } else if current_type == "ultrapromaxthicc" {
                        print!("[THICC]")
                    } else if current_type == "russia" {
                        print!("[]")
                    }
                } else if tetrisArray[index] != 1 {
                    print!("[!THIS BLOCK SHOULD NOT EXIST!]")
                }
            }
            println!("");
        }
        thread::sleep(Duration::from_millis(delay));
        print!("\x1B[2J\x1B[H");
    }
}
