use handlebars::Handlebars;
mod config;
mod lite_db;
pub mod solver;

use config::Config;
use lite_db::LiteDB;

use log::info;

pub struct Datasources {
    hb: handlebars::Handlebars<'static>,
    config: Config,
    db: LiteDB,
}

impl Datasources {
    pub fn new() -> Self {
        // Handlebars uses a repository for the compiled templates. This object must be
        // shared between the application threads, and is therefore passed to the
        // Application Builder as an atomic reference-counted pointer.
        info!("Loading handlebars...");
        let mut handlebars = Handlebars::new();
        handlebars
            .register_templates_directory(".hbs", "./static/templates")
            .unwrap();
        handlebars.set_strict_mode(true);
        info!("Handlebars loaded!");
        info!("Loading config...");
        let config = Config::load();
        info!("Config loaded!");
        info!("Loading database...");
        let db = LiteDB::load(&config.db_file);
        info!("Database loaded!");
        db.check_or_create_tables().expect("Failed to create tables!");
        Self {
            hb: handlebars,
            config,
            db,
        }
    }

    pub fn handlebars(&self) -> &handlebars::Handlebars<'static> {
        &self.hb
    }

    pub fn conf(&self) -> &Config {
        &self.config
    }

    pub fn db(&self) -> &LiteDB {
        &self.db
    }
}