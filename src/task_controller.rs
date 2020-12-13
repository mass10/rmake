use super::configuration;
use super::lib;

/// タスクランナー
pub struct TaskController {
	/// タスク実行記録
	tasks: Vec<configuration::Task>,
}

impl TaskController {
	/// construction
	pub fn new(tasks: Vec<configuration::Task>) -> TaskController {
		let instance = TaskController { tasks: tasks };
		return instance;
	}

	pub fn get_tasks(&self) -> &Vec<configuration::Task> {
		return &self.tasks;
	}

	/// タスクを名前で検索します。
	fn find_first_task(&self) -> Option<&configuration::Task> {
		for task in self.get_tasks() {
			return Some(task);
		}
		return None;
	}

	/// タスクを名前で検索します。
	fn find_task(&self, name: &str) -> Option<&configuration::Task> {
		// 名前の一致するタスクを探して実行します。
		for task in self.get_tasks() {
			// 名前が一致したタスクを返します。
			if task.get_name() == name {
				return Some(task);
			}
		}
		return None;
	}

	/// タスクを実行します。
	pub fn run(&self, name: &str) -> std::result::Result<bool, Box<dyn std::error::Error>> {
		// 対象のタスクを検索します。
		let result = match name {
			"" => self.find_first_task(),
			_ => self.find_task(name),
		};
		if result.is_none() {
			println!("[ERROR] タスクがみつかりませんでした。{}", name);
			return Ok(false);
		}
		let target_task = result.unwrap();

		// 依存タスクを先に実行します。
		let dependencies = target_task.get_depends_on();
		for task in dependencies {
			if !self.run(&task)? {
				println!("[ERROR] タスクの実行に失敗しています。処理はキャンセルされました。");
				return Ok(false);
			}
		}

		println!("[TRACE] タスクを実行中... [{}]", target_task.get_name());

		// ターゲットのタスクを実行します。
		let command_params = target_task.get_command();
		let exit_code = lib::shell_exec(command_params)?;
		if exit_code != 0 {
			return Ok(false);
		}

		return Ok(true);
	}
}
