use std::io;

fn main() {
    let input_str: Vec<&str> = vec!["option1", "option2", "option3"];
    let mut stdin = String::new();
    let mut selection: usize = 0;

    loop {
        stdin.clear();

        for (index, &element) in input_str.iter().enumerate() {
            if selection == index {
                println!("{}) {} <--", index, &element);
            } else {
                println!("{}) {}", index, &element);
            }
        }

        let _ = io::stdin().read_line(&mut stdin);

        match stdin.trim() {
            "q" => break,
            "j" => {
                // TODO: Could error if overflows during addition?
                selection = selection.saturating_add(1).min(input_str.len() - 1);
            }
            "k" => {
                selection = selection.saturating_sub(1);
            }
            _ => {}
        }
    }
    println!("You chose {}", input_str[selection]);
}
