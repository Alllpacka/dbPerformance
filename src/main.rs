use fake::faker::address::en::CityName;
use fake::faker::internet::en::SafeEmail;
use fake::faker::name::en::{FirstName, LastName};
use fake::faker::phone_number::en::PhoneNumber;
use fake::{Dummy, Fake, Faker};
use mongodb::error::Result as MongoResult;
use mongodb::{options::ClientOptions, Client};
use postgres::NoTls;
use serde::Serialize;
use std::time::Instant;

#[derive(Serialize, Dummy, Clone)]
pub struct Data {
    #[dummy(faker = "FirstName()")]
    firstname: String,
    #[dummy(faker = "LastName()")]
    lastname: String,
    #[dummy(faker = "CityName()")]
    city: String,
    #[dummy(faker = "(18..65)")]
    age: u8,
    #[dummy(faker = "SafeEmail()")]
    email: String,
    #[dummy(faker = "PhoneNumber()")]
    phone_number: String,
}

// Function to load data into PostgreSQL
fn load_into_postgres(data: Vec<Data>) -> Result<(), postgres::Error> {
    let mut client = postgres::Client::connect(
        "host=postgresql user=postgres password=postgres dbname=fakedata port=5432", NoTls)?;

    let table_query: String = "CREATE TABLE IF NOT EXISTS data (
        id SERIAL PRIMARY KEY,
        first_name VARCHAR(255),
        last_name VARCHAR(255),
        city VARCHAR(255),
        age INT,
        email VARCHAR(255),
        phone_number VARCHAR(255)
    )"
    .to_string();

    client.execute(&table_query, &[])?;

    for d in data {
        client.execute(
            "INSERT INTO data (first_name, last_name, city, age, email, phone_number) VALUES ($1, $2, $3, $4, $5, $6)",
            &[&d.firstname, &d.lastname, &d.city, &(d.age as i32), &d.email, &d.phone_number],
        )?;
    }

    Ok(())
}

// Function to load data into MongoDB
async fn load_into_mongodb(data: Vec<Data>) -> MongoResult<()> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = Client::with_options(client_options)?;
    let db = client.database("fakeData");
    let collection = db.collection::<Data>("data");
    collection.insert_many(data, None).await?;
    Ok(())
}

// Function to select data from PostgreSQL
fn select_from_postgres() -> Result<(), postgres::Error> {
    let mut client = postgres::Client::connect(
        "host=localhost user=postgres password=postgres dbname=fakedata",
        NoTls,
    )?;

    let select_query = "SELECT * FROM data";
    let _ = client.query(select_query, &[])?;

    Ok(())
}

// Function to select data from MongoDB
async fn select_from_mongodb() -> MongoResult<()> {
    let client_options = ClientOptions::parse("mongodb://mongodb:27017").await?;
    let client = Client::with_options(client_options)?;
    let db = client.database("fakeData");
    let collection = db.collection::<Data>("data");
    let _ = collection.find(None, None).await?;

    Ok(())
}


fn main() {
    let mut data_to_insert: Vec<Data> = Vec::new();

    println!("Starting Data generation");

    let now = Instant::now();

    {
        for _ in 0..1_000_000 {
            let data: Data = Faker.fake();
            data_to_insert.push(data);
        }
    }

    let elapsed = now.elapsed();

    println!("Generated fake data.");
    println!("Took {:?}s", elapsed.as_secs());

    // Insert into PostgreSQL
    println!("\nStarting load to Postgres");

    let now = Instant::now();

    {
        match load_into_postgres(data_to_insert.clone()) {
            Ok(_) => {
                let elapsed = now.elapsed();
                println!("Data inserted into PostgreSQL successfully.");
                println!("Took {:?}s", elapsed.as_secs());
            },
            Err(err) => eprintln!("Error inserting data into PostgreSQL: {:?}", err),
        }
    }


    // Insert into MongoDB
    println!("\nStarting load into MongoDB");

    let now = Instant::now();

    {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            match load_into_mongodb(data_to_insert.clone()).await {
                Ok(_) => {
                    println!("Data inserted into MongoDB successfully.");
                },
                Err(err) => eprintln!("Error inserting data into MongoDB: {:?}", err),
            }
        });
    }
    let elapsed = now.elapsed();
    println!("haiiiiiii");
    println!("Took {:?}s", elapsed.as_secs());


    // Selects
    println!("\nTesting speed of selects without Index");

    // Select from PostgreSQL
    println!("\nSelecting data from PostgreSQL");

    let now = Instant::now();

    {
            match select_from_postgres() {
                Ok(_) => {
                    println!("Data selected from PostgreSQL successfully.");
                },
                Err(err) => eprintln!("Error selecting data from PostgreSQL: {:?}", err),
            }
    }
    let elapsed = now.elapsed();
    println!("haiiiiii :3");
    println!("Took {:?}ms", elapsed.as_millis());


    // Select from MongoDB
    println!("\nSelecting data from MongoDB");

    let now = Instant::now();

    {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            match select_from_mongodb().await {
                Ok(_) => {
                    let elapsed = now.elapsed();
                    println!("Data selected from MongoDB successfully.");
                    println!("Took {:?}ms", elapsed.as_millis());
                },
                Err(err) => eprintln!("Error selecting data from MongoDB: {:?}", err),
            }
        });
    }
}
