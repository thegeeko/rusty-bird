use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 30.00;
enum GameMode {
	Menu,
	Playing,
	End,
}

struct State {
	mode: GameMode,
	player: Player,
	frame_time: f32,
	wall: Wall,
	score: i32,
}

struct Player {
	x: i32,
	y: i32,
	velocity: f32,
}

struct Wall {
	x: i32,
	gap_center: i32,
	gap_size: i32,
}

impl Wall {
	fn new(x: i32, score: i32) -> Self {
		let mut random = RandomNumberGenerator::new();
		Wall {
			x,
			gap_center: random.range(10, 40),
			gap_size: i32::max(4, 20 - score), // make it harder as the scores goes up
		}
	}
	fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
		let screen_x = self.x - player_x;
		let half_size = self.gap_size / 2;

		//draw from the top
		for y in 0..self.gap_center - half_size {
			ctx.set(screen_x, y, RED, BLACK, to_cp437('|'))
		}

		//draw from the bottom
		for y in self.gap_center + half_size..SCREEN_HEIGHT {
			ctx.set(screen_x, y, RED, BLACK, to_cp437('|'))
		}
	}
	fn hit_wall(&self, player: &Player) -> bool {
		let half_size = self.gap_size / 2;

		if player.x == self.x {
			//check if player above or below the gap
			return player.y < (self.gap_center - half_size) || player.y > (self.gap_center + half_size);
		} else {
			return false;
		}
	}
}

impl Player {
	fn new(x: i32, y: i32) -> Self {
		Player {
			x,
			y,
			velocity: 0.0,
		}
	}
	fn render(&mut self, ctx: &mut BTerm) {
		ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'))
	}
	fn gravity_movement(&mut self) {
		if self.velocity < 2.0 {
			self.velocity += 0.2;
		}
		self.y += self.velocity as i32;
		if self.y < 0 {
			// cap the player on the top
			self.y = 0
		}
		self.x += 1;
	}
	fn flap(&mut self) {
		self.velocity -= 2.0
	}
}

impl State {
	fn new() -> Self {
		State {
			player: Player::new(5, 25),
			frame_time: 0.0,
			mode: GameMode::Menu,
			wall: Wall::new(SCREEN_WIDTH, 0),
			score: 0,
		}
	}
	fn restart(&mut self) {
		self.player = Player::new(5, 25);
		self.frame_time = 0.0;
		self.mode = GameMode::Playing;
		self.score = 0;
		self.wall = Wall::new(SCREEN_WIDTH, 0)
	}
	fn play(&mut self, ctx: &mut BTerm) {
		ctx.cls_bg(NAVY);
		ctx.print(0, 0, "Press Space to flap");
		ctx.print(0, 2, &format!("Score : {}", self.score));
		self.frame_time += ctx.frame_time_ms;
		if self.frame_time > FRAME_DURATION {
			// limit the time the function called to 1000/FRAME_DURATION
			self.player.gravity_movement();
			self.frame_time = 0.0
		}
		if let Some(VirtualKeyCode::Space) = ctx.key {
			self.player.flap()
		}
		self.player.render(ctx);
		self.wall.render(ctx, self.player.x);
		if self.player.x > self.wall.x {
			self.score += 1;
			self.wall = Wall::new(SCREEN_WIDTH + self.player.x, self.score); // replace the old wall
		}

		if self.player.y > SCREEN_HEIGHT || self.wall.hit_wall(&self.player) {
			self.mode = GameMode::End;
		}
	}
	fn main_menu(&mut self, ctx: &mut BTerm) {
		ctx.print_centered(5, "Welcome to Rusty bird");
		ctx.print_centered(8, "(P) Play");
		ctx.print_centered(10, "(Q) Quit");

		if let Some(key) = ctx.key {
			match key {
				VirtualKeyCode::P => self.restart(),
				VirtualKeyCode::Q => ctx.quitting = true,
				_ => {}
			}
		}
	}
	fn dead(&mut self, ctx: &mut BTerm) {
		ctx.print_centered(5, format!("NT, {} Wanna try again ?", self.score));
		ctx.print_centered(8, "(P) let's go");
		ctx.print_centered(10, "(Q) nah");

		if let Some(key) = ctx.key {
			match key {
				VirtualKeyCode::P => self.restart(),
				VirtualKeyCode::Q => ctx.quitting = true,
				_ => {}
			}
		}
	}
}

impl GameState for State {
	fn tick(&mut self, ctx: &mut BTerm) {
		match self.mode {
			GameMode::Menu => self.main_menu(ctx),
			GameMode::End => self.dead(ctx),
			GameMode::Playing => self.play(ctx),
		}
	}
}

fn main() -> BError {
	let context = BTermBuilder::simple80x50()
		.with_title("Rusty bird")
		.build()?;
	main_loop(context, State::new())
}
