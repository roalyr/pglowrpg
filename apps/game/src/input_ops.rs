pub fn get_commands() -> Vec<String> {
	//For predictive input, can be moved somewhere else later
	//All commands must be registered here in ordet to be able to
	//match to them
	[
		//Movement directions
		"north".to_string(),
		"east".to_string(),
		"south".to_string(),
		"west".to_string(),
		//teleport
		"x".to_string(),
		"y".to_string(),
		//Common actions
		"?".to_string(),
		"q".to_string(),
		"render surrounding".to_string(),
	]
	.to_vec()
}
