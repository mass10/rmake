use super::configuration;
use super::task_controller;

/// アプリケーション本体の定義
pub struct Application {}

impl Application {
	/// 指定された rmake ファイルを実行します。
	pub fn start(&self, rmakefile_path: &str, target_task_name: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
		// コンフィギュレーション
		let conf = configuration::Configuration::new(rmakefile_path);
		if conf.is_err() {
			println!("[ERROR] {}", conf.err().unwrap());
			return Ok(());
		}
		let conf = conf.ok().unwrap();

		// タスクを実行
		let controller = task_controller::TaskController::new(conf.tasks);
		controller.run(target_task_name)?;

		return Ok(());
	}
}
