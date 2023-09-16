use self::popup::{Lyrics, Popup, Tags, Tracks};
use crate::{player::Player, queue::Queue, state::State};
use ratatui::{prelude::Rect, Frame};

mod popup;
mod utils;
mod window;

#[derive(Debug, Clone, Copy)]
pub enum Popups {
	Tracks,
	Tags,
	Lyrics,
}

#[derive(Debug)]
pub struct Ui {
	tags: Tags,
	lyrics: Lyrics,
	tracks: Tracks,
	popup: Option<Popups>,
}

impl Ui {
	pub fn new(queue: &Queue) -> Self {
		Ui {
			tags: Tags::default(),
			lyrics: Lyrics::default(),
			tracks: Tracks::new(queue),
			popup: None,
		}
	}

	pub fn draw(&mut self, frame: &mut Frame, state: &State, queue: &Queue) {
		let size = frame.size();
		let (window, seek) = window::layout(size);

		window::main(frame, window, state);
		window::seek(frame, seek, state);

		self.popup(frame, window, state, queue);
	}

	// todo make generic maybe ?
	fn popup(
		&mut self,
		frame: &mut Frame,
		main: Rect,
		state: &State,
		queue: &Queue,
	) {
		let area = window::popup(main);
		match self.popup {
			Some(Popups::Tags) => {
				self.tags.update_scroll(area, state);
				self.tags.draw(frame, area, state);
			}
			Some(Popups::Tracks) => self.tracks.draw(frame, area, queue),
			Some(Popups::Lyrics) => {
				self.lyrics.update_scroll(area, state);
				self.lyrics.draw(frame, area, state);
			}
			None => {}
		}
	}

	pub fn reset(&mut self) {
		self.tags.reset();
		self.lyrics.reset();
	}

	pub fn tracks(&mut self) {
		if let Some(Popups::Tracks) = self.popup {
			self.popup = None;
		} else {
			self.popup = Some(Popups::Tracks);
		}
	}

	pub fn tags(&mut self) {
		if let Some(Popups::Tags) = self.popup {
			self.popup = None;
		} else {
			self.popup = Some(Popups::Tags);
		}
	}

	pub fn lyrics(&mut self) {
		if let Some(Popups::Lyrics) = self.popup {
			self.popup = None;
		} else {
			self.popup = Some(Popups::Lyrics);
		}
	}

	pub fn up(&mut self) {
		match self.popup {
			Some(Popups::Tags) => self.tags.up(),
			Some(Popups::Tracks) => self.tracks.up(),
			Some(Popups::Lyrics) => self.lyrics.up(),
			None => {}
		}
	}

	pub fn down(&mut self) {
		match self.popup {
			Some(Popups::Tags) => self.tags.down(),
			Some(Popups::Tracks) => self.tracks.down(),
			Some(Popups::Lyrics) => self.lyrics.down(),
			None => {}
		}
	}

	pub fn enter(&mut self, player: &mut Player, queue: &mut Queue) {
		if let Some(Popups::Tracks) = self.popup {
			self.tracks.enter(player, queue);
		}
	}

	pub fn esc(&mut self) {
		self.popup = None;
	}
}
