use db::Db;
use rusqlite::Result;

mod db;

fn main() -> Result<()> {
    Db::init()?;

    Ok(())
}
