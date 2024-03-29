//!
//! Helpers
//!
///
/// Status holder
///
pub struct StatusHolder {
	/// Statuses.
	status: std::collections::HashMap<String, String>,
}

impl StatusHolder {
	/// Create a new instance.
	///
	/// # Returns
	/// A new instance of `StatusHolder`.
	pub fn new() -> StatusHolder {
		let instance = StatusHolder {
			status: std::collections::HashMap::new(),
		};
		return instance;
	}

	/// Return mutable reference to the internal collection.
	///
	/// # Returns
	/// Mutable reference to the internal collection.
	#[allow(unused)]
	pub fn get_map(&mut self) -> &mut std::collections::HashMap<String, String> {
		return &mut self.status;
	}

	/// Set task status.
	///
	/// # Arguments
	/// * `name` Task name
	/// * `status` Task status
	pub fn set_status(&mut self, name: &str, status: &str) {
		self.status.insert(name.to_string(), status.to_string());
	}

	/// Return task status.
	///
	/// # Arguments
	/// * `name` Task name
	///
	/// # Returns
	/// The status of task named `name`
	pub fn get_status(&mut self, name: &str) -> String {
		let status = self.status.get(&name.to_string());
		if status.is_none() {
			return String::new();
		}
		return status.unwrap().clone();
	}
}
