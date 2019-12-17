use rusqlite;
use rusqlite::{Connection, params, NO_PARAMS};
//use rusqlite::{NO_PARAMS, named_params};
use log::{error,debug, info};
use std::sync::Mutex;

use super::super::entity::{User, PostIdent};

// Struct for interacting with a SQLite database
pub struct LiteDB {
    conn: Mutex<Connection>,
}



impl LiteDB {
    pub fn load(file: &str) -> Self {
        let conn = Mutex::new(Connection::open(&file).expect("Unable to connect to db file!"));
        Self {
            conn
        }
    }

    /*pub fn close(&mut self) -> Result<(),&str> {
        let conn = self.conn.lock().unwrap();
        let res = conn.close();
        match res {
            Ok(_) => Ok(()),
            Err((_conn, e)) => {
                error!("Unable to close connection: {}", e);
                self.conn = Mutex::new(_conn);
                Err("Unable to close connection")
            }
        }
    }*/

    pub fn check_or_create_tables(&self) -> Result<(), String> {
        let conn = self.conn.lock().expect("Failed to get handle on the connection");
        match Self::check_table(&conn, "user") {
            Some(_) => {
                debug!("Tables already created!");
                Ok(())
            },
            None => Self::create_tables(&conn)
        }
    }

    pub fn get_users(&self) -> rusqlite::Result<Vec<User>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT u.username, u.is_admin FROM user u")?;
        let results = stmt.query_map(NO_PARAMS, |row| -> rusqlite::Result<User> {
            debug!("Fetched row...");
            Ok(
                User {
                    name: row.get_unwrap(0),
                    is_admin: row.get_unwrap(1)
                }
            )
        }).or_else(|e: rusqlite::Error| {
            error!("Error, {}", e);
            Err(e)
        })?;
        // TODO use collect
        let mut list: Vec<User> = Vec::with_capacity(3);
        for result in results {
            let val = result.unwrap();
            debug!("Result is: {:?}", &val);
            list.push(val);
        }
        Ok(list)
    }

    /**
     * Gets the raw text field
     */
    pub fn get_raw_post_by_id(&self, id: i32) -> Option<String> {
        let conn = self.conn.lock().unwrap();
        let res: rusqlite::Result<String> = conn.query_row("SELECT content FROM post WHERE id=?1", params![id],
                                 |row| -> rusqlite::Result<String> {
                                     Ok(row.get_unwrap(0))
                                 }
        );
        let ret = match res {
            Ok(val) => {
                Some(val)
            },
            Err(e) => {
                error!("Error getting raw post: {}", e);
                None
            }
        };
        ret
    }

    /**
     * Gets the list of posts (sorted by date created, descending)
     */
    pub fn get_posts(&self, limit: i32, offset: i32) -> rusqlite::Result<Vec<PostIdent>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, title, strftime(\"%s\", created_at) as created_at FROM post ORDER BY created_at DESC LIMIT ?1 OFFSET ?2")?;
        let ret = stmt.query_map(params![limit, offset], |row| -> rusqlite::Result<PostIdent> {
            debug!("Fetched PostIdent row...");
            // created_at is read as string.
            // represents unix time in seconds.
            let created_at: String = row.get_unwrap(2);
            debug!("Created at: {}", created_at);
            Ok(
                PostIdent {
                    id: row.get_unwrap(0),
                    title: row.get_unwrap(1),
                    created: created_at.parse::<i64>().or_else( |e| -> Result<i64, ()> {
                        info!("Created at cannot be read. {}, returning 0", e);
                        Ok(0 as i64)
                    }).unwrap() * 1000,
                }
            )
        }).or_else(|e: rusqlite::Error| {
            error!("Error, {}", e);
            Err(e)
        })?;
        // TODO use collect?
        let mut items = Vec::new();
        for item in ret {
            items.push(item.unwrap());
        }
        Ok(items)
    }

    fn check_table(conn: &Connection, table: &str) -> Option<()> {
        let res = conn.query_row("SELECT name FROM sqlite_master WHERE type='table' AND name=?1",
                                 params![table],
                                 |_| Ok(Some(())))
            .or_else(|e| -> Result<Option<()>, String> {
                debug!("{}", e);
                Ok(None)
            }).unwrap();

        return res;
    }

    fn create_tables(conn: &Connection) -> Result<(), String> {
        info!("Creating tables...");
        conn.execute_batch("BEGIN;
CREATE TABLE user(
  id INTEGER PRIMARY KEY,
  username TEXT UNIQUE NOT NULL,
  email TEXT UNIQUE NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  is_admin INTEGER NOT NULL
);

CREATE TABLE post(
  id INTEGER PRIMARY KEY,
  title TEXT,
  content TEXT,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  author_id INTEGER NOT NULL,
  FOREIGN KEY(author_id) REFERENCES user(id)
);

CREATE TABLE tag(
  id INTEGER PRIMARY KEY,
  name TEXT UNIQUE NOT NULL
);

CREATE TABLE post_tag(
  post_id INTEGER NOT NULL,
  tag_id  INTEGER NOT NULL,
  PRIMARY KEY(post_id, tag_id),
  FOREIGN KEY(post_id) REFERENCES post(id),
  FOREIGN KEY(tag_id) REFERENCES tag(id)
);

COMMIT;
").or_else( |e| {
            error!("{}", e);
            Err(format!("Failed to create tables. {}", e))
            }
        )
    }


}
