use rusqlite::{Connection, Result};

pub(crate) const TABLE: &str = r"
CREATE TABLE IF NOT EXISTS websites(
    websiteID int PRIMARY KEY,
    url varchar(255) NOT NULL UNIQUE
);
";

pub struct WebsitesItem {
    id: usize,
    url: String,
}

impl WebsitesItem {
    fn new(url: impl Into<String>) -> Self {
        Self {
            id: usize::MAX,
            url: url.into(),
        }
    }

    fn insert(&self, conn: &Connection) -> Result<()> {
        conn.execute("INSERT INTO queue (url) VALUES (?1)", [&self.url])?;

        unimplemented!()
    }
}
