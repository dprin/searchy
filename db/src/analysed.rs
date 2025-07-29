use rusqlite::{Connection, Result};

pub(crate) const TABLE: &str = r"
CREATE TABLE IF NOT EXISTS analysed (
    analysedID int PRIMARY KEY,
    url varchar(255) NOT NULL UNIQUE,
    FOREIGN KEY(analysedID) REFERENCES websites(websiteID) 
);
";

pub struct AnalysedItem {
    id: usize,
}

impl AnalysedItem {
    fn insert(&self, conn: &Connection) -> Result<()> {
        unimplemented!("Analysed Items can't be stored yet")
    }
}
