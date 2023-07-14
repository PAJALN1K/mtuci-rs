use std::io;

#[macro_export]
macro_rules! app_intro {
    ($name:expr, $description:expr) => {
        println!("\n\n----------------------------");
        println!("{}", $name);
        println!("----------------------------");
        println!("\nWelcome! {}", $description);
    };
}

#[macro_export]
macro_rules! the_beginning_of_app {
    ($elder_label:tt, $younger_label:tt) => {
        $younger_label: loop {
            println!("\n\nWould you like to start using this program?");
            println!("\nPlease, type a letter to choose an operation:");
            println!("[Y] - Yes, I would.");
            println!("[N] - No, I'd rather quit.");

            let mut operation_choice = String::new();

            io::stdin()
                .read_line(&mut operation_choice)
                .expect("\nError! Line has not been read! \nPlease, try again.\n");

            let operation_choice = operation_choice.trim().to_uppercase();

            if operation_choice != "Y" && operation_choice != "N" {
                println!("\nWrong input! \nPlease, type a correct letter.");
                continue;
            } 
            else if operation_choice == "N" {
                break $elder_label;
            } 
            else {
                break $younger_label;
            }
        }
    };
}

#[macro_export]
macro_rules! the_end_of_app {
    ($label:tt) => {
        loop {
            println!("\n\nWould you like to continue using this program?");
            println!("\nPlease, type a letter to choose an operation:");
            println!("[Y] - Yes, I would.");
            println!("[N] - No, I'd rather quit.");

            let mut operation_choice: String = String::new();
            io::stdin()
                .read_line(&mut operation_choice)
                .expect("\nError! \nLine has not been read! \nPlease, try again.\n");

            let operation_choice = operation_choice.trim();

            if operation_choice == "Y" || operation_choice == "y" {
                continue $label;
            }
            else if operation_choice == "N" || operation_choice == "n" {
                break $label;
            } 
            else {
                println!("\nWrong input! \nPlease, type a correct letter.");
            }
        }
    }
}