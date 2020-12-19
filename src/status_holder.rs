pub struct StatusHolder {
	status: std::collections::HashMap<String, String>,
}

impl StatusHolder {
	pub fn new() -> StatusHolder {
		let instance = StatusHolder {
			status: std::collections::HashMap::new(),
		};
		return instance;
	}

	#[allow(unused)]
	pub fn get_map(&self) -> &std::collections::HashMap<String, String> {
		return &self.status;
	}
}
