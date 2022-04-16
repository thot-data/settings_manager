use fake::faker::name::raw::Name;
use super::*;


#[test]
fn system_settings_load_should_work() {
	let _settings = match Settings::load() {
		Ok( sets ) => sets,
		Err( err ) => {
			return assert!(
				false,
				"should not error: {:?}", err
			);
		}
	};
}


#[test]
fn system_settings_save_should_work() {
	let settings = Settings::new();
	if let Err( err ) = settings.save() {
		assert!(
			false,
			"should not cause error: {:?}", err
		);
	};
}