#[derive(Debug, Clone)]
pub struct ApplicationError {
	pub description: String,
}

impl ApplicationError {
	#[allow(unused)]
	pub fn new(description: String) -> ApplicationError {
		return ApplicationError { description: description };
	}
}

impl std::fmt::Display for ApplicationError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
		write!(f, "{}", self.description)
	}
}

impl std::error::Error for ApplicationError {
	fn description(&self) -> &str {
		&self.description
	}
}
