//!
//! # rmake
//!
//! `rmake` is a simple task runner like `make`.
//!

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

///
/// Commandline options
///
#[derive(Clone)]
struct CommandlineConfiguration {
	/// rmake ファイルへのパス
	rmakefile_path: String,
	/// ターゲットのタスク
	target_task: String,
}

/// Reads commandline options
fn configure() -> std::result::Result<CommandlineConfiguration, ()> {
	let mut conf = CommandlineConfiguration {
		target_task: String::new(),
		rmakefile_path: String::new(),
	};
	let mut current_scope = String::new();

	let args: Vec<String> = std::env::args().skip(1).collect();
	for e in args {
		if e == "--help" || e == "-h" {
			return Err(());
		}
		if e.starts_with("--file=") || e.starts_with("-f=") {
			let (_, value) = functions::split_string(&e, "=");
			if value == "" {
				return Err(());
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
			return Err(());
		}
		if current_scope == "-f" || current_scope == "--file" {
			current_scope.clear();
			conf.rmakefile_path = e;
			continue;
		}
		conf.target_task = e;
	}

	if current_scope != "" {
		return Err(());
	}

	return Ok(conf);
}

/// Entrypoint
fn main() {
	// read commandline options
	let result = configure();
	if result.is_err() {
		usage();
		return;
	}

	let conf = result.unwrap();

	// Initialize application
	let app = application::Application::new();

	// コマンドライン引数で要求された rmake ファイルを実行します。
	// タスクが指定されなかった場合は、先頭のタスクが対象になります。
	let result = app.start(&conf.rmakefile_path, &conf.target_task);
	if result.is_err() {
		println!("{} [ERROR] Error! reason: {}", functions::get_timestamp(), result.err().unwrap());
		return;
	}
}
