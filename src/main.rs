use sqlx::Row;
use std::env;
use std::error::Error;
use std::io;

#[derive(Debug)]
struct Task {
    pub title: String,
    pub description: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct QuriedTasks {
    pub id: i32,
    pub title: String,
    pub description: String,
}

async fn add_task(task: &Task, conn: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO tasks_table (title, description, updated_at) VALUES ($1, $2, NOW())";

    sqlx::query(&query)
        .bind(&task.title)
        .bind(&task.description)
        .execute(conn)
        .await?;

    println!("task create successfully");

    Ok(())
}

async fn create_task(conn: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let mut task = String::new();
    let mut description = String::new();

    println!("Enter you Title");
    io::stdin().read_line(&mut task).expect("Cannot read line");
    println!("Enter you descrition");
    io::stdin()
        .read_line(&mut description)
        .expect("Cannot read line");

    let task = Task {
        title: task.trim().to_string(),
        description: description.trim().to_string(),
    };

    add_task(&task, &conn).await?;

    Ok(())
}

async fn get_all_tasks(conn: &sqlx::PgPool) -> Result<Vec<QuriedTasks>, Box<dyn Error>> {
    let q = "SELECT * FROM tasks_table";
    let query = sqlx::query(q);
    let rows = query.fetch_all(conn).await?;
    let tasks = rows
        .iter()
        .map(|row| QuriedTasks {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
        })
        .collect();
    Ok(tasks)
}

async fn show_all_tasks(conn: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    match get_all_tasks(&conn).await {
        Ok(tasks) => {
            let json_output = serde_json::to_string_pretty(&tasks).unwrap();
            println!("{}", json_output);
        }
        Err(e) => {
            eprintln!("Error fetching tasks: {}", e);
        }
    }
    Ok(())
}
async fn get_database_url(key: &str) -> Result<String, Box<dyn Error>> {
    // Attempt to get the environment variable
    let url = env::var(key.trim())
        .map(|value| value.trim().to_string())
        .map_err(|e| {
            println!("Couldn't read [{}]: {}", key, e); // Log the error
            e
        })?;

    Ok(url)
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    let url = get_database_url(&"DATABASE_URL".to_string()).await?;
    let conn = sqlx::postgres::PgPool::connect(&url).await?;

    sqlx::migrate!("./migrations").run(&conn).await?;

    loop {
        let mut choice = String::new();
        println!("Enter your Choice");
        println!("(1). Add Tasks");
        println!("(2). Show All Tasks");
        io::stdin()
            .read_line(&mut choice)
            .expect("Cannot read line");

        if choice.trim().to_string() == "1" {
            println!("adding task");
            create_task(&conn).await?;
            break;
        }
        if choice.trim().to_string() == "2" {
            println!("Showing all tasks");
            show_all_tasks(&conn).await?;
            break;
        }
        println!("Wrong choice")
    }
    Ok(())
}
