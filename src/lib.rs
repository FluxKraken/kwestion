use anyhow::Result;
use crossterm::{
    cursor::{MoveToColumn, MoveUp},
    event::{self, Event, KeyCode, poll},
    execute,
};
use ratatui::{
    DefaultTerminal, TerminalOptions, Viewport,
    layout::{Constraint, Layout},
    palette::Hsl,
    style::{Color, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Clear},
};
use ratatui_textarea::TextArea;
use std::{io::stdout, time::Duration};

mod options;
mod prompt;

pub use crate::options::{PromptOptions, prompt_with_options};
pub use crate::prompt::prompt;

fn clear_terminal(terminal: &mut DefaultTerminal) -> Result<()> {
    terminal.draw(|f| {
        f.render_widget(Clear, f.area());
    })?;
    Ok(())
}
