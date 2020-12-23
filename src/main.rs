extern crate serde_derive;

mod application;
mod configuration;
mod functions;
mod myerror;
mod status_holder;
mod task_controller;

/// Shows usage.
fn usage() {
	println!("USAGE:");
	println!("    > rmake --help");
	println!("        Show this message.");
	println!("");
	println!("    > rmake");
	println!("        Run 1st task in rmake.toml");
	println!("");
	println!("    > rmake --file rmake.toml");
	println!("        Run 1st task in rmake.toml");
	println!("");
	println!("    > rmake TASK-1");
	println!("        Run task [[TASK-1]]");
	println!("");
	println!("    > rmake --file rmake.toml TASK-1");
	println!("        Run task [[TASK-1]]");
	println!("");
}

///
/// commandline options
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
