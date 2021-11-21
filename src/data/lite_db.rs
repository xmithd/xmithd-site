use rusqlite;
use rusqlite::{Connection, params};
use log::{error,debug, info};
use std::sync::Mutex;

use super::super::entity::{User, PostIdent, Post};

// Struct for interacting with a SQLite database
pub struct LiteDB {
    conn: Mutex<Connection>,
}

const BLOG_TABLES_SQL: &str = "BEGIN;
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
  FOREIGN KEY(author_id) REFERENCES user(id) ON DELETE CASCADE
);

CREATE TABLE tag(
  id INTEGER PRIMARY KEY,
  name TEXT UNIQUE NOT NULL
);

CREATE TABLE post_tag(
  post_id INTEGER NOT NULL,
  tag_id  INTEGER NOT NULL,
  PRIMARY KEY(post_id, tag_id),
  FOREIGN KEY(post_id) REFERENCES post(id) ON DELETE CASCADE,
  FOREIGN KEY(tag_id) REFERENCES tag(id) ON DELETE CASCADE
);

COMMIT;
";

const CHAT_TABLES_SQL: &str = "BEGIN;
CREATE TABLE chat_room(
  id INTEGER PRIMARY KEY,
  name TEXT
);

CREATE TABLE chat_message(
  id INTEGER PRIMARY KEY,
  content TEXT,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  user_id INTEGER NOT NULL,
  FOREIGN KEY(user_id) REFERENCES user(id) ON DELETE CASCADE
);

CREATE TABLE chat_log(
  message_id INTEGER NOT NULL,
  room_id INTEGER NOT NULL,
  PRIMARY KEY(message_id, room_id)
  FOREIGN KEY(message_id) REFERENCES chat_message(id) ON DELETE CASCADE,
  FOREIGN KEY(room_id) REFERENCES chat_room(id) ON DELETE CASCADE
);
COMMIT;
";


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
                debug!("Blog Tables already created!");
                Ok(())
            },
            None => Self::create_tables(&conn, BLOG_TABLES_SQL)
        }?;
        match Self::check_table(&conn, "chat_room") {
            Some(_) => {
                debug!("Chat tables already created!");
                Ok(())
            },
            None => Self::create_tables(&conn, CHAT_TABLES_SQL)
        }
    }

    pub fn get_users(&self) -> rusqlite::Result<Vec<User>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT u.username, u.is_admin FROM user u")?;
        let results = stmt.query_map([], |row| -> rusqlite::Result<User> {
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
     * Get post by id
     */
    pub fn get_post_by_id(&self, id: i32) -> Option<Post> {
        let conn = self.conn.lock().unwrap();
        let res: rusqlite::Result<Post> = conn.query_row("SELECT id, title, strftime(\"%s\", created_at) as created_at, content, strftime(\"%s\", updated_at) as updated_at FROM post WHERE id=?1",
                 params![id],
                |row| -> rusqlite::Result<Post> {
                    let created_at: String = row.get_unwrap(2);
                    let updated_at: String = row.get_unwrap(4);
                       Ok(
                           Post {
                               ident: PostIdent {
                                   id: row.get_unwrap(0),
                                   title: row.get_unwrap(1),
                                   created: created_at.parse::<i64>().or_else( |e| -> Result<i64, ()> {
                                       info!("Created at cannot be read. {}, returning 0", e);
                                       Ok(0 as i64)
                                   }).unwrap() * 1000,
                               },
                               updated: updated_at.parse::<i64>().or_else( |e| -> Result<i64, ()> {
                                   info!("Updated at cannot be read. {}, returning 0", e);
                                   Ok(0 as i64)
                               }).unwrap() * 1000,
                               content: row.get_unwrap(3),
                           }
                       )
                   }
        );
        let ret = match res {
            Ok(val) => {
                Some(val)
            },
            Err(e) => {
                error!("Error getting post: {}", e);
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

    fn create_tables(conn: &Connection, sql: &str) -> Result<(), String> {
        info!("Creating tables...");
        conn.execute_batch(sql).or_else( |e| {
            error!("{}", e);
            Err(format!("Failed to create tables. {}", e))
            }
        )
    }


}
