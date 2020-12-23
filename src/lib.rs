fn create_command(command_fields: &Vec<String>) -> std::result::Result<std::process::Child, Box<dyn std::error::Error>> {
	// Create command for Windows
	{
		let mut command = std::process::Command::new("cmd");
		let mut args: Vec<String> = vec![];
		args.push("/C".to_string());
		for e in command_fields {
			args.push(e.to_string());
		}
		let command = command.args(args).spawn();
		if !command.is_err() {
			return Ok(command.unwrap());
		}
	}

	// Create command for somewhere else
	{
		let mut command = std::process::Command::new("sh");
		let mut args: Vec<String> = vec![];
		args.push("-c".to_string());
		for e in command_fields {
			args.push(e.to_string());
		}
		let command = command.args(args).spawn()?;
		return Ok(command);
	}
}

/// execute command in os shell
pub fn shell_exec(command_fields: Vec<String>) -> std::result::Result<i32, Box<dyn std::error::Error>> {
	// Create command
	let mut command = create_command(&command_fields)?;
	let response = command.wait();
	if response.is_err() {
		return Err(Box::new(response.err().unwrap()));
	}

	// check exit status
	let status = response.unwrap();
	if !status.success() {
		let exit_code = status.code().unwrap();
		println!("[ERROR] command exited with status: {}", exit_code);
		return Ok(exit_code);
	}

	return Ok(0);
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
