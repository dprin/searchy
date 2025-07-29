mod analysed;
mod websites;

use rusqlite::{Connection, Result};

pub struct Db {
    pub connection: Connection,
}

impl Db {
    pub(crate) fn init() -> Result<Self> {
        let conn = Connection::open("./db.sqlite3")?;

        // TODO: make sure that the tables exist
        conn.execute(websites::TABLE, ())?;
        conn.execute(analysed::TABLE, ())?;

        Ok(Self { connection: conn })
    }

    pub fn new() -> Result<Self> {
        let conn = Connection::open("./db.sqlite3")?;

        Ok(Self { connection: conn })
    }
}
