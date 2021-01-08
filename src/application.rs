use super::configuration;
use super::functions;
use super::stopwatch;
use super::task_controller;

///
/// Application structure
///
pub struct Application {
	_x: i32,
}

impl Application {
	/// Returns a new instance of Application.
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
	pub fn start(&self, rmakefile_path: &str, target_task_name: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
		// Stopwatch
		let watch = stopwatch::Stopwatch::new();

		// shows banner
		Application::show_banner();

		println!("{} rmake [INFO] START", functions::get_timestamp());

		// configuration
		let conf = configuration::ConfigurationSettings::new(rmakefile_path);
		if conf.is_err() {
			println!("{} rmake [ERROR] Configuration failed. reason: [{}]", functions::get_timestamp(), conf.err().unwrap());
			return Ok(());
		}
		let conf = conf.ok().unwrap();

		// execute tasks
		let mut controller = task_controller::TaskController::new(conf.tasks);
		controller.run(target_task_name)?;

		println!();
		println!("{} rmake [INFO] END ({})", functions::get_timestamp(), watch);

		return Ok(());
	}
}
