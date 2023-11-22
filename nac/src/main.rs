// noughts and crosses
fn reset_table(table: &mut Vec<char>) {
    for i in 0..9 {
        table[i] = (b'1' + (i) as u8) as char;
    }
}

fn print_table(table: &Vec<char>){
    let s1 = String::from("     |     |     ");
    let s2 = String::from("_____|_____|_____");

    for i in 0..3 {
        println!("{s1}");
        let s3: String = format!("  {}  |  {}  |  {}  ", table[3*i + 0], table[3*i + 1], table[3*i + 2]);
        println! ("{s3}");
        if i != 2 {
            println!("{s2}");
        } else {
            println!("{s1}");
        }
    }
}

fn check_win_state(table: &Vec<char>) -> bool {
    let chars = vec!['X', 'O'];

    for play in &chars {
        for i in 0..3 {
            let mut ok: i32 = 1;
            for j in 0..3 {
                if table[i*3 + j] != *play {
                    ok = 0;
                }
            }
            if ok == 1{
                return true;
            }
        }
    }
    for play in &chars {
        for i in 0..3 {
            let mut ok = 1;
            for j in 0..3 {
                if table[j*3 + i] != *play {
                    ok = 0;
                }
            }
            if ok == 1{
                return true;
            }
        }
    }
    if table[0] == table[4] && table[4] == table[8]{
        return true;
    }
    if table[2] == table[4] && table[4] == table[6]{
        return true;
    }

    return false
}

fn run_game(table: &mut Vec<char>) {
    let mut is_running: bool = true;
    while is_running {
        reset_table(table);
        let mut turn:i32 = 1;
        while !check_win_state(table) && turn < 10 {
            let play: &char;
            if turn % 2 == 0 {
                play = &'X';
            } else {
                play = &'O';
            }
            println!();
            

            print_table(table);

            let mut exit:bool = true;
            while exit {
                println!("Player {} moves!", 2 - ((turn) % 2));
                println!("Please enter a number (1->9): ");
                let mut s = String::new();
                std::io::stdin().read_line(&mut s).expect("Failed to read line");
                let num: u16 = s.trim().parse().expect("Please input a number!");
                println!("{}", table[usize::from(num) - 1]);
                println!("{}", char::from_digit(u32::from(num), 10).unwrap());
                if num >= 1 && num <= 9 && table[usize::from(num - 1)] == char::from_digit(u32::from(num), 10).unwrap(){
                    exit = false;
                    table[usize::from(num - 1)] = *play;
                } else {
                    println!("Try again please!");
                }
            }

            turn += 1;
            print!("{}[2J", 27 as char); // this clears the console
        }
        if turn == 10 {
            println!("Draw!");
        } else {
            println!("Congratulations player {}!", 2 - ((turn - 1) % 2))
        }
        println!();
        println!("Press any key to continue/ q to quit: ");
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).expect("Failed to read line!");
        s.truncate(s.len() - 1);
        println!("{s}");
        if s == "q" {
            println!("ba sunt aici");
            is_running = false;
        }
        print!("{}[2J", 27 as char); // this clears the console

    }
}

fn main() {
    let mut table: Vec<char> = vec!['1','2','3','4','5','6','7','8','9'];
    //println!("{}",check_win_state(&vec!['X','X','X', '4','5','6','7','8','9']));
    run_game(&mut table);
    println!("Thank you for playing!");
}
