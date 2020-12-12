use super::configuration;
use super::step_controller;
/// ステップコントローラー

/// アプリケーション本体の定義
pub struct Application {}

impl Application {
	/// 指定された rmake ファイルを実行します。
	pub fn start(&self, path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
		// コンフィギュレーション
		let conf = configuration::Configuration::new(path);
		if conf.is_err() {
			println!("[ERROR] configuration error. {}", conf.err().unwrap());
			return Ok(());
		}

		let conf = conf.ok().unwrap();

		// [settings]
		println!("[TRACE] {:?}", conf.settings);

		// [steps]
		let step_controller = step_controller::StepController::new(conf.steps);
		for step in step_controller.get_steps() {
			// println!("[TRACE] {:?}", step);
			step.run()?;
		}

		return Ok(());
	}
}
