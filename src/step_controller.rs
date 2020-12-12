use super::configuration;

use super::lib;

pub struct StepController {
	/// ステップ実行記録
	steps: std::vec::Vec<configuration::Step>,
}

impl StepController {
	/// construction
	pub fn new(steps: std::vec::Vec<configuration::Step>) -> StepController {
		let instance = StepController { steps: steps };
		return instance;
	}

	pub fn get_steps(&self) -> std::vec::Vec<configuration::Step> {
		return self.steps.clone();
	}

	fn find_step(&self, name: &str) -> Option<configuration::Step> {
		// 名前の一致するステップを探して実行します。
		for step in self.get_steps() {
			if name == "" {
				return Some(step);
			}
			if step.get_name() != name {
				continue;
			}
			return Some(step);
		}
		return None;
	}

	/// ステップを実行します。
	pub fn run(&self, name: &str) -> std::result::Result<bool, Box<dyn std::error::Error>> {
		// 対象のステップを検索します。
		let target_step = self.find_step(name);
		if target_step.is_none() {
			println!("[ERROR] ステップがみつかりませんでした。{}", name);
			return Ok(false);
		}
		let target_step = target_step.unwrap();

		// 依存タスクを先に実行します。
		let dependencies = target_step.get_depends_on();
		for step in dependencies {
			if !self.run(&step)? {
				println!("[ERROR] ステップの実行に失敗しています。処理はキャンセルされました。");
				return Ok(false);
			}
		}

		// ターゲットのタスクを実行します。
		println!("[TRACE] ステップを実行中... [{}]", target_step.get_name());
		let command_params = target_step.get_command();
		let exit_code = lib::shell_exec(command_params)?;
		if exit_code != 0 {
			println!("[WARN] yarn exited with status: {}", exit_code);
			return Ok(false);
		}

		return Ok(true);
	}
}
