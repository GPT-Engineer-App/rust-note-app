use iced::{
    widget::{button, column, container, row, scrollable, text, text_input},
    Application, Command, Element, Length, Subscription,
};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    id: usize,
    title: String,
    content: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    AddNote,
    DeleteNote(usize),
    UpdateNoteTitle(usize, String),
    UpdateNoteContent(usize, String),
    SaveNotes,
    LoadNotes,
}

pub struct NotesApp {
    notes: Vec<Note>,
    next_id: usize,
}

impl Application for NotesApp {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                notes: Vec::new(),
                next_id: 0,
            },
            Command::perform(async {}, |_| Message::LoadNotes),
        )
    }

    fn title(&self) -> String {
        String::from("iOS Notes App")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::AddNote => {
                self.notes.push(Note {
                    id: self.next_id,
                    title: String::from("New Note"),
                    content: String::new(),
                });
                self.next_id += 1;
                Command::perform(async {}, |_| Message::SaveNotes)
            }
            Message::DeleteNote(id) => {
                self.notes.retain(|note| note.id != id);
                Command::perform(async {}, |_| Message::SaveNotes)
            }
            Message::UpdateNoteTitle(id, title) => {
                if let Some(note) = self.notes.iter_mut().find(|n| n.id == id) {
                    note.title = title;
                }
                Command::perform(async {}, |_| Message::SaveNotes)
            }
            Message::UpdateNoteContent(id, content) => {
                if let Some(note) = self.notes.iter_mut().find(|n| n.id == id) {
                    note.content = content;
                }
                Command::perform(async {}, |_| Message::SaveNotes)
            }
            Message::SaveNotes => {
                let json = serde_json::to_string(&self.notes).unwrap();
                fs::write("notes.json", json).unwrap();
                Command::none()
            }
            Message::LoadNotes => {
                if let Ok(json) = fs::read_to_string("notes.json") {
                    if let Ok(notes) = serde_json::from_str::<Vec<Note>>(&json) {
                        self.notes = notes;
                        self.next_id = self.notes.iter().map(|n| n.id).max().unwrap_or(0) + 1;
                    }
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let notes_list = self
            .notes
            .iter()
            .fold(column().spacing(10), |column, note| {
                column.push(
                    row()
                        .spacing(10)
                        .push(
                            text_input("Title", &note.title)
                                .on_input(move |title| Message::UpdateNoteTitle(note.id, title)),
                        )
                        .push(
                            button("Delete")
                                .on_press(Message::DeleteNote(note.id))
                                .style(iced::theme::Button::Destructive),
                        ),
                )
                .push(
                    text_input("Content", &note.content)
                        .on_input(move |content| Message::UpdateNoteContent(note.id, content)),
                )
            });

        let content = column()
            .spacing(20)
            .push(button("Add Note").on_press(Message::AddNote))
            .push(scrollable(notes_list));

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}