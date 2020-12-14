#[derive(Debug, Clone)]
pub struct MyError {
	pub description: String,
}

impl MyError {
	#[allow(unused)]
	pub fn new(description: String) -> MyError {
		return MyError { description: description };
	}
}

impl std::fmt::Display for MyError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
		write!(f, "{}", self.description)
	}
}

impl std::error::Error for MyError {
	fn description(&self) -> &str {
		&self.description
	}
}
