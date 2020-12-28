extern crate serde_derive;

mod application;
mod application_error;
mod configuration;
mod functions;
mod status_holder;
mod task_controller;

/// Strengthen text
fn make_bold(s: &str) -> String {
	return format!("\x1b[1m{}\x1b[0m", s);
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
#[derive(Clone)]
struct CommandlineConfiguration {
	/// Path to rmake file
	rmakefile_path: String,
	/// Task name to execute
	target_task: String,
}

/// Reads commandline options
fn configure() -> std::result::Result<CommandlineConfiguration, String> {
	let mut conf = CommandlineConfiguration {
		target_task: String::new(),
		rmakefile_path: String::new(),
	};
	let mut current_scope = String::new();

	let args: Vec<String> = std::env::args().skip(1).collect();
	for e in args {
		if e == "--help" || e == "-h" {
			return Err("show usage".to_string());
		}
		if e == "--version" || e == "-v" {
			return Err("show version".to_string());
		}
		if e.starts_with("--file=") || e.starts_with("-f=") {
			let (_, value) = functions::split_string(&e, "=");
			if value == "" {
				return Err("show usage".to_string());
			}
			conf.rmakefile_path = value;
			continue;
		}
		if e == "--file" || e == "-f" {
			current_scope = e;
			continue;
		}
		if e.starts_with("-") {
			current_scope.clear();
			println!("Unknown option {}", e);
			return Err("show usage".to_string());
		}
		if current_scope == "-f" || current_scope == "--file" {
			current_scope.clear();
			conf.rmakefile_path = e;
			continue;
		}
		conf.target_task = e;
	}

	if current_scope != "" {
		return Err("show usage".to_string());
	}

	return Ok(conf);
}

/// Entrypoint
fn main() {
	// read commandline options
	let result = configure();
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
	let app = application::Application::new();

	// Run application.
	// Configure with the rmake file given or "rmake.toml" at current directory. That is default file.
	// Application will execute the task given or the first task in rmake file.
	let result = app.start(&conf.rmakefile_path, &conf.target_task);
	if result.is_err() {
		println!("{} [ERROR] Error! reason: {}", functions::get_timestamp(), result.err().unwrap());
		return;
	}
}
