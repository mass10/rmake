extern crate serde_derive;

mod application;
mod configuration;
mod lib;
mod task_controller;

/// 使用方法を表示します。
fn usage() {
	println!("USAGE:");
}

/// コマンドライン引数を読み取り、コンフィギュレーションファイルへのパスを返します。
fn configure() -> (String, String) {
	let args: std::vec::Vec<String> = std::env::args().skip(1).collect();
	if args.len() == 0 {
		return (String::from("rmake.toml"), String::new());
	}
	if args.len() == 1 {
		return (String::from("rmake.toml"), String::from(&args[0]));
	}
	if args.len() == 2 {
		if args[0] == "-f" {
			return (String::from(&args[1]), String::new());
		}
	}
	if args.len() == 3 {
		if args[0] == "-f" {
			return (String::from(&args[1]), String::from(&args[2]));
		} else if args[1] == "-f" {
			return (String::from(&args[2]), String::from(&args[0]));
		}
	}
	return (String::new(), String::new());
}

/// エントリーポイント
fn main() {
	// コマンドラインオプションを読み取り
	let (path, target_task) = configure();
	if path == "" {
		usage();
		return;
	}

	// アプリケーションを初期化します。
	let app = application::Application {};

	// コマンドライン引数で要求された rmake ファイルを実行します。
	// タスクが指定されなかった場合は、先頭のタスクが対象になります。
	let result = app.start(&path, &target_task);
	if result.is_err() {
		println!("[ERROR] Error! reason: {}", result.err().unwrap());
		return;
	}
}
