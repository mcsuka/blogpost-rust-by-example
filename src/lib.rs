// path to crate::module::type, not entirely unlike a Java import
use std::collections::HashMap;

pub struct TitleBasics {
    id: String,                     // this is a private field
    pub title_type: Option<String>, // this is a public field
    primary_title: Option<String>,
    start_year: Option<i32>,
}

// add methods to TitleBasics
impl TitleBasics {
    // convert a HashMap to TitleBasics
    pub fn from(fields: &HashMap<&str, &str>) -> TitleBasics {
        TitleBasics {
            id: fields["id"].to_string(), // map[key] will 'panic' if key is not found
            title_type: fields
                .get("title_type")        // map.get returns an Option<&str>
                .map(|&s| s.to_string()),
            primary_title: fields.get("primary_title").map(|&s| s.to_string()),
            start_year: fields
                .get("start_year")
                .map(|&s| s.parse::<i32>().ok()).flatten(),
        }
    }
    // get a detail
    // &self is an implicit alias of the structure data
    pub fn get_start_year(&self) -> &Option<i32> {
        &self.start_year
    }
    // set a detail
    pub fn set_start_year(&mut self, start_year: i32) {
        self.start_year = Some(start_year);
    }
}

// create a new instance, get a detail then set a detail:
fn struct_impl() {
    let map: HashMap<&str, &str> = HashMap::from([
        ("id", "tt000001"),
        ("title_type", "documentary"),
        ("primary_title", "The Blue Planet"),
        ("start_year", "1999"),
    ]);
    let mut tb = TitleBasics::from(&map);
    // {:?} instructs the println macro to call the Debug::fmt() method of the Option
    println!("start_year={:?}", tb.get_start_year());
    tb.set_start_year(1998);
    println!("start_year={:?}", tb.get_start_year());
}


// the following trait abstracts the access to a database row.
// It may be implemented for different databases or for unit testing without a database.
pub trait DbRow {
    fn opt_string(&self, column: &str) -> Option<String>;
    fn opt_i32(&self, column: &str) -> Option<i32>;
}

// add a from_db_row method to TitleBasics
impl TitleBasics {
    pub fn from_db_row(r: &dyn DbRow) -> TitleBasics {
        TitleBasics {
            id: r.opt_string("tconst").unwrap(),
            title_type: r.opt_string("titletype"),
            primary_title: r.opt_string("primarytitle"),
            start_year: r.opt_i32("startyear"),
        }
    }
}

use rocket_db_pools::sqlx;
use rocket_db_pools::sqlx::{Error, PgPool, Row, postgres::PgRow};

// Implement DbRow for Postgres
// Although PgRow is coming from an external library, we can extend it,
// a bit like implicit classes in Scala2
impl DbRow for PgRow {
    fn opt_string(&self, column: &str) -> Option<String> {
        self.try_get::<String, &str>(column).ok()
    }
    fn opt_i32(&self, column: &str) -> Option<i32> {
        self.try_get::<i32, &str>(column).ok()
    }
}

// Use DB row for querying a DB table, with the rocket_db_pools library
// "async" is an asynchronous function, practically meaning it returns a Future
pub async fn query_title_basics(db_pool: &PgPool, id: &str) -> Result<TitleBasics, Error> {
    sqlx::query("SELECT * FROM title_basics WHERE tconst = $1")
        .bind(id)
        .fetch_one(db_pool)
        .await
        .and_then(|row: PgRow| Ok(TitleBasics::from_db_row(&row)))
}



#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{TitleBasics, DbRow};

// A mock DB row used for unit testing
struct TestDbRow<'r> {
    map: HashMap<&'static str, &'r str>,
}

// Implement DbRow for the mock DB row
impl<'r> DbRow for TestDbRow<'r> {
    fn opt_string(&self, column: &str) -> Option<String> {
        self.map.get(column).map(|x| x.to_string())
    }
    fn opt_i32(&self, column: &str) -> Option<i32> {
        self.map.get(column).map(|x| x.parse::<i32>().unwrap())
    }
}

    // Test TitleBasics::from_db_row() without a database:
    // #[test] is an annotation macro
    #[test]
    fn test_title_basics_from_db_row() {
        let values = HashMap::from([("tconst", "abcd")]);
        let test_row = TestDbRow { map: values };
        let title_basics = TitleBasics::from_db_row(&test_row);

        assert!(title_basics.id == "abcd");
        // ...
    }
}