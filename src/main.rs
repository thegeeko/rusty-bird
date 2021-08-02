use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;
enum GameMode {
	Menu,
	Playing,
	End,
}

struct State {
	mode: GameMode,
	player: Player,
	frame_time: f32,
}

struct Player {
	x: i32,
	y: i32,
	velocity: f32,
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
		ctx.set(self.x, self.y, YELLOW, BLACK, to_cp437('@'))
	}
	fn gravity_movement(&mut self) {
		if self.velocity < 2.0 {
			self.velocity += 0.2;
		}
		self.y += self.velocity as i32;
		if self.y < 0 {
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
		}
	}
	fn restart(&mut self) {
		self.player = Player::new(5, 25);
		self.frame_time = 0.0;
		self.mode = GameMode::Playing;
	}
	fn play(&mut self, ctx: &mut BTerm) {
		ctx.cls_bg(NAVY);
		ctx.print(0, 0, "Press Space to flap");
		self.frame_time += ctx.frame_time_ms;
		if self.frame_time > FRAME_DURATION {
			self.player.gravity_movement();
			self.frame_time = 0.0
		}
		if let Some(VirtualKeyCode::Space) = ctx.key {
			self.player.flap()
		}
		self.player.render(ctx);
		if self.player.y > SCREEN_HEIGHT {
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
		ctx.print_centered(5, "NT, Wanna try again ?");
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
