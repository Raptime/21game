use std::io;

fn main() {
    let mut run_tot = 0;
    let mut cpu;

    println!("**********************");
    println!("**    Game start    **");
    println!("**********************");

    loop {
        println!("** Current total: {}", run_tot);
        println!("** Choose your number: (1, 2, 3 or q to quit)");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.as_str().trim() {
            "q" => break,
            "1" => run_tot += 1,
            "2" => run_tot += 2,
            "3" => run_tot += 3,
            _ => continue,
        }

        if run_tot == 21 {
            println!("***********************");
            println!("** 21, Game is over! **");
            println!("**      You win!     **");
            println!("**  Congratulations! **");
            println!("***********************");
            break;
        }

        if run_tot < 18 {
            cpu = rand::random::<u32>() % 3 + 1;
        } else {
            cpu = 21 - run_tot;
        }

        run_tot += cpu;
        println!("** Computer is adding {}", cpu);

        if run_tot == 21 {
            println!("***********************");
            println!("** 21, Game is over! **");
            println!("**      Too bad!     **");
            println!("**     CPU wins!     **");
            println!("***********************");
            break;
        }
    }
    println!("quiting...");
}
