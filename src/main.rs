use iced::{Application, Command, Element, Settings, Theme};
use ios_notes_app::NotesApp;

pub fn main() -> iced::Result {
    NotesApp::run(Settings::default())
}