//!
//! Application core implementation
//!

pub mod errors;
mod task;

use crate::application;
use crate::configuration;
use crate::util;

///
/// Application structure
///
pub struct Application {
	/// Dummy
	_x: i32,
}

impl Application {
	/// Returns a new instance of `Application`.
	///
	/// # Returns
	/// A new instance of `Application`
	pub fn new() -> Application {
		let instance = Application { _x: 0 };
		return instance;
	}

	/// Shows banner
	fn show_banner() {
		let application_info = format!("{} {}, {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_DESCRIPTION"));

		println!("################################################################################");
		println!("################################################################################");
		println!("################################################################################");
		println!("##               #################################    ##########################");
		println!("###               #################################   ##########################");
		println!("###    ##          ################################   ##########################");
		println!("###    ###         ################################   ##########################");
		println!("###    ###         ################################   ##########################");
		println!("###    ##          ###########################  ###   #        ###           ###");
		println!("###              ###     #        ###            ##            ##  ##         ##");
		println!("###    ##         ###              ###           ##          ####  ##         ##");
		println!("###    ###         ##    #  #      ###  ###      ##            ##         ######");
		println!("###    ###         ##    #  #      ###  ###      ##   ##       ##  #######    ##");
		println!("##     ##          ##    ###       ##            ##   ##       ###           ###");
		println!("################################################################################");
		println!("## {:74} ##", application_info);
		println!("################################################################################");
		println!("");
	}

	/// Executes rmake.toml given.
	///
	/// # Arguments
	/// * `rmakefile_path` path to rmake file
	/// * `target_task_name` Task to launch
	pub fn start(&self, rmakefile_path: &str, target_task_name: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
		// Stopwatch
		let watch = util::time::Stopwatch::new();

		// shows banner
		Application::show_banner();

		println!("{} rmake [INFO] START", util::functions::get_timestamp());

		// configuration
		let conf = configuration::ConfigurationSettings::new(rmakefile_path);
		if conf.is_err() {
			println!("{} rmake [ERROR] Configuration failed. reason: [{}]", util::functions::get_timestamp(), conf.err().unwrap());
			return Ok(());
		}
		let conf = conf.ok().unwrap();

		// execute tasks
		let mut controller = application::task::TaskController::new(conf.tasks);
		controller.run(target_task_name)?;

		println!();
		println!("{} rmake [INFO] END ({})", util::functions::get_timestamp(), watch);

		return Ok(());
	}
}
