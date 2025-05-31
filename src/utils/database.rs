use mongodb::{bson::Document, Client, Collection};
use lazy_static::lazy_static;

use crate::Config;

use async_once::AsyncOnce;

lazy_static! { // i hate rust
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

pub async fn collection(coll_name: &str) -> Collection<Document>
{
    MONGO.get().await.database(&CONFIG.get_string("database.db").unwrap()).collection(coll_name)
}
