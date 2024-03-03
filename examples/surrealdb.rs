use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[derive(Debug, Deserialize, Serialize)]
struct Name {
    first: String,
    last: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct Person {
    title: String,
    name: Name,
    marketing: bool,
}

#[derive(Debug, Serialize)]
struct Responsibility {
    marketing: bool,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    // Connect to the server
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    // Select a specific namespace / database
    db.use_ns("test").use_db("test").await?;

    // Create a new person with a random id
    let created: Vec<Record> = db
        .create("person")
        .content(Person {
            title: "Founder & CEO".to_owned(),
            name: Name {
                first: "Tobie".to_owned(),
                last: "Morgan Hitchcock".to_owned(),
            },
            marketing: true,
        })
        .await?;
    dbg!(created);

    // Update a person record with a specific id
    // let updated: Record = db
    //     .update(("person", "jaime"))
    //     .merge(Responsibility { marketing: true })
    //     .await?;
    // dbg!(updated);

    // Select all people records
    let people: Person = db.select(("person", "juan")).await?.unwrap();

    dbg!(people);

    // Perform a custom advanced query
    // let groups = db
    //     .query("SELECT marketing, count() FROM type::table($table) GROUP BY marketing")
    //     .bind(("table", "person"))
    //     .await?;
    // dbg!(groups);

    Ok(())
}
