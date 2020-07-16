use std::io;

fn main() {
    let mut run_tot = 0;
    let mut cpu;

    loop {
        println!("Current total: {}", run_tot);
        println!("Choose your number: (1, 2, 3 or q to quit)");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim() == "q" {
            break;
        }

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match input {
            1 | 2 | 3 => run_tot += input,
            _ => println!("you can only add 1, 2 or 3 to the total!"),
        }

        if run_tot == 21 {
            println!("Game is over! You win, congratulations!");
            break;
        }

        if run_tot < 18 {
            cpu = rand::random::<u32>() % 3 + 1;
        } else {
            cpu = 21 - run_tot;
        }

        run_tot += cpu;
        println!("Computer is adding {}", cpu);

        if run_tot == 21 {
            println!("Game is over! Too bad, CPU wins!");
            break;
        }
    }
    println!("quiting...");
}
