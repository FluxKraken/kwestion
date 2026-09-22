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

fn clear_terminal(terminal: &mut DefaultTerminal) -> Result<()> {
    terminal.draw(|f| {
        f.render_widget(Clear, f.area());
    })?;
    Ok(())
}

pub fn prompt(prompt: impl Into<String>) -> Result<String> {
    let options = TerminalOptions {
        viewport: Viewport::Inline(3),
    };

    let mut terminal = ratatui::try_init_with_options(options)?;
    let mut textarea = TextArea::default();

    clear_terminal(&mut terminal)?;
    let input_title = prompt.into();

    loop {
        terminal.draw(|frame| {
            let area = frame.area();

            let container = Layout::vertical([Constraint::Fill(1)])
                .margin(1)
                .split(area);

            let surface = Block::default()
                .style(Style::default().bg(Color::Rgb(0, 0, 30)).fg(Color::White))
                .title(Span::from(input_title.as_str()).style(Style::default().fg(Color::White)))
                .border_style(Style::default().fg(Color::from_hsl(Hsl::new(200.0, 1.0, 0.5))))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);

            frame.render_widget(surface, area);
            frame.render_widget(&textarea, container[0]);
        })?;

        if poll(Duration::from_millis(500))?
            && let Event::Key(key) = event::read()?
            && key.is_press()
        {
            match key.code {
                KeyCode::Esc => {
                    textarea.clear();
                }
                KeyCode::Enter => break,
                _ => {
                    textarea.input_without_shortcuts(key);
                }
            }
        }
    }

    clear_terminal(&mut terminal)?;
    ratatui::restore();

    execute!(stdout(), MoveUp(2), MoveToColumn(0),)?;

    Ok(textarea.lines().join(""))
}
