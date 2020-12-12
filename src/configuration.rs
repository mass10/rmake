extern crate serde_derive;

use super::lib;

#[derive(serde_derive::Deserialize, Debug, std::clone::Clone)]
pub struct Attribute {
	attribute01: Option<String>,
	attribute02: Option<String>,
}

#[derive(serde_derive::Deserialize, Debug, std::clone::Clone)]
pub struct Step {
	/// 名前
	name: Option<String>,
	/// 説明文
	description: Option<String>,
	/// 依存するタスク
	depends_on: Option<Vec<String>>,
	/// コマンドライン
	command: Option<Vec<String>>,
}

impl Step {
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
/// ヘッダー情報
#[derive(serde_derive::Deserialize, Debug, std::clone::Clone)]
pub struct Header {
	pub email: Option<String>,
	pub threshold: Option<u32>,
	pub attributes: Option<Attribute>,
}

/// コンフィギュレーション
#[derive(serde_derive::Deserialize, Debug, std::clone::Clone)]
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

		let content = lib::read_text_file_all(path)?;

		// toml 文字列を解析します。
		let conf: Configuration = toml::from_str(&content)?;

		return Ok(conf);
	}
}
