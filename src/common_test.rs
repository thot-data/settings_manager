use super::*;


#[test]
fn level_priority_test() {
	let s = SettingsPriority::System;
	let u = SettingsPriority::User;
	let p = SettingsPriority::Project;

	assert!(
		s < u,
		"system priority should be less than user"
	);

	assert!(
		u < p,
		"user priority should be less than project"
	);

}
