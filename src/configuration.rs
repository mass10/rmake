extern crate serde_derive;

use super::lib;

/// タスク定義
#[derive(serde_derive::Deserialize, Debug, std::clone::Clone)]
pub struct Task {
	/// 名前
	name: Option<String>,
	/// 説明文
	description: Option<String>,
	/// 依存するタスク
	depends_on: Option<Vec<String>>,
	/// コマンドライン
	command: Option<Vec<String>>,
}

impl Task {
	/// 名前を返します。
	pub fn get_name(&self) -> String {
		if self.name.is_none() {
			return String::new();
		}
		let name = self.name.as_ref().unwrap();
		return name.as_str().to_string();
	}

	/// コマンドを返します。
	pub fn get_command(&self) -> Vec<String> {
		if self.command.is_none() {
			let result: Vec<String> = vec![];
			return result.clone();
		}
		let command = self.command.as_ref().unwrap();
		return command.clone();
	}

	/// 依存タスクを返します。
	pub fn get_depends_on(&self) -> Vec<String> {
		if self.command.is_none() {
			let result: Vec<String> = vec![];
			return result.clone();
		}
		let depends_on = self.depends_on.as_ref().unwrap();
		return depends_on.clone();
	}
}

/// コンフィギュレーション
#[derive(serde_derive::Deserialize, Debug, std::clone::Clone)]
pub struct Configuration {
	/// 変数定義
	pub env: Option<std::collections::btree_map::BTreeMap<String, String>>,
	/// タスク定義
	pub tasks: Vec<Task>,
}

impl Configuration {
	/// 新しいインスタンスを返します。
	pub fn new(rmakefile_path: &str) -> std::result::Result<Configuration, Box<dyn std::error::Error>> {
		extern crate toml;

		let rmakefile_path = lib::select(rmakefile_path, "rmake.toml");

		// ファイル全体を文字列として読み込みます。
		// println!("[TRACE] Reading rmake file ... [{}]", &rmakefile_path);

		let content = lib::read_text_file_all(&rmakefile_path)?;

		// toml 文字列を解析します。
		let conf: Configuration = toml::from_str(&content)?;

		if conf.env.is_some() {
			let env = conf.env.as_ref().unwrap();
			println!("[TRACE] {:?}", &env.keys());
		}

		return Ok(conf);
	}
}
