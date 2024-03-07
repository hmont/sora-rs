use mongodb::{bson::Document, Client, Collection};
use lazy_static::lazy_static;

use crate::Config;

use async_once::AsyncOnce;

lazy_static! { // fuck rust
	static ref CONFIG: config::Config = Config::load();

    static ref MONGO: AsyncOnce<Client> = AsyncOnce::new(async {

		let host: String = CONFIG.get_string("database.host").unwrap().clone();
		let port: i64 = CONFIG.get_int("database.port").unwrap();
		let database: String = CONFIG.get_string("database.db").unwrap();
	
		let auth_enabled: bool = CONFIG.get_bool("database.authorization").unwrap();
		let username: String = CONFIG.get_string("database.username").unwrap();
		let password: String = CONFIG.get_string("database.password").unwrap();
		let auth_db: String = CONFIG.get_string("database.authDb").unwrap();
        
		let mut __uri: String = "".to_string();

		if auth_enabled { __uri = format!("mongodb://{}:{}@{}:{}/{}?authSource={}", username, password, host, port, database, auth_db); }
		else { __uri = format!("mongodb://{}:{}/{}", host, port, database); }

		//println!("{}", CONFIG.get_bool("database.authorization").unwrap().to_string()); debugging
		//println!("{}", __uri.to_string()); debugging
        let client = Client::with_uri_str(__uri).await.unwrap();

        client
    });
}

/*async fn create_mongo_client() -> Client 
{
	const HOST: String = CONFIG.get_string("database.host").unwrap();
	const PORT: i64 = CONFIG.get_int("database.port").unwrap();
	const DATABASE: String = CONFIG.get_string("database.db").unwrap();

	const AUTH_ENABLED: bool = CONFIG.get_bool("database.authorization").unwrap();
	const USERNAME: String = CONFIG.get_string("database.username").unwrap();
	const PASSWORD: String = CONFIG.get_string("database.password").unwrap();


    let mongo_connection_string = get_connection_string();
    Client::with_uri_str(&mongo_connection_string).await.unwrap()
}*/

/*async fn get_connection_string() -> String 
{
    let host = CONFIG.get_string("database.host").unwrap();
    let port = CONFIG.get_int("database.port").unwrap();
    
	if CONFIG.get_bool("database.authorization").unwrap()
	{
		let username = CONFIG.get_string("database.username").unwrap();
		let password = CONFIG.get_string("database.password").unwrap();
		format!("mongodb://{}:{}@{}:{}", username, password, host, port)
	}
	else
	{
		format!("mongodb://{}:{}", host, port)
	}
}*/

pub async fn collection(coll_name: &str) -> Collection<Document>
{
    MONGO.get().await.database(&CONFIG.get_string("database.db").unwrap()).collection(coll_name)
}