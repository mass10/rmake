/// コマンドを実行します。
pub fn shell_exec(command_fields: Vec<String>) -> std::result::Result<i32, Box<dyn std::error::Error>> {
	// プロセスを実行します。
	let mut command = std::process::Command::new("cmd");
	let mut args: Vec<String> = vec![];
	args.push("/C".to_string());
	for e in command_fields {
		args.push(e.to_string());
	}

	// 実行
	let mut command = command.args(args).spawn()?;
	let response = command.wait();
	if response.is_err() {
		return Err(Box::new(response.err().unwrap()));
	}

	// 終了ステータスを確認
	let status = response.unwrap();
	if !status.success() {
		// バッチを終了
		let exit_code = status.code().unwrap();
		println!("[ERROR] command exited with status: {}", exit_code);
		return Ok(exit_code);
		// std::process::exit(exit_code);
	}

	return Ok(0);
}

/// テキストファイル全体を読み込んで文字列で返します。
pub fn read_text_file_all(path: &str) -> std::result::Result<String, Box<dyn std::error::Error>> {
	use std::io::Read;

	let mut file = std::fs::File::open(path)?;
	let mut s = String::new();
	file.read_to_string(&mut s)?;
	return Ok(s);
}

pub fn select(left: &str, right: &str) -> String {
	return match left {
		"" => String::from(right),
		_ => String::from(left),
	};
}
