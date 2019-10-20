use handlebars::Handlebars;
mod config;
mod lite_db;

use config::Config;
use lite_db::LiteDB;

pub struct Datasources {
    hb: handlebars::Handlebars,
    config: Config,
    db: LiteDB
}

impl Datasources {
    pub fn new() -> Self {
        // Handlebars uses a repository for the compiled templates. This object must be
        // shared between the application threads, and is therefore passed to the
        // Application Builder as an atomic reference-counted pointer.
        let mut handlebars = Handlebars::new();
        handlebars
            .register_templates_directory(".hbs", "./static/templates")
            .unwrap();
        handlebars.set_strict_mode(true);
        let config = Config::load();
        let db = LiteDB::load(&config.db_file);
        db.check_or_create_tables().expect("Failed to create tables!");
        Self {
            hb: handlebars,
            config,
            db
        }
    }

    pub fn handlebars(&self) -> &handlebars::Handlebars {
        &self.hb
    }

    pub fn conf(&self) -> &Config {
        &self.config
    }

    pub fn db(&self) -> &LiteDB {
        &self.db
    }
}
