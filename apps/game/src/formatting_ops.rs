use crate::struct_ops::{GameData, GameStrings};

pub fn get_strings_basic(
	gd: &GameData,
	gs: &mut GameStrings,
) {
	//Format strings
	//Into formatting ops
	gs.coord_str = [
		//&gs.gm_str.gm6,
		"x:",
		&(gd.x.to_string()),
		" y:",
		&(gd.y.to_string()),
		" index:",
		&(gd.index.to_string()),
		" ",
	]
	.concat();

	gs.temp_str = [
		"Temperature: ",
		//&gs.gm_str.gm8,
		&(gd.temp.to_string()),
		" ℃",
	]
	.concat();

	gs.biome_str = [
		//&gs.gm_str.gm16,
		"Biome: ",
		&(gd.biome.to_string()),
		"",
	]
	.concat();

	gs.georeg_id_str = [
		//&gs.gm_str.gm17,
		"Region: ",
		&(gd.georeg_id.to_string()),
		"",
	]
	.concat();

	gs.rain_str = [
		//&gs.gm_str.gm9,
		"Rainfall: ",
		&(gd.rain.to_string()),
		" mm",
	]
	.concat();

	gs.elev_str = [
		//&gs.gm_str.gm10,
		{
			//Must be less or equal
			if gd.elev <= gd.lp.wi.waterlevel {
				gs.s = [
					"Elevation (underwater): ",
					&(gd.elev.to_string()),
					" m ",
					"",
				]
				//&gs.gm_str.gm14]
				.concat();
				&gs.s
			} else {
				gs.s = ["Elevation: ", &(gd.elev.to_string()), " m ", ""]
					//&gs.gm_str.gm15]
					.concat();
				&gs.s
			}
		},
		"",
	]
	.concat();

	gs.water_str = [
		//&gs.gm_str.gm11,
		{
			match gd.water {
				0 => "", //&gs.gm_str.gm12,
				_ => {
					gs.s = [
						//&gs.gm_str.gm13,
						"Waterbody: ",
						&(gd.water.to_string()),
						"",
					]
					.concat();
					&gs.s
				}
			}
		},
		"",
	]
	.concat();

	gs.river_str = [
		//&gs.gm_str.gm18,
		{
			match gd.river_id {
				0 => "", //&gs.gm_str.gm19,
				_ => {
					gs.s = [
						//id
						//&gs.gm_str.gm20,
						"River ID: ",
						&(gd.river_id.to_string()),
						"\n",
						//width
						//&gs.gm_str.gm21,
						"River width: ",
						&(gd.river_width.to_string()),
						"\n",
						//element
						//&gs.gm_str.gm22,
						"River type: ",
						&(gd.river_element.to_string()),
						"\n",
						//upstream
						//&gs.gm_str.gm23,
						"River upstream: ",
						&(gd.river_upstream.to_string()),
						"\n",
						//downstream
						//&gs.gm_str.gm24,
						"River downstream: ",
						&(gd.river_downstream.to_string()),
						"",
					]
					.concat();
					&gs.s
				}
			}
		},
		"",
	]
	.concat();
}
