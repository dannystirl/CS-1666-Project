extern crate rogue_sdl;

use sdl2::rect::Rect;
use crate::gamedata::*;

pub struct Projectile{
	src: Rect, 
	pos: Rect,
	pub facing_right: bool,
	is_active: bool,
	vector: Vec<f64>,
}

 impl Projectile {
	pub fn new(pos: Rect, use_ability:bool, facing_right: bool, frame:i32, vector: Vec<f64>) -> Projectile {
		let src = Rect::new(0 , 0 , TILE_SIZE, TILE_SIZE);
		let is_active = true;
		Projectile {
			src, 
			pos,	
			facing_right,
			is_active,
			vector
		}
	}
	pub fn x(&self) -> i32 {
		return self.pos.x;
	}
	
	pub fn y(&self) -> i32 {
		return self.pos.y;
	}

	 pub fn set_x(&mut self, x: i32){
		 self.pos.x = x;
	 }
	 pub fn set_y(&mut self, y: i32){
		 self.pos.y = y;
	 }
	pub fn is_active(&self) -> bool{
		return self.is_active;
	}
	// the frames aren't calculating right so the fireball image doesnt look right, but the logic is there. 
	pub fn update_pos(&mut self, x_bounds: (i32, i32)) {
		self.set_x(self.x() + self.vector[0] as i32);
		self.set_y(self.y() + self.vector[1] as i32);
	}
	pub fn set_pos(&mut self, p:Rect){
		self.pos = p;
	}
	pub fn src(&self, col: i32, row: i32) -> Rect{
		return Rect::new(
			0,
			0,
			//(self.frame % col) * (TILE_SIZE as i32) * 3/2,
			//(self.frame % row) * (TILE_SIZE as i32),
			TILE_SIZE,
			TILE_SIZE,
		);
	}
	 pub fn die(&mut self){
		 // Set death animation when created
		 self.is_active = false;
	}
    pub fn pos(&self) -> Rect {
		self.pos
    }
}
