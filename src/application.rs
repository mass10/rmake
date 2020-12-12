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

		println!("[TRACE] {:?}", conf.settings);

		let step_controller = step_controller::StepController::new(conf.steps);
		step_controller.run("")?;

		return Ok(());
	}
}
