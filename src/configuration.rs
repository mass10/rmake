extern crate serde_derive;

#[derive(serde_derive::Deserialize, Debug, std::clone::Clone)]
pub struct Attribute {
	attribute01: Option<String>,
	attribute02: Option<String>,
}

#[derive(serde_derive::Deserialize, Debug, std::clone::Clone)]
pub struct Step {
	/// 名前
	pub name: Option<String>,
	/// 依存するタスク
	pub depends_on: Option<Vec<String>>,
	/// コマンドライン
	pub command: Option<Vec<String>>,
	/// 説明文
	pub description: Option<String>,
}

impl Step {
	/// contruct
	pub fn new() -> Step {
		let instance = Step {
			name: Some(String::new()),
			depends_on: Some(vec![]),
			command: Some(vec![]),
			description: Some(String::new()),
		};
		return instance;
	}

	/// タスクの実行
	pub fn run(&self) -> std::result::Result<i32, Box<dyn std::error::Error>> {
		// ステップの名前
		let name = self.get_name();
		let command_params = self.get_command();
		let depends_on = self.get_depends_on();

		for step_id in depends_on {
			let step = Step::new();
		}

		println!("[TRACE] executing step [{}] ... [{:?}]", name, command_params);

		// プロセスを実行します。
		let mut command = std::process::Command::new("cmd");
		// let args: &[&str] = &["/C"];
		let mut args: Vec<String> = vec![];
		// args.push(command_params);
		// args.fill(command_params);
		args.push("/C".to_string());
		for e in command_params {
			args.push(e)
		}

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
			return Ok(exit_code);
			// std::process::exit(exit_code);
		}

		return Ok(0);
	}

	pub fn get_name(&self) -> String {
		if self.name.is_none() {
			return String::new();
		}
		let name = self.name.as_ref().unwrap();
		return name.as_str().to_string();
	}

	pub fn get_command(&self) -> Vec<String> {
		if self.command.is_none() {
			let result: Vec<String> = vec![];
			return result.clone();
		}
		let command = self.command.as_ref().unwrap();
		return command.clone();
	}

	pub fn get_depends_on(&self) -> Vec<String> {
		if self.command.is_none() {
			let result: Vec<String> = vec![];
			return result.clone();
		}
		let depends_on = self.depends_on.as_ref().unwrap();
		return depends_on.clone();
	}
}

#[derive(serde_derive::Deserialize, Debug)]
pub struct Job {
	pub steps: std::vec::Vec<Step>,
}

#[derive(serde_derive::Deserialize, Debug)]
pub struct Header {
	pub email: Option<String>,
	pub threshold: Option<u32>,
	pub attributes: Option<Attribute>,
}

#[derive(serde_derive::Deserialize, Debug)]
pub struct Configuration {
	pub settings: Header,
	pub steps: std::vec::Vec<Step>,
}

impl Configuration {
	/// 新しいインスタンスを返します。
	pub fn new(path: &str) -> std::result::Result<Configuration, Box<dyn std::error::Error>> {
		extern crate toml;

		// ファイル全体を文字列として読み込みます。
		println!("[TRACE] Reading rmake file ... [{}]", path);

		let content = super::lib::read_text_file_all(path)?;

		// toml 文字列を解析します。
		let conf: Configuration = toml::from_str(&content)?;

		return Ok(conf);
	}
	// pub fn new(path: String) -> std::boxed::Box<Configuration> {
	// 	let instance = Configuration {};
	// 	instance.configure(path);
	// 	return std::boxed::Box::new(instance);
	// }

	// fn configure(&self, path: String) {}
}
