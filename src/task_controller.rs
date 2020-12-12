use super::configuration;

use super::lib;

/// タスクランナー
pub struct TaskController {
	/// タスク実行記録
	steps: std::vec::Vec<configuration::Step>,
}

impl TaskController {
	/// construction
	pub fn new(steps: std::vec::Vec<configuration::Step>) -> TaskController {
		let instance = TaskController { steps: steps };
		return instance;
	}

	pub fn get_steps(&self) -> std::vec::Vec<configuration::Step> {
		return self.steps.clone();
	}

	/// タスクを名前で検索します。
	fn find_step(&self, name: &str) -> Option<configuration::Step> {
		// 名前の一致するタスクを探して実行します。
		for step in self.get_steps() {
			// 名前が指定されなかったときはデフォルト(=先頭)のタスクを返します。
			if name == "" {
				return Some(step);
			}
			// 名前が一致したタスクを返します。
			if step.get_name() == name {
				return Some(step);
			}
		}
		return None;
	}

	/// タスクを実行します。
	pub fn run(&self, name: &str) -> std::result::Result<bool, Box<dyn std::error::Error>> {
		// 対象のタスクを検索します。
		let result = self.find_step(name);
		if result.is_none() {
			println!("[ERROR] タスクがみつかりませんでした。{}", name);
			return Ok(false);
		}
		let target_step = result.unwrap();

		// 依存タスクを先に実行します。
		let dependencies = target_step.get_depends_on();
		for step in dependencies {
			if !self.run(&step)? {
				println!("[ERROR] タスクの実行に失敗しています。処理はキャンセルされました。");
				return Ok(false);
			}
		}

		println!("[TRACE] タスクを実行中... [{}]", target_step.get_name());

		// ターゲットのタスクを実行します。
		let command_params = target_step.get_command();
		let exit_code = lib::shell_exec(command_params)?;
		if exit_code != 0 {
			return Ok(false);
		}

		return Ok(true);
	}
}