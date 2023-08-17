pub mod models;
pub mod task_parameters;
pub mod schema;
pub mod operations;

use std::io;

fn main() {
    println!("\n\n----------------------------");
    println!("TODO-CLI");
    println!("----------------------------");
    println!("\nWelcome! This is a todo-program that helps you to organize your routines tasks.");

    'main_loop: loop {
        println!("\nPlease, input a needed action.");
        println!("1. Add a task to the list.");
        println!("2. Delete a task from the list.");
        println!("3. Rename a task from the list.");
        println!("4. Mark a task as completed.");
        println!("5. Mark a task as unfinished.");
        println!("6. Mark a task as rejected.");
        println!("7. Mark an importance of the task.");
        println!("8. Mark a lifesphere of the task.");
        println!("9. Display the list of the tasks.");
        println!("10. Exit from cli.");

        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Error! \nLine has not been read! \nPlease, try again.");

        let command = match command.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match command {
            1 => operations::cmd_1_add::add_task(), 
            2 => operations::cmd_2_delete::delete_task(),
            3 => operations::cmd_3_rename::rename_task(),
            4 => operations::cmd_4_complete::mark_as_completed(),
            5 => operations::cmd_5_unfinish::mark_as_unfinished(), 
            6 => operations::cmd_6_reject::mark_as_rejected(),  
            7 => operations::cmd_7_importance::mark_an_importance(),
            8 => operations::cmd_8_lifesphere::mark_a_lifesphere(),
            9 => operations::cmd_9_display::display_tasks(),
            10 => break 'main_loop,
            _ => continue,
        }
    }
}
