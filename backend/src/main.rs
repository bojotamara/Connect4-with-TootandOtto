
#[macro_use]
extern crate bson;
use mongodb::{Client, options::ClientOptions};

fn main() -> Result<(), mongodb::error::Error> {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017")?;

    // Manually set an option.
    client_options.app_name = Some("My App".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;

    // List the names of the databases in that deployment.
    for db_name in client.list_database_names(None)? {
        println!("{}", db_name);
    }

    // Get a handle to a database.
    let db = client.database("mydb");
    // List the names of the collections in that database.
    for collection_name in db.list_collection_names(None)? {
        println!("{}", collection_name);
    }

    Ok(())
}
