use super::configuration;
use super::task_controller;

/// Application structure.
pub struct Application {
	_x: i32,
}

impl Application {
	///
	/// Return a new instance of Application.
	///
	pub fn new() -> Application {
		let instance = Application { _x: 0 };
		return instance;
	}

	/// Execute requested rmake.
	pub fn start(&self, rmakefile_path: &str, target_task_name: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
		//
		// shows banner
		//
		println!("###############################################################################");
		println!("###############################################################################");
		println!("###############################################################################");
		println!("##              ###############################    ############################");
		println!("###              ###############################   ############################");
		println!("###    ##         ##############################   ############################");
		println!("###    ###        ##############################   ############################");
		println!("###    ###        ##############################   ############################");
		println!("###    ##         #########################  ###   #     ##          ##########");
		println!("###             ###    ##       ###           ##        ##  ##        #########");
		println!("###    ##        ###             ###          ##     #####  ##        #########");
		println!("###    ###        ##   ##  #     ###  ##      ##         #        #############");
		println!("###    ###        ##   ##  #     ###  ##      ##    #    #  ######    #########");
		println!("##     ##         ##   ####      ##           ##    #    ##          ##########");
		println!("###############################################################################");
		println!("## RMAKE ver. 0.1.0                                                          ##");
		println!("###############################################################################");
		println!("");

		//
		// Configuration
		//
		let conf = configuration::Configuration::new(rmakefile_path);
		if conf.is_err() {
			println!("[ERROR] Configuration filed. reason: {}", conf.err().unwrap());
			return Ok(());
		}
		let conf = conf.ok().unwrap();

		//
		// Executin tasks
		//
		let mut controller = task_controller::TaskController::new(conf.tasks);
		controller.run(target_task_name)?;

		return Ok(());
	}
}
