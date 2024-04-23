use fake::faker::address::en::CityName;
use fake::faker::internet::en::SafeEmail;
use fake::faker::name::en::{FirstName, LastName};
use fake::faker::phone_number::en::PhoneNumber;
use fake::{Dummy, Fake, Faker};
use mongodb::error::Result as MongoResult;
use mongodb::{Client, IndexModel, bson, Database, Collection};
use serde::Serialize;
use std::time::Instant;
use mongodb::bson::{doc, Document};

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
async fn load_into_postgres(data: Vec<Data>) -> Result<(), tokio_postgres::Error> {
    let (mut client, connection) =
        tokio_postgres::connect("postgresql://postgres:postgres@localhost/fakedata", tokio_postgres::NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });


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

    client.execute(&table_query, &[]).await?;

    let transaction = client.transaction().await?;


    let statement = transaction
        .prepare(
            "INSERT INTO data (first_name, last_name, city, age, email, phone_number) VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .await?;

    for d in data {
        transaction
            .execute(
                &statement,
                &[
                    &d.firstname,
                    &d.lastname,
                    &d.city,
                    &(d.age as i32),
                    &d.email,
                    &d.phone_number,
                ],
            )
            .await?;
    }

    transaction.commit().await?;

    Ok(())
}

// Function to load data into MongoDB
async fn load_into_mongodb(data: Vec<Data>) -> MongoResult<()> {
    let client = Client::with_uri_str("mongodb://localhost:27017/").await?;
    let db: Database = client.database("fakedata");
    let collection: Collection<Document> = db.collection("data");

    let user_documents: Vec<Document> = data
        .into_iter()
        .map(|user| bson::to_document(&user).unwrap())
        .collect();

    collection.insert_many(user_documents, None).await?;

    Ok(())
}

// Function to select data from PostgreSQL
async fn select_from_postgres() -> Result<(), postgres::Error> {
    let (client, connection) =
        tokio_postgres::connect("postgresql://postgres:postgres@localhost/fakedata", tokio_postgres::NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let select_query = "SELECT * FROM data";
    let _ = client.query(select_query, &[]).await?;

    Ok(())
}

// Function to select data from MongoDB
async fn select_from_mongodb() -> MongoResult<()> {
    let client = Client::with_uri_str("mongodb://localhost:27017/").await?;
    let db: Database = client.database("fakedata");
    let collection: Collection<Document> = db.collection("data");

    collection.find(None, None).await?;

    Ok(())
}

// Function to create indexes in PostgreSQL
async fn create_indexes_postgres() -> Result<(), postgres::Error> {
    let (client, connection) =
        tokio_postgres::connect("postgresql://postgres:postgres@localhost/fakedata", tokio_postgres::NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Create indexes
    client.execute("CREATE INDEX data_index ON data (lower(first_name))", &[]).await?;

    Ok(())
}

// Function to create indexes in MongoDB
async fn create_indexes_mongodb() -> MongoResult<()> {
    let client = Client::with_uri_str("mongodb://localhost:27017/").await?;
    let db: Database = client.database("fakedata");
    let collection: Collection<Document> = db.collection("data");

    // Define the index model
    let index_model = IndexModel::builder()
        .keys(doc! { "first_name": 1 })
        .build();

    // Create the index
    collection.create_index(index_model, None).await?;

    Ok(())
}


#[tokio::main]
async fn main() {
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
        match load_into_postgres(data_to_insert.clone()).await {
            Ok(_) => {
                let elapsed = now.elapsed();
                println!("Data inserted into PostgreSQL successfully.");
                println!("Took {:?}s", elapsed.as_secs());
            },
            Err(err) => println!("Error inserting data into PostgreSQL: {:?}", err),
        }
    }


    // Insert into MongoDB
    println!("\nStarting load into MongoDB");

    let now = Instant::now();

    {
            match load_into_mongodb(data_to_insert.clone()).await {
                Ok(_) => {
                    let elapsed = now.elapsed();
                    println!("Data inserted into MongoDB successfully.");
                    println!("Took {:?}s", elapsed.as_secs());
                },
                Err(err) => println!("Error inserting data into MongoDB: {:?}", err),
            }
    }



    // Selects
    println!("\nTesting speed of selects without Index");

    // Select from PostgreSQL
    println!("\nSelecting data from PostgreSQL");

    let now = Instant::now();

    {
            match select_from_postgres().await {
                Ok(_) => {
                    let elapsed = now.elapsed();
                    println!("Data selected from PostgreSQL successfully.");
                    println!("Took {:?}ms", elapsed.as_millis());
                },
                Err(err) => eprintln!("Error selecting data from PostgreSQL: {:?}", err),
            }
    }


    // Select from MongoDB
    println!("\nSelecting data from MongoDB");

    let now = Instant::now();

    {
            match select_from_mongodb().await {
                Ok(_) => {
                    let elapsed = now.elapsed();
                    println!("Data selected from Mongo DB successfully.");
                    println!("Took {:?}ms", elapsed.as_millis());
                },
                Err(err) => eprintln!("Error selecting data from MongoDB: {:?}", err),
            }
    }

    println!("\nCreating Indexes");

    // Create indexes for PostgreSQL
    println!("\nCreating indexes for PostgreSQL");

    let now = Instant::now();
    match create_indexes_postgres().await {
        Ok(_) => {
            let elapsed = now.elapsed();
            println!("Indexes created in PostgreSQL successfully.");
            println!("Took {:?}ms", elapsed.as_millis());
        }
        Err(err) => eprintln!("Error creating indexes in PostgreSQL: {:?}", err),
    }

    // Create indexes for MongoDB
    println!("\nCreating indexes for MongoDB");

    let now = Instant::now();
    match create_indexes_mongodb().await {
        Ok(_) => {
            let elapsed = now.elapsed();
            println!("Indexes created in MongoDB successfully.");
            println!("Took {:?}ms", elapsed.as_millis());
        }
        Err(err) => eprintln!("Error creating indexes in MongoDB: {:?}", err),
    }

    println!("\nTesting speed of selects with Index");

    // Select from PostgreSQL
    println!("\nSelecting data from PostgreSQL");

    let now = Instant::now();

    {
        match select_from_postgres().await {
            Ok(_) => {
                let elapsed = now.elapsed();
                println!("Data selected from PostgreSQL successfully.");
                println!("Took {:?}ms", elapsed.as_millis());
            },
            Err(err) => eprintln!("Error selecting data from PostgreSQL: {:?}", err),
        }
    }


    // Select from MongoDB
    println!("\nSelecting data from MongoDB");

    let now = Instant::now();

    {
        match select_from_mongodb().await {
            Ok(_) => {
                let elapsed = now.elapsed();
                println!("Data selected from Mongo DB successfully.");
                println!("Took {:?}ms", elapsed.as_millis());
            },
            Err(err) => eprintln!("Error selecting data from MongoDB: {:?}", err),
        }
    }
}
