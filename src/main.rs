extern crate serde_derive;

mod util {
	pub fn read_text_file_all(path: &str) -> std::result::Result<String, Box<dyn std::error::Error>> {
		use std::io::Read;

		let mut file = std::fs::File::open(path).unwrap();
		let mut s = String::new();
		file.read_to_string(&mut s)?;
		return Ok(s);
	}
}

mod configuration {

	#[derive(serde_derive::Deserialize, Debug)]
	pub struct Attribute {
		attribute01: Option<String>,
		attribute02: Option<String>,
	}

	#[derive(serde_derive::Deserialize, Debug)]
	pub struct Step {
		pub name: Option<String>,
		pub depends_on: Option<String>,
	}

	#[derive(serde_derive::Deserialize, Debug)]
	pub struct Job {
		pub steps: std::vec::Vec<Step>,
	}

	#[derive(serde_derive::Deserialize, Debug)]
	pub struct Settings {
		pub email: Option<String>,
		pub threshold: Option<u32>,
		pub attributes: Option<Attribute>,
	}

	#[derive(serde_derive::Deserialize, Debug)]
	pub struct Configuration {
		pub settings: Settings,
		pub steps: std::vec::Vec<Step>,
	}

	impl Configuration {
		pub fn new(path: &str) -> std::result::Result<Configuration, Box<dyn std::error::Error>> {
			extern crate toml;
			// ファイル全体を文字列として読み込みます。
			let content = super::util::read_text_file_all(path)?;
			// toml 文字列を解析します。
			let conf: Configuration = toml::from_str(&content)?;
			return Ok(conf);
		}
		// pub fn new(path: String) -> std::boxed::Box<Configuration> {
		// 	let instance = Configuration {};
		// 	instance.configure(path);
		// 	return std::boxed::Box::new(instance);
		// }

		// fn configure(&self, path: String) {}
	}
}

mod application_core {
	pub struct Application {}
}

fn usage() {
	println!("USAGE:");
}

fn main() {
	let args: std::vec::Vec<String> = std::env::args().skip(1).collect();
	if args.len() == 0 {
		usage();
		return;
	}

	let path = &args[0];
	let _app = application_core::Application {};

	let conf = configuration::Configuration::new(path);

	if conf.is_err() {
		println!("[ERROR] configuration error. {}", conf.err().unwrap());
		return;
	}

	let conf = conf.ok().unwrap();

	// [settings]
	println!("[TRACE] {:?}", conf.settings);
	// [steps]
	for step in conf.steps {
		println!("[TRACE] {:?}", step);
	}
}
