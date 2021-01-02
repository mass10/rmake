use super::configuration;
use super::functions;
use super::status_holder;

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
	pub fn run(&mut self, name: &str) -> std::result::Result<bool, Box<dyn std::error::Error>> {
		// Find target task.
		let result = match name {
			"" => self.find_first_task(),
			_ => self.find_task(name),
		};
		if result.is_none() {
			println!("{} [ERROR] Task not found. [{}]", functions::get_timestamp(), name);
			return Ok(false);
		}
		let target_task = result.unwrap().clone();

		// Verify task status
		{
			let status = self.task_status.get_status(&target_task.get_name());
			if status == "COMPLETED" {
				return Ok(true);
			}
		}

		// Execute dependencies first
		{
			for task in target_task.get_depends_on() {
				if !self.run(&task)? {
					println!("{} [ERROR] Task failed. Operation canceled.", functions::get_timestamp());
					return Ok(false);
				}
			}
		}

		// Execute target task
		{
			println!("{} [TRACE] executing task...", functions::get_timestamp());
			println!("==============================================================================");
			println!("name        : {}", target_task.get_name());
			println!("description : {}", target_task.get_description());
			println!("==============================================================================");

			for commands in target_task.get_command() {
				let exit_code = functions::shell_exec(&commands)?;
				if exit_code != 0 {
					return Ok(false);
				}
			}
			println!("");
		}

		// Mark task completed
		{
			self.task_status.set_status(&target_task.get_name(), "COMPLETED");
		}

		return Ok(true);
	}
}
