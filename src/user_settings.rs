use std::io;
use std::path::{PathBuf, Path};
use directories::ProjectDirs;
use crate::common::{SettingsResult, SettingsError};
use crate::settings::Settings;


pub trait UserSettings: Settings {

	/**
	 * Create a new settings struct for the user.
	 */
	fn new(
		qualifier: &str,
		organization: &str,
		application: &str,
		file_name: &Path,
	) -> SettingsResult<Self> {
		// get user settigns path
		let dirs_opt = ProjectDirs::from(
			qualifier,
			organization,
			application,
		);

		let dirs = match dirs_opt {
			Some( d ) => d,
			None => return Err( SettingsError::IoError(
				io::Error::new(
					io::ErrorKind::NotFound,
				"user settings directory not found"
				)
			) )
		};

		let mut path = PathBuf::from( dirs.config_dir() );
		path.push( file_name );
		
		// load settings
		Self::load( &path )
	}

}



#[cfg(test)]
#[path = "./user_settings_test.rs"]
mod user_settings_test;