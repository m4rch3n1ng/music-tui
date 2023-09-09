use self::{player::Player, state::State, tui::Tui};
use crossterm::{
	event::{self, Event, KeyCode, KeyModifiers},
	execute, terminal,
};
use queue::Queue;
use ratatui::{
	prelude::{Backend, CrosstermBackend},
	Terminal,
};
use std::{
	io,
	time::{Duration, Instant},
};

mod player;
mod queue;
mod state;
mod tui;

#[derive(Debug)]
struct Application {
	pub player: Player,
	pub tui: Tui,
	pub state: State,
	pub queue: Queue,
	tick: Duration,
}

impl Application {
	pub fn new() -> Self {
		let tui = Tui;
		let state = State::init();
		let queue = Queue::state(&state);

		let mut player = Player::new();
		player.state(&queue, &state);

		let tick = Duration::from_millis(100);

		Application {
			player,
			tui,
			state,
			queue,
			tick,
		}
	}

	pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) {
		let mut last = Instant::now();
		let mut ticks = 0;

		loop {
			terminal.draw(|f| self.tui.ui(f, &self.state)).unwrap();

			let timeout = self.tick.saturating_sub(last.elapsed());
			if event::poll(timeout).unwrap() {
				// todo check if press
				if let Event::Key(key) = event::read().unwrap() {
					match (key.code, key.modifiers) {
						(KeyCode::Char('q'), _) => return,
						(KeyCode::Char(' '), _) => self.player.toggle(),
						(KeyCode::Char('m'), _) => self.player.mute(),
						(KeyCode::Up, KeyModifiers::SHIFT) => self.player.i_vol(5f64),
						(KeyCode::Down, KeyModifiers::SHIFT) => self.player.d_vol(5f64),
						(KeyCode::Right, KeyModifiers::SHIFT) => {
							self.queue.next(&mut self.player).unwrap()
						}
						(KeyCode::Left, KeyModifiers::SHIFT) => {
							self.queue.last(&mut self.player);
						}
						(KeyCode::Char('0'), _) => self.queue.restart(&mut self.player),
						(KeyCode::Left, KeyModifiers::NONE) => {
							self.queue.seek_d(&mut self.player, &self.state, 5)
						}
						(KeyCode::Right, KeyModifiers::NONE) => {
							self.queue.seek_i(&mut self.player, &self.state, 5)
						}
						_ => {}
					}
				}
			}

			if last.elapsed() >= self.tick {
				self.state.tick(&self.player, &self.queue);
				self.queue.done(&mut self.player, &self.state);

				last = Instant::now();

				// todo amt
				if ticks >= 10 {
					self.state.write();
					ticks = 0;
				} else {
					ticks += 1;
				}
			}
		}
	}

	pub fn start(&mut self) {
		terminal::enable_raw_mode().unwrap();

		let mut stdout = io::stdout();
		execute!(
			stdout,
			terminal::EnterAlternateScreen,
			event::EnableMouseCapture
		)
		.unwrap();

		let backend = CrosstermBackend::new(&stdout);
		let mut terminal = Terminal::new(backend).unwrap();

		self.run(&mut terminal);

		terminal::disable_raw_mode().unwrap();
		execute!(
			terminal.backend_mut(),
			terminal::LeaveAlternateScreen,
			event::DisableMouseCapture
		)
		.unwrap();

		terminal.show_cursor().unwrap();
	}
}

#[derive(Debug)]
struct Thing(u32);

fn main() {
	color_eyre::install().unwrap();

	let mut app = Application::new();

	app.start();
}
