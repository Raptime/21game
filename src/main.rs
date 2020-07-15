use rand::Rng;
use std::io;

fn main() {
    let mut run_tot = 0;
    let mut cpu;

    loop {
        println!(
            "Choose your number: (1, 2, 3 or q to quit), running total is: {}",
            run_tot
        );

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim() == "q" {
            println!("quiting...");
            break;
        }

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match input {
            1 | 2 | 3 => {
                run_tot += input;
                println!(
                    "adding {} to running total...\nnew total is: {}",
                    input, run_tot
                );
            }
            _ => println!("you can only add 1, 2 or 3 to the total!"),
        }

        if run_tot == 21 {
            println!("Game is over! You win, congratulations!");
            break;
        }

        if run_tot < 18 {
            cpu = rand::thread_rng().gen_range(1, 3);
            run_tot += cpu;

            println!(
                "Computer is adding {} to running total\nnew total is: {}",
                cpu, run_tot
            );
        } else {
            cpu = 21 - run_tot;
            run_tot += cpu;
            println!(
                "Computer is adding {} to running total\nnew total is: {}",
                cpu, run_tot
            );
        }

        if run_tot == 21 {
            println!("Game is over! Too bad, CPU wins!");
            break;
        }
    }
}
