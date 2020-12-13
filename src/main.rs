extern crate serde_derive;

mod application;
mod configuration;
mod lib;
mod task_controller;

/// 使用方法を表示します。
fn usage() {
	println!("USAGE:");
	println!("    rmake");
}

#[derive(std::clone::Clone)]
struct CommandlineConfiguration {
	/// rmake ファイルへのパス
	rmakefile_path: String,
	/// ターゲットのタスク
	target_task: String,
}

/// コマンドライン引数を読み取ります。
fn configure() -> Option<CommandlineConfiguration> {
	let mut conf = CommandlineConfiguration {
		target_task: String::new(),
		rmakefile_path: String::new(),
	};
	let mut current_scope = String::new();

	let args: Vec<String> = std::env::args().skip(1).collect();
	for e in args {
		if e == "--file" {
			current_scope = e;
			continue;
		}
		if e == "-f" {
			current_scope = e;
			continue;
		}
		if e.starts_with("-") {
			current_scope.clear();
			println!("Unknown option {}", e);
			return None;
		}
		if current_scope == "-f" || current_scope == "--file" {
			current_scope.clear();
			conf.rmakefile_path = e;
			continue;
		}
		conf.target_task = e;
	}

	return Some(conf);
}

/// エントリーポイント
fn main() {
	// コマンドラインオプションを読み取り
	let result = configure();
	if result.is_none() {
		usage();
		return;
	}

	let conf = result.unwrap();

	// アプリケーションを初期化します。
	let app = application::Application {};

	// コマンドライン引数で要求された rmake ファイルを実行します。
	// タスクが指定されなかった場合は、先頭のタスクが対象になります。
	let result = app.start(&conf.rmakefile_path, &conf.target_task);
	if result.is_err() {
		println!("[ERROR] Error! reason: {}", result.err().unwrap());
		return;
	}
}
