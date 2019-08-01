use handlebars::Handlebars;
mod config;

use config::Config;

pub struct Datasources {
    hb: handlebars::Handlebars,
    config: Config,
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
        Self {
            hb: handlebars,
            config: Config::load()
        }
    }

    pub fn handlebars(&self) -> &handlebars::Handlebars {
        &self.hb
    }

    pub fn conf(&self) -> &Config {
        &self.config
    }
}
