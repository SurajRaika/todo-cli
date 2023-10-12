use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn create_data_base() {
    let conn: Connection = Connection::open("tasks.db").unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Tasks (
            id    INTEGER PRIMARY KEY,
            task_name  TEXT NOT NULL
        )",
        [],
    )
    .expect("Not Able To Make");
}

#[derive(Debug)]
struct Task {
    id: i32,
    name: String,
}

pub fn extract_todo_list() -> Result<()> {
    println!("extract_todo_list");
    let conn = Connection::open("tasks.db")?;

    create_data_base();

    let mut sql_statement = conn.prepare("SELECT * FROM Tasks")?;
    let iter_tasks = sql_statement.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    });

    for task in iter_tasks.unwrap() {
        println!("{:?}", task);
    }
    Ok(())
}

pub fn add_todo_list(task: String) -> Result<()> {
    create_data_base();
    let conn = Connection::open("tasks.db")?;
    let som = conn.execute("INSERT INTO Tasks (task_name) VALUES (?1)", (&task,))?;
    Ok(())
}
