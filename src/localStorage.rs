use lazy_static::lazy_static;
use rusqlite::{Connection, Result,params};
use rusqlite_migration::{Migrations, M};
use crate::task::Task;










pub fn extract_todo_list() -> Result<()> {
    let  conn = Connection::open("tasks.db").unwrap();
    let mut sql_statement = conn.prepare("SELECT * FROM Tasks")?;
    let iter_tasks = sql_statement.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            name: row.get(1)?,
            completed: row.get(2)?,
            description: row.get(3)?,

        })
    });

    for task in iter_tasks.unwrap() {
        task.unwrap().colored_display()
    }
    Ok(())
}

pub fn add_todo_list(task: Task) -> Result<()> {
    let  conn = Connection::open("tasks.db").unwrap();
    let som = conn.execute("INSERT INTO Tasks (task_name,completed,description) VALUES (?1,?2,?3)",
    params![task.name,task.completed,task.description],)?;
    Ok(())
}
