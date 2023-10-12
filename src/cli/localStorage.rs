use lazy_static::lazy_static;
use rusqlite::{Connection, Result};

use rusqlite_migration::{Migrations, M};

lazy_static! {
    static ref MIGRATIONS: Migrations<'static> = Migrations::new(vec![
        M::up(
            r#"
        CREATE TABLE IF NOT EXISTS Tasks (
            id    INTEGER PRIMARY KEY,
            task_name  TEXT NOT NULL
        )
          "#
        ),
        M::up(
            r#"
                  ALTER TABLE Tasks ADD COLUMN completed BOOL;
                  ALTER TABLE Tasks ADD COLUMN description TEXT;
                  "#
        ),
    ]);
}
pub fn init_db() -> Result<Connection> {
    let mut conn = Connection::open("tasks.db")?;

    // Update the database schema, atomically
    MIGRATIONS.to_latest(&mut conn).unwrap();

    Ok(conn)
}

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn create_data_base() {
    let mut conn = init_db().unwrap();    
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
    let mut conn = init_db().unwrap();    


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
    let mut conn: Connection = init_db().unwrap();    
    let som = conn.execute("INSERT INTO Tasks (task_name) VALUES (?1)", (&task,))?;
    Ok(())
}
