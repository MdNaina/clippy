use rusqlite::{Connection, Result};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct Favorite {
    pub id: i32,
    pub value: String
}

impl Favorite {
    pub fn create(conn: &Connection, value: String) -> Result<()> {
        conn.execute("INSERT INTO favorite (value) VALUES (:value)", &[
            (":value", &value)
        ]).unwrap();
        Ok(())
    }

    pub fn find_all(conn: &Connection) -> Result<Vec<Favorite>>{
        let mut stmt = conn.prepare("SELECT * FROM favorite").unwrap();
        let rows: Vec<Favorite> = stmt.query_map([], |row| {
            Ok(Favorite {
                id: row.get(0)?,
                value: row.get(1)?
            })
        })?.map(|row| {
            row.unwrap()
        }).collect();

        Ok(rows)
    }

}

