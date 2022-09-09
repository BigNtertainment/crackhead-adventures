use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;
use navmesh::{NavMesh, NavVec3, NavTriangle};

pub struct EnemyNavMesh {
	vertices: Vec<NavVec3>,
	triangles: Vec<NavTriangle>,
	nav_mesh: Option<NavMesh>
}

#[allow(unused)]
impl EnemyNavMesh {
	pub fn new() -> Self {
		EnemyNavMesh {
			vertices: Vec::new(),
			triangles: Vec::new(),
			nav_mesh: None
		}
	}

	pub fn bake(&mut self) {
		// The clones might make a bottleneck here, if there are some optimization problems, make sure it's not this
		self.nav_mesh = Some(NavMesh::new(self.vertices.clone(), self.triangles.clone()).expect("Invalid input for baking the NavMesh"));
	}

	pub fn get_nav_mesh(&self) -> Option<&NavMesh> {
		self.nav_mesh.as_ref()
	}

	fn vertex_index(&mut self, vertex: NavVec3) -> usize {
		match self.vertices.iter().position(|&r| r == vertex) {
			Some(index) => index,
			None => {
				self.vertices.push(vertex);
				self.vertices.len() - 1
			}
		}
	}

	pub fn insert_triangle(&mut self, p1: Vec2, p2: Vec2, p3: Vec2) {
		let indices = (
			self.vertex_index(NavVec3::new(p1.x, p1.y, 0.0)) as u32,
			self.vertex_index(NavVec3::new(p2.x, p2.y, 0.0)) as u32,
			self.vertex_index(NavVec3::new(p3.x, p3.y, 0.0)) as u32,
		);

		self.triangles.push(indices.into());
	}

	pub fn insert_rect(&mut self, p1: Vec2, p2: Vec2, p3: Vec2, p4: Vec2) {
		let indices = (
			self.vertex_index(NavVec3::new(p1.x, p1.y, 0.0)) as u32,
			self.vertex_index(NavVec3::new(p2.x, p2.y, 0.0)) as u32,
			self.vertex_index(NavVec3::new(p3.x, p3.y, 0.0)) as u32,
			self.vertex_index(NavVec3::new(p4.x, p4.y, 0.0)) as u32,
		);

		self.triangles.push((indices.0, indices.1, indices.2).into());
		self.triangles.push((indices.0, indices.3, indices.2).into());
	}

	pub fn clear(&mut self) {
		self.vertices.clear();
		self.triangles.clear();
		self.nav_mesh = None;
	}

	pub fn debug(&self) {
		for area in self.nav_mesh.as_ref().unwrap().areas() {
			print!("({}, {}), ", area.center.x, area.center.y);
		}

		println!();
	}

	pub fn draw(&self, lines: &mut ResMut<DebugLines>) {
		let vertices = self.nav_mesh.as_ref().unwrap().vertices();

		for triangle in self.nav_mesh.as_ref().unwrap().triangles() {
			let first = Vec3::new(vertices[triangle.first as usize].x, vertices[triangle.first as usize].y, 0.0);
			let second = Vec3::new(vertices[triangle.second as usize].x, vertices[triangle.second as usize].y, 0.0);
			let third = Vec3::new(vertices[triangle.third as usize].x, vertices[triangle.third as usize].y, 0.0);

			lines.line(
				first,
				second,
				0.0
			);

			lines.line(
				second,
				third,
				0.0
			);

			lines.line(
				first,
				third,
				0.0
			);
		}
	}
}