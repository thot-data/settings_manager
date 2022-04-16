use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, BufReader};
use std::path::Path;
use tempfile::NamedTempFile;
use serde::Serialize;
use serde::de::DeserializeOwned;
use crate::common::{self, SettingsResult, SettingsError};


/**
 * 
 */
pub trait Settings: Serialize + DeserializeOwned {

	/**
	 * Create a new object.
	 */
	fn new() -> Self;


	/**
	 * Ensures that the system settings directory is created and
	 * returns the `File` to the system settings.
	 */
	fn file_ensured( path: &Path ) -> SettingsResult<File> {

		// ensure settings directory exists
		let settings_dir = match path.parent() {
			Some( path ) => path,
			None => return Err( SettingsError::IoError(
				io::Error::new(
					io::ErrorKind::NotFound,
					"invalid path"
				)
			) )
		};

		match fs::create_dir_all( settings_dir ){
			Ok( () ) => {},  // ok, continue
			Err( ref err ) if err.kind() == io::ErrorKind::AlreadyExists => {},  // directories already exist, continue
			Err( err ) => return Err( SettingsError::IoError( err ) ),
		}

		// create file if needed
		let file_res = OpenOptions::new()
			.read( true )
			.write( true )
			.create( true )
			.open( path );

		match file_res {
			Ok( file ) => Ok( file ),
			Err( err ) => return Err( SettingsError::IoError( err ) ),
		}
	}


	fn load( path: &Path ) -> SettingsResult<Self> {
		// get settings file and lock
		let settings_file = Self::file_ensured( path )?;
		let _file_lock = common::lock( &settings_file )?;

		// get current settings
		let mut reader = BufReader::new( &settings_file );
		let mut settings_json = String::new(); 
		if let Err( err ) = reader.read_to_string( &mut settings_json ) {
			return Err( SettingsError::IoError( err ) )
		};

		if settings_json.is_empty() {
			// no content in file, create default object
			return Ok( Self::new() )
		}

		return match serde_json::from_str( &settings_json ) {
			Ok( sets ) => Ok( sets ),
			Err( err ) => Err( SettingsError::JsonError( err ) ),
		}
	}


	fn save( &self, path: &Path ) -> SettingsResult<()> {
		// get settings file and lock
		let settings_file = Self::file_ensured( path )?;
		let _file_lock = common::lock( &settings_file )?;

		// write settings to temp file
		let settings_file_tmp = match NamedTempFile::new() {
			Ok( file ) => file,
			Err( err ) => return Err( SettingsError::IoError( err ) ),
		};

		let write_result = serde_json::to_writer_pretty( &settings_file_tmp, &self );
		if let Err( err ) = write_result {
			return Err( SettingsError::JsonError( err ) )
		};

		let mv_result = fs::rename( settings_file_tmp.path(), path );
		if let Err( err ) = mv_result {
			return Err( SettingsError::IoError( err ) )
		}

		// success
		Ok( () )
	}


	// /**
	//  * Checks if the settings in the struct are
	//  * equivalent to those saved on disk.
	//  */
	// pub fn stale() -> bool {
	// }

}



// #[cfg(test)]
// #[path = "./settings_test.rs"]
// mod settings_test;