use std::path::Path;
use crate::common::SettingsResult;
use crate::settings::Settings;


pub trait SystemSettings: Settings {

	/**
	 * Create a new settings struct for the user.
	 */
	fn new(
		qualifier: &str,
		organization: &str,
		application: &str,
		file_name: &Path,
	) -> SettingsResult<Self>;
}



#[cfg(test)]
#[path = "./system_settings_test.rs"]
mod system_settings_test;