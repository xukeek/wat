#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate derivative;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

mod api_v1;
mod config;
mod db;
mod migrate;
mod models;
mod schema;
mod views;

use diesel::r2d2::{ConnectionManager, Pool};
use rocket_contrib::templates::Template;
use structopt::StructOpt;

use crate::config::Config;
use crate::db::Database;

#[derive(StructOpt)]
enum Opt {
    #[structopt(name = "generate-config")]
    GenerateConfig,

    #[structopt(name = "migrate")]
    Migrate,

    #[structopt(name = "runserver")]
    RunServer,
}

fn main() {
    let opt = Opt::from_args();
    match opt {
        Opt::GenerateConfig => {
            let config = Config::default();
            let output = toml::to_string_pretty(&config).expect("default config should not panic");
            println!("{}", output);
        }
        Opt::Migrate => (),
        Opt::RunServer => {
            let config = Config::read().unwrap();

            rocket::ignite()
                .attach(Database::fairing())
                .attach(Template::fairing())
                .mount("/api/v1", api_v1::routes())
                .mount("/", views::routes())
                .launch();
        }
    }
}
