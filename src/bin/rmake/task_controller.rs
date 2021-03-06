use super::configuration;
use super::functions;
use super::status_holder;
use super::stopwatch;

/// Make command fixed if the command starts with exclamation("!").
///
/// ### Returns
/// Original command string
fn fix_command_string(command: &str) -> String {
	if command.starts_with("!") {
		return command[1..].to_string();
	}
	return command.to_string();
}

///
/// Task runner structure
///
pub struct TaskController {
	/// task definitions
	tasks: Vec<Box<configuration::Task>>,

	/// task statuses
	task_status: status_holder::StatusHolder,
}

impl TaskController {
	/// construction
	///
	/// ### Arguments
	/// * `tasks` Tasks
	///
	/// ### Returns
	/// A new instance of `TaskController`
	pub fn new(tasks: Vec<configuration::Task>) -> TaskController {
		// Duplicates task definitions
		let mut new_tasks: Vec<Box<configuration::Task>> = vec![];
		for task in tasks {
			new_tasks.push(Box::new(task));
		}

		// Creating a new instance
		let instance = TaskController {
			tasks: new_tasks,
			task_status: status_holder::StatusHolder::new(),
		};

		return instance;
	}

	/// Retrieve all task definitions
	pub fn get_tasks(&mut self) -> &mut Vec<Box<configuration::Task>> {
		return &mut self.tasks;
	}

	/// Retrieve the first task definition
	fn find_first_task(&mut self) -> Option<&mut configuration::Task> {
		for task in self.get_tasks() {
			return Some(task);
		}
		return None;
	}

	/// Find task by its name
	fn find_task(&mut self, name: &str) -> Option<&mut configuration::Task> {
		// Enumerating tasks
		for task in self.get_tasks() {
			// Returns target task
			if task.get_name() == name {
				return Some(task);
			}
		}
		return None;
	}

	/// Execute a task
	///
	/// ### Arguments
	/// * `task_name` Task to launch
	///
	/// ### Returns
	/// Task result.
	pub fn run(&mut self, task_name: &str) -> std::result::Result<bool, Box<dyn std::error::Error>> {
		// Verify task status. Does not run a task twice.
		{
			let status = self.task_status.get_status(&task_name);
			if status == "COMPLETED" {
				return Ok(true);
			}
		}

		// Find target task.
		let result = match task_name {
			"" => self.find_first_task(),
			_ => self.find_task(task_name),
		};
		if result.is_none() {
			println!("{} rmake [ERROR] Task not found. [{}]", functions::get_timestamp(), task_name);
			return Ok(false);
		}
		let target_task = result.unwrap().clone();

		// Execute dependencies first
		{
			for task in &target_task.get_depends_on() {
				if !self.run(&task)? {
					println!("{} rmake [ERROR] Task failed. Operation canceled.", functions::get_timestamp());
					return Ok(false);
				}
			}
		}

		// Execute target task
		{
			println!();
			println!("==============================================================================");
			println!("name        : {}", target_task.get_name());
			println!("description : {}", target_task.get_description());
			println!("==============================================================================");
			println!("{} rmake [INFO] executing task [{}] ...", functions::get_timestamp(), target_task.get_name());

			// Stopwatch
			let watch = stopwatch::Stopwatch::new();

			for command in target_task.get_command() {
				let is_safe_command = command.starts_with("!");
				let command = fix_command_string(&command);
				println!("{} rmake [INFO] executing command [{}]", functions::get_timestamp(), command);
				let exit_code = functions::shell_exec(&command)?;
				if exit_code != 0 {
					if is_safe_command {
						println!("{} rmake [WARN] command exited with status: [{}] ({})", functions::get_timestamp(), exit_code, watch);
					} else {
						println!("{} rmake [ERROR] command exited with status: [{}] ({})", functions::get_timestamp(), exit_code, watch);
						return Ok(false);
					}
				}
			}
			println!();
			println!("{} rmake [INFO] task [{}] terminated. ({})", functions::get_timestamp(), target_task.get_name(), watch);
		}

		// Mark task completed
		{
			self.task_status.set_status(&target_task.get_name(), "COMPLETED");
		}

		return Ok(true);
	}
}
