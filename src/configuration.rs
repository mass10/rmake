extern crate serde_derive;

use super::functions;

///
/// Task definition
///
#[derive(serde_derive::Deserialize, Debug, std::clone::Clone)]
pub struct Task {
	/// Its name
	name: Option<String>,
	/// Description
	description: Option<String>,
	/// Dependencies
	depends_on: Option<Vec<String>>,
	/// Command and options
	command: Option<Vec<Vec<String>>>,
}

impl Task {
	/// Returns name
	#[allow(unused)]
	pub fn get_name(&self) -> String {
		if self.name.is_none() {
			return String::new();
		}
		return self.name.clone().unwrap();
	}

	/// Returns description
	#[allow(unused)]
	pub fn get_description(&self) -> String {
		if self.description.is_none() {
			return String::new();
		}
		return self.description.clone().unwrap();
	}

	/// Returns command and options
	#[allow(unused)]
	pub fn get_command(&self) -> Vec<Vec<String>> {
		if self.command.is_none() {
			let result: Vec<Vec<String>> = vec![];
			return result.clone();
		}
		let command = self.command.as_ref().unwrap();
		return command.clone();
	}

	/// Returns dependencies
	#[allow(unused)]
	pub fn get_depends_on(&self) -> Vec<String> {
		if self.depends_on.is_none() {
			let result: Vec<String> = vec![];
			return result.clone();
		}
		let depends_on = self.depends_on.as_ref().unwrap();
		return depends_on.clone();
	}
}

///
/// Configuration structure
///
#[derive(serde_derive::Deserialize, Debug, std::clone::Clone)]
pub struct Configuration {
	/// Variables
	pub env: Option<std::collections::btree_map::BTreeMap<String, String>>,
	/// Tasks definition
	pub tasks: Vec<Task>,
}

impl Configuration {
	/// Returns a new instance of Configuration
	pub fn new(rmakefile_path: &str) -> std::result::Result<Configuration, Box<dyn std::error::Error>> {
		extern crate toml;

		// rmake configuration file
		let rmakefile_path = functions::select(rmakefile_path, "rmake.toml");

		// Read the whole content of given file
		println!("{} [TRACE] Reading rmake file ... [{}]", functions::get_timestamp(), &rmakefile_path);
		println!();
		let content = functions::read_text_file_all(&rmakefile_path)?;

		// TODO: REPLACE ENV VARS PLACEHOLDERS IN THE RMAKE FILE
		// {

		// }

		// Read TOML file
		let conf: Configuration = toml::from_str(&content)?;

		// Retrieves and set the environment variables from configuration file
		// TODO: REMOVE
		// if conf.env.is_some() {
		// 	let env = conf.env.as_ref().unwrap();
		// 	for (k, v) in env {
		// 		println!("{} [TRACE] ENVIRONMENT [{}]=[{}]", functions::get_timestamp(), k, v);
		// 		std::env::set_var(k, v);
		// 	}
		// }

		return Ok(conf);
	}
}
