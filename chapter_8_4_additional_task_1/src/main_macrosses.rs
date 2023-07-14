pub use crate::vec_functions::{vec_medium, vec_median, vec_mode};

#[macro_export]
macro_rules! vector_pushing_init {
    ($v:expr, $elder_label:tt) => {
        $elder_label: loop { 
            println!("\nInput vector's element number {}:", $v.len());

            let mut elem = String::new();

            io::stdin()
                    .read_line(&mut elem)
                    .expect("Error! \nLine has not been read! \nPlease, try again.");

            let elem: i32 = match elem.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            $v.push(elem);
            println!("\nElement '{}' has been pushed to the vector!", elem);

            vector_pushing_continue!($elder_label);
        }
    };
}

#[macro_export]
macro_rules! vector_pushing_continue {
    ($elder_label:tt) => {
        loop {
            println!("\n\nWould you like to continue pushing elements to the vector?");
            println!("\nPlease, type a letter to choose an operation:");
            println!("[Y] - Yes, I would.");
            println!("[N] - No, I'd rather watch results.");

            let mut operation_choice: String = String::new();
            io::stdin()
                .read_line(&mut operation_choice)
                .expect("\nError! \nLine has not been read! \nPlease, try again.\n");

            let operation_choice = operation_choice.trim().to_uppercase();

            if operation_choice == "N" {
                break $elder_label;
            }
            else if operation_choice == "Y" {
                continue $elder_label;
            }
            else {
                println!("\nWrong input! \nPlease, type a correct letter.");
            }
        }
    };
}

#[macro_export]
macro_rules! vectors_transformations_results {
    ($v:expr) => {
        println!("\n\nVector's medium value is {}", vec_medium(&mut $v));
        println!("Vector's median is {}", vec_median(&mut $v));
        println!("Vector's mode is {}", vec_mode(&mut $v));
    };
}
