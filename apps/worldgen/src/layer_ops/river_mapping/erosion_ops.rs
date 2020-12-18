use crate::layer_ops::river_mapping::*;

pub fn map_erosion(
  rg: &mut RgParams,
  lp: &mut LayerPack,
) {
  //Must be cloned
  let erosion_queue = rg.rivers_paths.erosion_queue.clone();
  let mut to_do_queue = Vec::new();

  //Sort out the queue, take the values with greatest terrain_diff
  for entry in erosion_queue.iter() {
    let found_entry = to_do_queue.iter().find(
      |ErosionEntry {
         river_id_downstr: x,
         ..
       }| *x == entry.river_id_downstr,
    );

    //Entry index
    let found_entry_pos = to_do_queue.iter().position(
      |ErosionEntry {
         river_id_downstr: x,
         ..
       }| *x == entry.river_id_downstr,
    );

    match found_entry {
      Some(other_entry) => {
        //If same id recorded - compare their terrain_diff and
        //replace the entry if needed
        if entry.terrain_diff > other_entry.terrain_diff {
          to_do_queue[found_entry_pos.unwrap()] = *entry;
        }
      }
      None => {
        to_do_queue.push(*entry);
      }
    };
  }

  //Enact mapping, should be in reverse (I think)
  for entry in to_do_queue.iter().rev() {
    let river_id = entry.river_id_downstr;
    let terrain_diff = entry.terrain_diff;

    if river_id == NONE_ID_U16 {
      continue;
    }
    let river_entry = rg
      .rivers_paths
      .list
      .iter()
      .by_ref()
      .find(|RiverEntry { river_id: x, .. }| *x == river_id)
      .expect("river entry not found in erosion to do queue")
      .clone();

    let path_array = river_entry.path_array;
    erode_path(rg, lp, path_array, terrain_diff);
  }
}

fn erode_path(
  rg: &mut RgParams,
  lp: &mut LayerPack,
  path_array: Vec<path::Pos>,
  terrain_diff: u16,
) {
  //Aliases
  let erosion_width_max = lp.wi.river_erosion_width;

  let i_source = path_array[0].0;
  let j_source = path_array[0].1;
  let index_source = lp.xy.ind(i_source, j_source);

  //Level at source of river
  let terrain_source = lp.topography.read(lp.topography.TERRAIN, index_source);

  //Lower down the river if needed in order to avoid rivers flowing
  //above each other
  let erosion_floor = if lp.wi.waterlevel < lp.wi.abs_elev_min {
    0u16
  } else {
    translate::get_rel(
      lp.wi.waterlevel as f32,
      255.0,
      lp.wi.abs_elev_min as f32,
      lp.wi.abs_elev_max as f32,
    ) as u16
  };

  let mut terrain_stream = terrain_source.saturating_sub(terrain_diff);

  //Bound if needed
  if terrain_stream < erosion_floor {
    terrain_stream = erosion_floor;
  }

  //Iterate over path, modifying the terrain
  for n in path_array.windows(2) {
    let i0 = n[0].0;
    let j0 = n[0].1;
    let i1 = n[1].0;
    let j1 = n[1].1;

    //Aliases
    let index_downstr = lp.xy.ind(i1, j1);
    let index_current = lp.xy.ind(i0, j0);
    let terrain_current =
      lp.topography.read(lp.topography.TERRAIN, index_current);
    let terrain_downstr =
      lp.topography.read(lp.topography.TERRAIN, index_downstr);
    let river_elem_downstr = lp.rivers.read(lp.rivers.ELEMENT, index_downstr);

    //Cease if reached the end
    if river_elem_downstr == NO_RIVER {
      break;
    }

    //Make sure current doesn't go up
    if terrain_current > terrain_stream {
      lp.topography
        .write(terrain_stream, lp.topography.TERRAIN, index_current);
    }

    //Skip if erosion floor is reached
    if terrain_current < erosion_floor {
      continue;
    }

    //Erode if downstream goes up
    if terrain_downstr > terrain_current {
      lp.topography.write(
        terrain_current,
        lp.topography.TERRAIN,
        index_downstr,
      );
    }

    //Erode a square area of terrain of given width
    for erosion_width in 1..erosion_width_max {
      let double_width = erosion_width * 2;
      for i in 0..double_width {
        let shif_i: isize = i as isize - erosion_width as isize;

        let erosion_i = (i0 as isize + shif_i) as usize;

        for j in 0..double_width {
          let shif_j: isize = j as isize - erosion_width as isize;

          let erosion_j = (j0 as isize + shif_j) as usize;

          erosion(rg, lp, erosion_width, erosion_i, erosion_j, terrain_current);
        }
      }
    }
  }
}

fn erosion(
  _rg: &mut RgParams,
  lp: &mut LayerPack,
  erosion_width_iter: usize,
  erosion_i: usize,
  erosion_j: usize,
  terrain_current: u16,
) {
  //Aliases
  let map_size = lp.wi.map_size;

  //Check if within the map
  if (erosion_i < map_size) && (erosion_j < map_size) {
    let index = lp.xy.ind(erosion_i, erosion_j);

    if lp.topography.read(lp.topography.TERRAIN, index) > terrain_current {
      let terrain_current = terrain_current as f32;
      let terrain_to_erode =
        lp.topography.read(lp.topography.TERRAIN, index) as f32;

      let terrain_relative: f32 = terrain_to_erode / 255.0;

      let value = (terrain_to_erode.powf((1.0 - terrain_relative).powf(
        terrain_relative
          / ((erosion_width_iter as f32).powf(lp.wi.river_erosion_smooth)
            * (1.0 - terrain_relative)),
      ))) as u16;

      //Write the eroded terrain back onto map
      lp.topography.write(value, lp.topography.TERRAIN, index);

      //Bound so as to avoid excessive erosion
      if lp.topography.read(lp.topography.TERRAIN, index)
        < terrain_current as u16
      {
        lp.topography.write(
          terrain_current as u16,
          lp.topography.TERRAIN,
          index,
        );
      }
    }
  }
}
