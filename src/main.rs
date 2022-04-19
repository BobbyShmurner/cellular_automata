use bevy::{
	prelude::*,
	render::mesh::*,
	ecs::system::QuerySingleError
};

use core::panic;
use std::vec;

const BACKGROUND_COLOR: Color = Color::rgb(0.53, 0.81, 0.92);
const SIM_SPEED: f32 = 0.1;
const CAM_SPEED: f32 = 0.5;

const START_SIZE: usize = 4;

const GRID_WIDTH:   usize = 20; // X
const GRID_HEIGHT:  usize = 20; // Y
const GRID_DEPTH:   usize = 20; // Z

const NEIGHBOURS_TO_SURVIVE: 		u32 = 4;
const NEIGHBOURS_TO_BECOME_ALIVE:	u32 = 4;
const STATE_VALUE:					u32 = 3;
const DYING_COUNTS_AS_NEIGHBOUR:	bool = true;

pub struct Grid {
	vals: [[[u32; GRID_DEPTH] ; GRID_HEIGHT] ; GRID_WIDTH],
}

impl Grid {
	pub fn enable_cell(self: &mut Grid, x: usize, y: usize, z: usize) {
		self.vals[x][y][z] = STATE_VALUE;
	}

	pub fn kill_cell(self: &mut Grid, x: usize, y: usize, z: usize) {
		self.vals[x][y][z] -= 1;
	}
	
	pub fn is_cell_alive(self: &Grid, x: usize, y: usize, z: usize) -> bool {
		if DYING_COUNTS_AS_NEIGHBOUR { return self.vals[x][y][z] != 0; }
		return self.vals[x][y][z] == STATE_VALUE;
	}

	pub fn get_cell_state(self: &Grid, x: usize, y: usize, z: usize) -> u32 {
		return self.vals[x][y][z];
	}

	pub fn set_cell_state(self: &mut Grid, x: usize, y: usize, z: usize, val: u32) {
		self.vals[x][y][z] = val;
	}

	pub fn get_neighbour_count(self: &Grid, x: usize, y: usize, z: usize) -> u32 {
		let mut neighbour_count = 0;

		if x != 0 && y != 0 && z != 0                							&& self.is_cell_alive(x - 1, y - 1, z - 1)	 { neighbour_count += 1; }
		if y != 0 && z != 0                               							&& self.is_cell_alive(x, y - 1, z - 1)		 { neighbour_count += 1; }
		if x != GRID_WIDTH - 1 && y != 0 && z != 0								&& self.is_cell_alive(x + 1, y - 1, z - 1)	 { neighbour_count += 1; }

		if x != 0 && z != 0				 											&& self.is_cell_alive(x - 1, y, z - 1)		 { neighbour_count += 1; }
		if z != 0								 											&& self.is_cell_alive(x, y, z - 1)			 { neighbour_count += 1; }
		if x != GRID_WIDTH - 1 && z != 0	 											&& self.is_cell_alive(x + 1, y, z - 1)		 { neighbour_count += 1; }

		if x != 0 && y != GRID_HEIGHT - 1 && z != 0								&& self.is_cell_alive(x - 1, y + 1, z - 1)	 { neighbour_count += 1; }
		if y != GRID_HEIGHT - 1 && z != 0												&& self.is_cell_alive(x, y + 1, z - 1)		 { neighbour_count += 1; }
		if x != GRID_WIDTH - 1 && y != GRID_HEIGHT - 1 && z != 0					&& self.is_cell_alive(x + 1, y + 1, z - 1)	 { neighbour_count += 1; }

		if x != 0 && y != 0                											&& self.is_cell_alive(x - 1, y - 1, z)		 { neighbour_count += 1; }
		if y != 0                               											&& self.is_cell_alive(x, y - 1, z)			 { neighbour_count += 1; }
		if x != GRID_WIDTH - 1 && y != 0												&& self.is_cell_alive(x + 1, y - 1, z)		 { neighbour_count += 1; }

		if x != 0				 															&& self.is_cell_alive(x - 1, y, z)			 { neighbour_count += 1; }
		if x != GRID_WIDTH - 1	 															&& self.is_cell_alive(x + 1, y, z)			 { neighbour_count += 1; }

		if x != 0 && y != GRID_HEIGHT - 1												&& self.is_cell_alive(x - 1, y + 1, z)		 { neighbour_count += 1; }
		if y != GRID_HEIGHT - 1															&& self.is_cell_alive(x, y + 1, z)			 { neighbour_count += 1; }
		if x != GRID_WIDTH - 1 && y != GRID_HEIGHT - 1								&& self.is_cell_alive(x + 1, y + 1, z)		 { neighbour_count += 1; }

		if x != 0 && y != 0 && z != GRID_DEPTH - 1                				&& self.is_cell_alive(x - 1, y - 1, z + 1)	 { neighbour_count += 1; }
		if y != 0 && z != GRID_DEPTH - 1                               				&& self.is_cell_alive(x, y - 1, z + 1)		 { neighbour_count += 1; }
		if x != GRID_WIDTH - 1 && y != 0 && z != GRID_DEPTH - 1					&& self.is_cell_alive(x + 1, y - 1, z + 1)	 { neighbour_count += 1; }

		if x != 0 && z != GRID_DEPTH - 1				 								&& self.is_cell_alive(x - 1, y, z + 1)		 { neighbour_count += 1; }
		if z != GRID_DEPTH - 1								 								&& self.is_cell_alive(x, y, z + 1)			 { neighbour_count += 1; }
		if x != GRID_WIDTH - 1 && z != GRID_DEPTH - 1	 								&& self.is_cell_alive(x + 1, y, z + 1)		 { neighbour_count += 1; }
		
		if x != 0 && y != GRID_HEIGHT - 1 && z != GRID_DEPTH - 1					&& self.is_cell_alive(x - 1, y + 1, z + 1)	 { neighbour_count += 1; }
		if y != GRID_HEIGHT - 1 && z != GRID_DEPTH - 1								&& self.is_cell_alive(x, y + 1, z + 1)		 { neighbour_count += 1; }
		if x != GRID_WIDTH - 1 && y != GRID_HEIGHT - 1 && z != GRID_DEPTH - 1	&& self.is_cell_alive(x + 1, y + 1, z + 1)	 { neighbour_count += 1; }

		return neighbour_count;
	}

	pub fn generate_faces(self: &Grid, x: usize, y: usize, z: usize, mut face_count: u32) -> (Vec<[f32; 3]>, Vec<u32>, usize) {
		let mut verts: Vec<[f32; 3]> = Vec::new();
		let mut tris: Vec<u32> = Vec::new();

		if z == 0 || !self.is_cell_alive(x, y, z - 1) { 
			let mut new_verts = vec![[x as f32, y as f32 + 1.0, z as f32], [x as f32 + 1.0, y as f32 + 1.0, z as f32], [x as f32 + 1.0, y as f32, z as f32], [x as f32, y as f32, z as f32]];
			let mut new_tris = vec![
				face_count * 4,
				face_count * 4 + 1,
				face_count * 4 + 2,
				face_count * 4 + 2,
				face_count * 4 + 3,
				face_count * 4
			];

			verts.append(&mut new_verts);
			tris.append(&mut new_tris);

			face_count += 1;
		}


		if y == 0 || !self.is_cell_alive(x, y - 1, z) { 
			let mut new_verts = vec![[x as f32, y as f32, z as f32], [x as f32 + 1.0, y as f32, z as f32], [x as f32 + 1.0, y as f32, z as f32 + 1.0], [x as f32, y as f32, z as f32 + 1.0]];
			let mut new_tris = vec![
				face_count * 4,
				face_count * 4 + 1,
				face_count * 4 + 2,
				face_count * 4 + 2,
				face_count * 4 + 3,
				face_count * 4
			];

			verts.append(&mut new_verts);
			tris.append(&mut new_tris);

			face_count += 1;
		}

		if x == 0 || !self.is_cell_alive(x - 1, y, z) { 
			let mut new_verts = vec![[x as f32, y as f32, z as f32 + 1.0], [x as f32, y as f32 + 1.0, z as f32 + 1.0], [x as f32, y as f32 + 1.0, z as f32], [x as f32, y as f32, z as f32]];
			let mut new_tris = vec![
				face_count * 4,
				face_count * 4 + 1,
				face_count * 4 + 2,
				face_count * 4 + 2,
				face_count * 4 + 3,
				face_count * 4
			];

			verts.append(&mut new_verts);
			tris.append(&mut new_tris);

			face_count += 1;
		}

		if x == GRID_WIDTH - 1 || !self.is_cell_alive(x + 1, y, z) { 
			let mut new_verts = vec![[x as f32 + 1.0, y as f32, z as f32], [x as f32 + 1.0, y as f32 + 1.0, z as f32], [x as f32 + 1.0, y as f32 + 1.0, z as f32 + 1.0], [x as f32 + 1.0, y as f32, z as f32 + 1.0]];
			let mut new_tris = vec![
				face_count * 4,
				face_count * 4 + 1,
				face_count * 4 + 2,
				face_count * 4 + 2,
				face_count * 4 + 3,
				face_count * 4
			];

			verts.append(&mut new_verts);
			tris.append(&mut new_tris);

			face_count += 1;
		}

		if y == GRID_HEIGHT - 1 || !self.is_cell_alive(x, y + 1, z) { 
			let mut new_verts = vec![[x as f32, y as f32 + 1.0, z as f32 + 1.0], [x as f32 + 1.0, y as f32 + 1.0, z as f32 + 1.0], [x as f32 + 1.0, y as f32 + 1.0, z as f32], [x as f32, y as f32 + 1.0, z as f32]];
			let mut new_tris = vec![
				face_count * 4,
				face_count * 4 + 1,
				face_count * 4 + 2,
				face_count * 4 + 2,
				face_count * 4 + 3,
				face_count * 4
			];

			verts.append(&mut new_verts);
			tris.append(&mut new_tris);

			face_count += 1;
		}

		if z == GRID_DEPTH - 1	|| !self.is_cell_alive(x, y, z + 1) { 
			let mut new_verts = vec![[x as f32, y as f32, z as f32 + 1.0], [x as f32 + 1.0, y as f32, z as f32 + 1.0], [x as f32 + 1.0, y as f32 + 1.0, z as f32 + 1.0], [x as f32, y as f32 + 1.0, z as f32 + 1.0]];
			let mut new_tris = vec![
				face_count * 4,
				face_count * 4 + 1,
				face_count * 4 + 2,
				face_count * 4 + 2,
				face_count * 4 + 3,
				face_count * 4
			];

			verts.append(&mut new_verts);
			tris.append(&mut new_tris);

			face_count += 1;
		}

		return (verts, tris, face_count as usize);
	}
}

impl Default for Grid {
	fn default() -> Self {
		Grid {
			vals: [[[0; GRID_DEPTH] ; GRID_HEIGHT] ; GRID_WIDTH],
		}
	}
}

pub struct SetupScene;

impl Plugin for SetupScene {
	fn build(&self, app: &mut App) {
		app.insert_resource(AmbientLight {
			color: Color::WHITE,
			brightness: 1.0
		})
		.add_startup_system(setup_base_scene);
	}
}

#[derive(Component)]
struct RotateAboutPoint {
	pub center: Vec3,
	pub speed: f32,
	pub distance: f32,
	pub look_at_point: bool
}

impl Default for RotateAboutPoint {
	fn default() -> Self {
		RotateAboutPoint {
			center: Vec3::ZERO,
			speed: 1.0,
			distance: 2.0,
			look_at_point: true
		}
	}
}

#[derive(Component)]
struct GridMesh;

fn rotate_about_point(
	mut query: Query<(&RotateAboutPoint, &mut Transform)>,
	time: Res<Time>
) {
	for (rotate_comp, mut transform) in query.iter_mut() {
		let rot_pos = Vec3::new(
			f32::sin(time.seconds_since_startup() as f32 * rotate_comp.speed) * rotate_comp.distance,
			0.0,
			f32::cos(time.seconds_since_startup() as f32 * rotate_comp.speed) * rotate_comp.distance
		);

		transform.translation = rotate_comp.center + rot_pos;
		if rotate_comp.look_at_point {
			let new_rot = transform.looking_at(rotate_comp.center, Vec3::Y).rotation;
			transform.rotation = new_rot;
		}
	}
}

fn tick(
	mut grid: ResMut<Grid>,

	time: Res<Time>,
	timer: ResMut<TickTimer>
) {
	if !should_tick(time, timer) { return; }

	for x in 0..GRID_WIDTH {
		for y in 0..GRID_HEIGHT {
			for z in 0..GRID_DEPTH {
				let state = grid.get_cell_state(x, y, z);
				match state {
					STATE_VALUE => {
						if grid.get_neighbour_count(x, y, z) != NEIGHBOURS_TO_SURVIVE {
							grid.kill_cell(x, y, z);
						}
					}
					0 => {
						if grid.get_neighbour_count(x, y, z) == NEIGHBOURS_TO_BECOME_ALIVE {
							grid.enable_cell(x, y, z);
						}
					}
					_ => {
						grid.set_cell_state(x, y, z, state - 1);
					}
				}
				grid.get_neighbour_count(x, y, z);
			}
		}
	}
}

fn generate_mesh(
	grid: ResMut<Grid>,
	query: Query<Entity, With<GridMesh>>,
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,

	time: Res<Time>,
	timer: ResMut<MeshTimer>
) {
	if !should_update_mesh(time, timer) { return; }

	let mut verts: Vec<[f32; 3]> = Vec::new();
	let mut tris: Vec<u32> = Vec::new();

	let mut face_count: usize = 0;

	for x in 0..GRID_WIDTH {
		for y in 0..GRID_HEIGHT {
			for z in 0..GRID_DEPTH {
				if !grid.is_cell_alive(x, y, z) { continue; }

				let (mut new_verts, mut new_tris, new_face_count) = grid.generate_faces(x, y, z, face_count as u32);

				verts.append(&mut new_verts);
				tris.append(&mut new_tris);
				face_count = new_face_count;
			}
		}
	}

	let mesh_query = query.get_single();
	match mesh_query {
		Ok(_) => {
			commands.entity(mesh_query.unwrap()).despawn();
		}
		Err(QuerySingleError::MultipleEntities(_)) => {
			panic!("Multiple Gird Meshes Found!");
		}
		_ => {}
	}

	let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

	mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, verts);
	mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 0.0]; 4 * face_count]);
	mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0, 0.0]; 4 * face_count]);

	mesh.set_indices(Some(Indices::U32(tris)));

	commands.spawn_bundle(PbrBundle {
		mesh: meshes.add(mesh),
		material: materials.add(StandardMaterial {
			base_color: Color::RED,
			unlit: true,
			..default()
		}),
		..default()
	})
	.insert(GridMesh);
}

fn setup_base_scene(
	mut commands: Commands,
) {
	// Camera
	commands.spawn_bundle(PerspectiveCameraBundle {
		transform: Transform::from_xyz(-20.0, 25.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
		..default()
	})
	.insert(RotateAboutPoint {
		distance: 50.0,
		speed: -CAM_SPEED,
		center: Vec3::new(GRID_WIDTH as f32 / 2.0, GRID_HEIGHT as f32 / 2.0, GRID_DEPTH as f32 / 2.0),
		..default()
	});
}

fn setup_starting_grid(mut grid: ResMut<Grid>) {
	for x in 0..GRID_WIDTH {
		for y in 0..GRID_HEIGHT {
			for z in 0..GRID_DEPTH {
				if x < GRID_WIDTH / 2 - START_SIZE / 2 || x > GRID_WIDTH / 2 + START_SIZE / 2 { continue; }
				if y < GRID_HEIGHT / 2 - START_SIZE / 2 || y > GRID_HEIGHT / 2 + START_SIZE / 2 { continue; }
				if x < GRID_DEPTH / 2 - START_SIZE / 2 || z > GRID_DEPTH / 2 + START_SIZE / 2 { continue; }

				grid.enable_cell(x, y, z);
			}
		}
	}
}

struct MeshTimer(Timer);
struct TickTimer(Timer);

fn should_update_mesh(
	time: Res<Time>,
	mut timer: ResMut<MeshTimer>
) -> bool {
	return timer.0.tick(time.delta()).just_finished();
}

fn should_tick(
	time: Res<Time>,
	mut timer: ResMut<TickTimer>
) -> bool {
	return timer.0.tick(time.delta()).just_finished();
}

fn main() {
	App::new()
	.insert_resource(ClearColor(BACKGROUND_COLOR))
	.insert_resource(Msaa { samples: 4 })
	.insert_resource(MeshTimer(Timer::from_seconds(SIM_SPEED, true)))
	.insert_resource(TickTimer(Timer::from_seconds(SIM_SPEED, true)))
	.init_resource::<Grid>()
	.add_plugins(DefaultPlugins)
	.add_plugin(SetupScene)
	.add_startup_system(setup_starting_grid)
	.add_system(rotate_about_point)
	.add_system_set(
		SystemSet::new()
			.with_system(tick)
			.with_system(generate_mesh.after(tick))
	)
	.run();
}