use super::configuration;
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
		println!("## RMAKE ver. 0.1.0                                                           ##");
		println!("################################################################################");
		println!("");
	}

	/// Executes rmake.toml given.
	pub fn start(&self, rmakefile_path: &str, target_task_name: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
		// shows banner
		Application::show_banner();

		// configuration
		let conf = configuration::Configuration::new(rmakefile_path);
		if conf.is_err() {
			println!("[ERROR] Configuration failed. reason: {}", conf.err().unwrap());
			return Ok(());
		}
		let conf = conf.ok().unwrap();

		// execute tasks
		let mut controller = task_controller::TaskController::new(conf.tasks);
		controller.run(target_task_name)?;

		return Ok(());
	}
}
