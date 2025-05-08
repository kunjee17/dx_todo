use std::path::PathBuf;
use std::fs;
use robius_directories::ProjectDirs;
use libsql::{Builder, Connection, Database, Error};
pub fn data_home() -> PathBuf {
    let proj_dirs = ProjectDirs::from("in.fuzzycloud", "FuzzyCloud", "todo")
        .expect("Failed to get project directories");
    let data_dir = proj_dirs.data_dir();
    if !data_dir.exists() {
        if let Err(e) = fs::create_dir_all(data_dir) {
            println!("Failed to create data directory: {:?}", e);
        }
    }
    data_dir.to_path_buf()
}
pub async fn get_db() -> Result<Database, Error> {
    let db_path = data_home().join("local.db");
    let url = "libsql://todotest-kunjee17.aws-ap-south-1.turso.io".to_string();
    let token = "".to_string();
    let db = Builder::new_synced_database(db_path, url, token)
        .build()
        .await
        .expect("Failed to build database connection");
    Ok(db)
}
pub async fn get_conn() -> Result<Connection, Error> {
    let db = get_db().await?;
    db.connect()
}
#[derive(Clone, PartialEq)]
pub struct Person {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
}
pub async fn init_db() -> Result<(), Error> {
    let conn = get_conn().await?;
    match conn
        .execute(
            "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT, 
            first_name TEXT NOT NULL, 
            last_name TEXT NOT NULL
        )",
            (),
        )
        .await
    {
        Ok(_) => {
            println!("Table created successfully");
            Ok(())
        }
        Err(e) => {
            println!("Error creating table: {:?}", e);
            Err(e)
        }
    }
}
pub async fn insert_initial_data() -> Result<(), Error> {
    let db = get_db().await?;
    let conn = db.connect()?;
    match conn
        .execute(
            "INSERT INTO users (first_name, last_name) VALUES 
        ('Raj', 'Patel'),
        ('Priya', 'Sharma'),
        ('Arjun', 'Kumar'),
        ('Neha', 'Singh'),
        ('Aditya', 'Gupta'),
        ('Meera', 'Reddy')",
            (),
        )
        .await
    {
        Ok(stmt) => {
            println!("Initial data inserted successfully {:?}", stmt);
            db.sync().await?;
            Ok(())
        }
        Err(e) => {
            println!("Error inserting data: {:?}", e);
            Err(e)
        }
    }
}
pub async fn sync_db() -> Result<(), Error> {
    let db = get_db().await?;
    println!(
        ".............................................................................",
    );
    let res = db.sync().await?;
    println!("Synced database: {:?}", res);
    Ok(())
}
pub async fn check_if_data_exists() -> Result<bool, Error> {
    let conn = get_conn().await?;
    match conn.query("SELECT COUNT(*) as count FROM users", ()).await {
        Ok(mut rows) => {
            if let Some(row) = rows.next().await? {
                let count: i64 = row.get(0)?;
                Ok(count == 0)
            } else {
                Ok(true)
            }
        }
        Err(e) => {
            println!("Error checking data existence: {:?}", e);
            Err(e)
        }
    }
}
pub async fn get_people() -> Result<Vec<Person>, Error> {
    let conn = get_conn().await?;
    match conn.query("SELECT * FROM users", ()).await {
        Ok(mut rows) => {
            let mut people = Vec::new();
            while let Some(row) = rows.next().await? {
                people
                    .push(Person {
                        id: row.get(0)?,
                        first_name: row.get(1)?,
                        last_name: row.get(2)?,
                    });
            }
            Ok(people)
        }
        Err(e) => {
            println!("Error getting people: {:?}", e);
            Err(e)
        }
    }
}
pub async fn delete_all_people() -> Result<(), Error> {
    let conn = get_conn().await?;
    match conn.execute("DELETE FROM users", ()).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
