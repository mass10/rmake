use super::configuration;
use super::lib;
use super::status_holder;

/// タスクランナー
pub struct TaskController {
	/// タスク実行記録
	tasks: Vec<Box<configuration::Task>>,

	/// タスク終了ステータス
	#[allow(unused)]
	task_status: status_holder::StatusHolder,
}

impl TaskController {
	/// construction
	pub fn new(tasks: Vec<configuration::Task>) -> TaskController {
		// タスクリストを複製して
		let mut new_tasks: Vec<Box<configuration::Task>> = vec![];
		for task in tasks {
			new_tasks.push(Box::new(task));
		}

		// インスタンスを初期化
		let instance = TaskController {
			tasks: new_tasks,
			task_status: status_holder::StatusHolder::new(),
		};

		return instance;
	}

	pub fn get_tasks(&mut self) -> &mut Vec<Box<configuration::Task>> {
		return &mut self.tasks;
	}

	/// タスクを名前で検索します。
	fn find_first_task(&mut self) -> Option<&mut configuration::Task> {
		for task in self.get_tasks().into_iter() {
			return Some(task);
		}
		return None;
	}

	/// タスクを名前で検索します。
	fn find_task(&mut self, name: &str) -> Option<&mut configuration::Task> {
		// 名前の一致するタスクを探して実行します。
		for task in self.get_tasks().iter_mut() {
			// 名前が一致したタスクを返します。
			if task.get_name() == name {
				return Some(task);
			}
		}
		return None;
	}

	/// タスクを実行します。
	pub fn run(&mut self, name: &str) -> std::result::Result<bool, Box<dyn std::error::Error>> {
		// Find target task.
		let result = match name {
			"" => self.find_first_task(),
			_ => self.find_task(name),
		};
		if result.is_none() {
			println!("[ERROR] Task not found. [{}]", name);
			return Ok(false);
		}
		let target_task = result.unwrap().clone();

		// Verify task status.
		{
			let status = self.task_status.get_status(target_task.get_name());
			if status == "COMPLETED" {
				return Ok(true);
			}
		}

		// Execute dependencies first.
		{
			for task in target_task.get_depends_on() {
				if !self.run(&task)? {
					println!("[ERROR] タスクの実行に失敗しています。処理はキャンセルされました。");
					return Ok(false);
				}
			}
		}

		// Execute target task.
		{
			println!("");
			println!("[TRACE] executing task... [{}]", target_task.get_name());
			let command_params = target_task.get_command();
			let exit_code = lib::shell_exec(command_params)?;
			if exit_code != 0 {
				return Ok(false);
			}
		}

		// Mark completed.
		{
			self.task_status.set_status(target_task.get_name(), String::from("COMPLETED"));
		}

		return Ok(true);
	}
}
