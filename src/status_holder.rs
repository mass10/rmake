///
/// Status holder
///
pub struct StatusHolder {
	status: std::collections::HashMap<String, String>,
}

impl StatusHolder {
	/// Create a new instance.
	pub fn new() -> StatusHolder {
		let instance = StatusHolder {
			status: std::collections::HashMap::new(),
		};
		return instance;
	}

	/// Return mutable reference to the internal collection.
	#[allow(unused)]
	pub fn get_map(&mut self) -> &mut std::collections::HashMap<String, String> {
		return &mut self.status;
	}

	/// Set task status.
	pub fn set_status(&mut self, name: &str, status: &str) {
		self.status.insert(name.to_string(), status.to_string());
	}

	/// Return task status.
	pub fn get_status(&mut self, name: &str) -> String {
		let status = self.status.get(&name.to_string());
		if status.is_none() {
			return String::new();
		}
		return status.unwrap().clone();
	}
}
