#[allow(unused)]
/// yarn の実行
fn yarn(args: &[&str]) -> std::result::Result<(), Box<dyn std::error::Error>> {
	// yarn プロセスを実行します。
	let mut command = std::process::Command::new("C:\\Program Files (x86)\\Yarn\\bin\\yarn.cmd");
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
		println!("[WARN] yarn exited with status: {}", exit_code);
		std::process::exit(exit_code);
	}
	return Ok(());
}

/// テキストファイル全体を読み込んで文字列で返します。
pub fn read_text_file_all(path: &str) -> std::result::Result<String, Box<dyn std::error::Error>> {
	use std::io::Read;

	let mut file = std::fs::File::open(path)?;
	let mut s = String::new();
	file.read_to_string(&mut s)?;
	return Ok(s);
}
