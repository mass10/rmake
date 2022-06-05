//!
//! Here is application entrypoint.
//!

extern crate serde_derive;

mod application;
mod configuration;
mod helper;
mod util;

/// Strengthen text
///
/// ### Arguments
/// * `text` text
///
/// ### Returns
/// Bold text
fn make_bold(text: &str) -> String {
	return format!("\x1b[1m{}\x1b[0m", text);
}

/// Shows usage
fn usage() {
	println!("-------------------------------------------------------");
	println!("{}", make_bold("NAME:"));
	println!("");
	println!("    rmake - A simple task runner like make.");
	println!("");
	println!("-------------------------------------------------------");
	println!("{}", make_bold("SYNOPSIS:"));
	println!("");
	println!("    {}", make_bold("rmake"));
	println!("");
	println!("    {}", make_bold("rmake [OPTIONS]"));
	println!("");
	println!("    {}", make_bold("rmake [taskname]"));
	println!("");
	println!("    {}", make_bold("rmake [taskname] [OPTIONS]"));
	println!("");
	println!("-------------------------------------------------------");
	println!("{}", make_bold("DESCRIPTION:"));
	println!("");
	println!("    {}", make_bold("[--help], [-h]"));
	println!("");
	println!("        Show this message.");
	println!("");
	println!("    {}", make_bold("[--version], [-v]"));
	println!("");
	println!("        Show application version.");
	println!("");
	println!("    {}", make_bold("[--file], [-f]"));
	println!("");
	println!("        Need value. Run tasks in the specified rmake file.");
	println!("");
	println!("        rmake --file \"rmake.toml\"");
	println!("");
	println!("        \"rmake.toml\" is default rmake file.");
	println!("");
	println!("    {}", make_bold("[taskname]"));
	println!("");
	println!("        Run a task named \"taskname\" in the rmake file.");
	println!("");
}

/// Show application version
fn version() {
	println!("{}", env!("CARGO_PKG_DESCRIPTION"));
	println!();
	println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
	println!();
	println!("{}", "https://crates.io/crates/make");
}

///
/// Commandline options
///
struct StartConfigurationSettings {
	/// Path to rmake file
	rmakefile_path: String,

	/// Task name to execute
	target_task: String,
}

impl StartConfigurationSettings {
	/// Reads commandline options
	pub fn configure() -> std::result::Result<StartConfigurationSettings, String> {
		// start configuration
		let mut conf = StartConfigurationSettings {
			target_task: String::new(),
			rmakefile_path: String::new(),
		};

		let mut current_option = String::new();

		// Commandline options. The 1st token is application itself.
		let args: Vec<String> = std::env::args().skip(1).collect();

		// Reading tokens
		for e in args {
			if e == "--help" || e == "-h" {
				return Err("show usage".to_string());
			}
			if e == "--version" || e == "-v" {
				return Err("show version".to_string());
			}
			if e.starts_with("--file=") || e.starts_with("-f=") {
				let (_, value) = util::functions::split_string(&e, "=");
				if value == "" {
					return Err("show usage".to_string());
				}
				conf.rmakefile_path = value;
				continue;
			}
			if e == "--file" || e == "-f" {
				current_option = e;
				continue;
			}
			if e.starts_with("-") {
				// Unknown option flag given.
				current_option.clear();
				println!("Unknown option {}", e);
				return Err("show usage".to_string());
			}

			if current_option == "--file" || current_option == "-f" {
				// Must be the path to rmake file.
				conf.rmakefile_path = e;
				current_option.clear();
				continue;
			}

			// Must be the name of a task to launch.
			conf.target_task = e;
		}

		if current_option != "" {
			// No values followed option flag.
			return Err("show usage".to_string());
		}

		// Configuration valid.
		return Ok(conf);
	}
}

/// Entrypoint
fn main() {
	// read commandline options
	let result = StartConfigurationSettings::configure();
	if result.is_err() {
		let result_string = result.err().unwrap();
		if result_string == "show usage" {
			usage();
		} else if result_string == "show version" {
			version();
		}
		return;
	}

	let conf = result.unwrap();

	// Initialize application
	let app = application::core::Application::new();

	// Run application.
	// Configure with the rmake file given or "rmake.toml" at current directory. That is default file.
	// Application will execute the task given or the first task in rmake file.
	let result = app.start(&conf.rmakefile_path, &conf.target_task);
	if result.is_err() {
		println!("{} rmake [ERROR] Error! reason: [{}]", util::functions::get_timestamp(), result.err().unwrap());
		return;
	}
}
