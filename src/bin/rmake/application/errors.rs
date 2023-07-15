//!
//! Application defined errors
//!

///
/// Application defined error.
///
#[derive(Debug, Clone)]
pub struct ApplicationError {
	/// Description
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
	///
	/// Implements behavior as [std::fmt::Display].
	///
	/// # Arguments
	/// * `f` Target to write
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
		write!(f, "{}", self.description)
	}
}

impl std::error::Error for ApplicationError {
	/// Description.
	///
	/// Implements behavior as [std::error::Error].
	fn description(&self) -> &str {
		&self.description
	}
}

/// Shows application version.
#[derive(Debug, Clone)]
pub struct ShowVersion;

impl std::fmt::Display for ShowVersion {
	/// Write description to formatter.
	///
	/// Implements behavior as [std::fmt::Display].
	///
	/// # Arguments
	/// * `f` Target to write
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
		write!(f, "ShowVersion")
	}
}

impl std::error::Error for ShowVersion {
	/// Description.
	///
	/// Implements behavior as [std::error::Error].
	fn description(&self) -> &str {
		return "ShowVersion";
	}
}

/// Shows application help.
#[derive(Debug, Clone)]
pub struct ShowHelp;

impl std::fmt::Display for ShowHelp {
	/// Write description to formatter.
	///
	/// Implements behavior as [std::fmt::Display].
	///
	/// # Arguments
	/// * `f` Target to write
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
		write!(f, "ShowHelp")
	}
}

impl std::error::Error for ShowHelp {
	/// Description.
	///
	/// Implements behavior as [std::error::Error].
	fn description(&self) -> &str {
		return "ShowHelp";
	}
}
