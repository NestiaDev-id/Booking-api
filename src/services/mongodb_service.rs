pub struct Database {
    bookings: mongodb::Collection<crate::models::booking_model::Booking>,
    dogs: mongodb::Collection<crate::models::dog_model::Dog>,
    owners: mongodb::Collection<crate::models::owner_model::Owner>,
}

impl Database {
    pub async fn init() -> Self {
        let uri = match env::var("MONGODB_URI") {
            Ok(v) => v,
            Err(_) => "mongodb://localhost:27017".to_string(),
        };

        let client = mongodb::Client::with_uri_str(&uri)
            .await
            .unwrap();

        let db = client.database("todo_api");

        let booking: mongodb::Collection<crate::models::booking_model::Booking> = db.collection("bookings");
        let dog: mongodb::Collection<crate::models::dog_model::Dog> = db.collection("dogs");
        let owner: mongodb::Collection<crate::models::owner_model::Owner> = db.collection("owners");

        Database {
            booking,
            dog,
            owner,
        }
    }
}