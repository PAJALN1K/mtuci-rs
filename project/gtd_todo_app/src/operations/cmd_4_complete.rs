use std::io::stdin;

use self::models::TaskDB;
use diesel::prelude::*;
use gtd_todo_app::*;

pub fn mark_as_completed() {
    use self::schema::tasks::dsl::{tasks, state};

    let mut task_id = String::new();

    println!("\nInput the ID of the needed task:");
    stdin().read_line(&mut task_id).expect("Input error");
    let task_id = task_id.trim().parse::<i32>().expect("Invalid ID");

    let conn = &mut establish_connection();

    let task = diesel::update(tasks.find(task_id))
        .set(state.eq("Completed"))
        .returning(TaskDB::as_returning())
        .get_result(conn)
        .unwrap();
    println!("The task with name \"{}\" is now completed!", task.name);
}
