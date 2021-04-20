//!
//! Configuration
//!

extern crate serde_derive;

use crate::util::functions;

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
	command: Option<Vec<String>>,
}

impl Task {
	/// Returns name
	///
	/// ### Returns
	/// Task name
	pub fn get_name(&self) -> String {
		if self.name.is_none() {
			return String::new();
		}
		return self.name.clone().unwrap();
	}

	/// Returns description
	///
	/// ### Returns
	/// Description of this
	pub fn get_description(&self) -> String {
		if self.description.is_none() {
			return String::new();
		}
		return self.description.clone().unwrap();
	}

	/// Returns command and options
	///
	/// ### Returns
	/// Command
	pub fn get_command(&self) -> Vec<String> {
		if self.command.is_none() {
			return vec![];
		}
		let command = self.command.as_ref().unwrap();
		return command.clone();
	}

	/// Returns dependencies
	///
	/// ### Returns
	/// Tasks this task depends on
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
pub struct ConfigurationSettings {
	/// Environment variables
	pub env: Option<std::collections::btree_map::BTreeMap<String, String>>,

	/// Simple variables
	pub variables: Option<std::collections::btree_map::BTreeMap<String, String>>,

	/// Tasks definition
	pub tasks: Vec<Task>,
}

impl ConfigurationSettings {
	/// Returns a new instance of ConfigurationSettings
	///
	/// ### Arguments
	/// * `rmakefile_path` Path to rmake file
	///
	/// ### Returns
	/// A new instance of `ConfigurationSettings`
	pub fn new(rmakefile_path: &str) -> std::result::Result<ConfigurationSettings, Box<dyn std::error::Error>> {
		extern crate toml;

		// rmake configuration file
		let rmakefile_path = functions::select(rmakefile_path, "rmake.toml");

		// Read the whole content of given file
		println!("{} rmake [INFO] Reading rmake file ... [{}]", functions::get_timestamp(), &rmakefile_path);

		let mut content = functions::read_text_file_all(&rmakefile_path)?;

		// Read TOML file
		let conf: ConfigurationSettings = toml::from_str(&content)?;

		// Fill placeholders with variables
		if conf.variables.is_some() {
			let variables = conf.variables.as_ref().unwrap();
			for (k, v) in variables {
				println!("{} rmake [INFO] VAR [{}]=[{}]", functions::get_timestamp(), k, v);
				let place_holder = format!("{{{{{}}}}}", k);
				content = content.replace(&place_holder, &v);
			}
			if variables.len() == 0 {
				println!("{} rmake [INFO] No var defined.", functions::get_timestamp());
			}
		} else {
			println!("{} rmake [INFO] No var defined.", functions::get_timestamp());
		}

		// Re-configure by the content.
		let conf: ConfigurationSettings = toml::from_str(&content)?;

		// Retrieves and set the environment variables from configuration file
		if conf.env.is_some() {
			let env = conf.env.as_ref().unwrap();
			for (k, v) in env {
				println!("{} rmake [INFO] ENV [{}]=[{}]", functions::get_timestamp(), k, v);
				std::env::set_var(k, v);
			}
			if env.len() == 0 {
				println!("{} rmake [INFO] No env defined.", functions::get_timestamp());
			}
		} else {
			println!("{} rmake [INFO] No env defined.", functions::get_timestamp());
		}

		return Ok(conf);
	}
}
