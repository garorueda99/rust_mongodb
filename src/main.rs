use bson::{doc, Document};
use mongodb::{Client, options::{ClientOptions, ResolverConfig}};
use std::env;
use std::error::Error;
use tokio;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
   // Load the MongoDB connection string from an environment variable:
   let client_uri =
      env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

   // A Client is needed to connect to MongoDB:
   // An extra line of code to work around a DNS issue on Windows:
   let options =
      ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
         .await?;
   let client = Client::with_options(options)?;

   // Print the databases in our MongoDB cluster:
   println!("Databases:");
   for name in client.list_database_names(None, None).await? {
      println!("- {}", name);
   }


   let new_doc = doc! {
    "title": "Parasite",
    "year": 2020,
    "plot": "A poor family, the Kims, con their way into becoming the servants of a rich family, the Parks. But their easy life gets complicated when their deception is threatened with exposure."
 };

 let movies = client.database("aheevaccs_1").collection::<Document>("Contacts");

 let insert_result = movies.insert_one(new_doc.clone(), None).await?;
println!("New document ID: {}", insert_result.inserted_id);

   Ok(())
}