extern crate chrono;

use super::functions;
use super::myerror;

/// Return system timestamp
pub fn get_timestamp() -> String {
	let date = chrono::Local::now();
	return format!("{}", date.format("%Y-%m-%d %H:%M:%S%.3f"));
}

/// Detect Windows OS
fn is_windows_os() -> std::result::Result<bool, Box<dyn std::error::Error>> {
	let result = std::process::Command::new("cmd").arg("/C").arg("echo").arg("%OS%").output();
	if result.is_err() {
		// cannot create command
		return Ok(false);
	}
	let output = result.unwrap();
	let output = String::from_utf8(output.stdout.to_vec())?;
	if !output.contains("Windows") {
		return Ok(false);
	}
	return Ok(true);
}

/// Detect Linux OS
fn is_linux_os() -> std::result::Result<bool, Box<dyn std::error::Error>> {
	let result = std::process::Command::new("sh").arg("-c").arg("uname").output();
	if result.is_err() {
		// cannot create command
		return Ok(false);
	}
	let output = result.unwrap();
	let output = String::from_utf8(output.stdout.to_vec())?;
	if !output.contains("Linux") {
		return Ok(false);
	}
	return Ok(true);
}

/// execute command in os shell
pub fn shell_exec(commands: &String) -> std::result::Result<i32, Box<dyn std::error::Error>> {
	// Try to execute command for Windows
	if is_windows_os()? {
		let mut command = std::process::Command::new("cmd");
		command.arg("/C");
		command.arg(commands);
		let mut response = command.spawn()?;
		let status = response.wait()?;
		if !status.success() {
			let exit_code = status.code().unwrap();
			println!("{} [ERROR] command exited with status: {}", functions::get_timestamp(), exit_code);
			return Ok(exit_code);
		}
		return Ok(0);
	}

	// Create command for somewhere else
	if is_linux_os()? {
		let mut command = std::process::Command::new("sh");
		command.arg("-c");
		command.arg(commands);
		let mut response = command.spawn()?;
		let status = response.wait()?;
		if !status.success() {
			let exit_code = status.code().unwrap();
			println!("{} [ERROR] command exited with status: {}", functions::get_timestamp(), exit_code);
			return Ok(exit_code);
		}
		return Ok(0);
	}

	return Err(Box::new(myerror::ApplicationError::new("Unrecognized environment".to_string())));
}

/// Retrieve the whole content of file
pub fn read_text_file_all(path: &str) -> std::result::Result<String, Box<dyn std::error::Error>> {
	use std::io::Read;

	let mut file = std::fs::File::open(path)?;
	let mut s = String::new();
	file.read_to_string(&mut s)?;
	return Ok(s);
}

/// Returns right if left was empty
pub fn select(left: &str, right: &str) -> String {
	return match left {
		"" => String::from(right),
		_ => String::from(left),
	};
}
