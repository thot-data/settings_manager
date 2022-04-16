use std::mem::{self, Discriminant};
use std::cmp::{PartialEq, PartialOrd};
use std::io;
use std::fs::File;
use serde::{Serialize, Deserialize};
use serde_json;
use cluFlock::{ExclusiveFlock, FlockLock};


// ***************
// *** Results ***
// ***************

pub type SettingsResult<T> = Result<T, SettingsError>;

#[derive(Debug)]
pub enum SettingsError {
    IoError( io::Error ),
    JsonError( serde_json::Error ),
}


// ****************
// *** Priority ***
// ****************


/**
 *  Priority level of the settings.
 *  Settings defined in lower priority settings are overwritten by higher ones.
 */
#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Debug)]
pub enum SettingsPriority {
    System = 0,
    User = 1,
    Project = 2,
}


impl SettingsPriority {
    pub fn priority( &self ) -> Discriminant<SettingsPriority> {
        mem::discriminant( &self )
    }
}


// ************
// *** Misc ***
// ************


/**
 * Obtain a file lock on the system settings file
 * to prevent other programs from accessing it.
 */
pub fn lock( file: &File ) -> SettingsResult<FlockLock<&File>> {
    match ExclusiveFlock::wait_lock( file ) {
        Ok( lock ) => Ok( lock ),
        Err( flock_err ) => return Err( SettingsError::IoError( flock_err.into_err() ) ),
    }
}



#[cfg(test)]
#[path = "./common_test.rs"]
mod common_test;