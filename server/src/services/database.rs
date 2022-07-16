use mongodb::bson::oid::ObjectId;
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client, Collection, Database,
};
use std::error::Error;
use std::{env, str::FromStr};

pub async fn connect() -> Result<Database, Box<dyn Error>> {
    let client_uri = env::var("MONGODB_URI").expect("Mising MONGODB_URI");
    let database_name = match env::var("DATABASE_NAME") {
        Ok(uri) => uri.to_string(),
        Err(_) => "jajanken-staging".to_string(),
    };
    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    let client = Client::with_options(options)?;
    let database = client.database(&database_name);
    Ok(database)
}

pub fn get_collection<T>(database: &Database, collection_name: &str) -> Collection<T> {
    let collection = database.collection(collection_name);
    collection
}

pub fn get_new_object_id() -> ObjectId {
    ObjectId::new()
}

pub fn get_new_id_string() -> String {
    let id = ObjectId::new();
    id.to_string()
}

pub fn get_object_id(id: &str) -> ObjectId {
    let object_id = ObjectId::from_str(id).unwrap();
    object_id
}

pub fn get_object_id_string(id: &ObjectId) -> String {
    id.to_string()
}