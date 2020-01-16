use actix_files as fs;
use actix_web::web;
use std::path::Path;

use crate::db::DB;

pub mod server;

mod api;
#[cfg(test)]
mod tests;

fn configure_app(
	cfg: &mut web::ServiceConfig,
	web_url: &str,
	web_dir_path: &Path,
	swagger_url: &str,
	swagger_dir_path: &Path,
	db: &DB,
) {
	// TODO logging
	cfg.data(db.clone())
		.service(
			web::scope("/api")
				.service(api::get_initial_setup)
				.service(api::get_version)
				.service(api::put_settings),
		)
		.service(fs::Files::new(swagger_url, swagger_dir_path).index_file("index.html"))
		.service(fs::Files::new(web_url, web_dir_path).index_file("index.html"));
}
