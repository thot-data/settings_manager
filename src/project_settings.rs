use std::path::Path;
use crate::common::SettingsResult;
use crate::settings::Settings;


pub trait ProjectSettings: Settings {
	fn new( path: &Path ) -> SettingsResult<Self> {
		Self::load( path )
	}
}



#[cfg(test)]
#[path = "./project_settings_test.rs"]
mod project_settings_test;