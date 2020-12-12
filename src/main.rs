extern crate serde_derive;

mod application;
mod configuration;
mod lib;
mod step_controller;

/// アプリケーションのコアクラス群

fn usage() {
	println!("USAGE:");
}

fn configure() -> String {
	let args: std::vec::Vec<String> = std::env::args().skip(1).collect();
	if args.len() == 0 {
		return "rmake.toml".to_string();
	}
	if args.len() == 1 {
		return String::new();
	}
	if args.len() == 2 {
		if args[0] == "-f" {
			return args[1].to_string();
		}
	}
	return String::new();
}

fn main() {
	// コマンドラインオプションを読み取り
	let path = configure();
	if path == "" {
		usage();
		return;
	}

	let app = application::Application {};
	let result = app.start(&path);
	if result.is_err() {
		println!("[ERROR] Error! reason: {}", result.err().unwrap());
		return;
	}

	println!("Ok.");
}
