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
    // MIGRATIONS.to_version(&mut conn, 1).unwrap();
    Ok(conn)
}

