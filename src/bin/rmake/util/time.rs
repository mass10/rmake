//!
//! Time utilities
//!

/// Custom formatter for std::time::Duration
trait MyDurationFormatter {
	/// Format elapsed time.
	///
	/// # Returns
	/// * Formatted string
	fn to_string(&self) -> String;
}

impl MyDurationFormatter for std::time::Duration {
	fn to_string(&self) -> String {
		let mut millis = self.as_millis();
		let mut sec = 0;
		let mut min = 0;
		let mut hour = 0;
		while 1000 <= millis {
			sec += 1;
			millis -= 1000;
		}
		while 60 <= sec {
			min += 1;
			sec -= 60;
		}
		while 60 <= min {
			hour += 1;
			min -= 60;
		}
		let s = format!("{:02}:{:02}:{:02}:{:03}", hour, min, sec, millis);
		return s;
	}
}

/// Stopwatch
pub struct Stopwatch {
	/// Timestamp
	time: std::time::Instant,
}

impl Stopwatch {
	/// Return a new instance.
	///
	/// # Returns
	/// A new instance of `Stopwatch`
	pub fn new() -> Stopwatch {
		return Stopwatch { time: std::time::Instant::now() };
	}
}

impl std::fmt::Display for Stopwatch {
	/// Format elapsed time.
	///
	/// # Arguments
	/// * `f` Target to write
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let elapsed = std::time::Instant::now() - self.time;
		write!(f, "{}", elapsed.to_string())?;
		return Ok(());
	}
}
