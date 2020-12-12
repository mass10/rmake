use super::configuration;

pub struct StepController {
	/// ステップ実行記録
	steps: std::vec::Vec<configuration::Step>,
}

impl StepController {
	/// construction
	pub fn new(steps: std::vec::Vec<configuration::Step>) -> StepController {
		let instance = StepController { steps: steps };
		return instance;
	}

	/// コンフィギュレーション
	pub fn configure(&self) -> std::result::Result<(), Box<dyn std::error::Error>> {
		return Ok(());
	}

	pub fn get_steps(&self) -> std::vec::Vec<configuration::Step> {
		return self.steps.clone();
	}

	/// ステップを実行します。
	pub fn run(&self, name: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
		// 名前の一致するステップを探して実行します。
		for step in self.get_steps() {
			if step.get_name() != name {
				continue;
			}
			if 0 != step.run()? {
				break;
			}
		}
		return Ok(());
	}
}
