///
/// Application defined error.
///
#[derive(Debug, Clone)]
pub struct ApplicationError {
	/// Error description
	pub description: String,
}

impl ApplicationError {
	/// Create a new instance.
	pub fn new(description: &str) -> ApplicationError {
		return ApplicationError { description: description.to_string() };
	}
}

impl std::fmt::Display for ApplicationError {
	/// Write description to formatter.
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
		write!(f, "{}", self.description)
	}
}

impl std::error::Error for ApplicationError {
	/// Description.
	fn description(&self) -> &str {
		&self.description
	}
}
