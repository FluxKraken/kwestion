use crossterm::cursor::MoveDown;
use ratatui::widgets::Paragraph;

use super::*;

#[derive(Default, Clone)]
pub struct PromptOptions {
    bg: Color,
    fg: Color,
    border: Color,
    title: Color,
    label: bool,
    label_text: String,
    label_width: u16,
    clear: bool,
}

impl PromptOptions {
    pub fn bg(mut self, hue: u16, saturation: u16, lightness: u16) -> Self {
        self.bg = Color::from_hsl(Hsl::new(
            hue as f32,
            saturation as f32 / 100.0,
            lightness as f32 / 100.0,
        ));
        self
    }

    pub fn fg(mut self, hue: u16, saturation: u16, lightness: u16) -> Self {
        self.fg = Color::from_hsl(Hsl::new(
            hue as f32,
            saturation as f32 / 100.0,
            lightness as f32 / 100.0,
        ));
        self
    }

    pub fn border(mut self, hue: u16, saturation: u16, lightness: u16) -> Self {
        self.border = Color::from_hsl(Hsl::new(
            hue as f32,
            saturation as f32 / 100.0,
            lightness as f32 / 100.0,
        ));
        self
    }

    pub fn title(mut self, hue: u16, saturation: u16, lightness: u16) -> Self {
        self.title = Color::from_hsl(Hsl::new(
            hue as f32,
            saturation as f32 / 100.0,
            lightness as f32 / 100.0,
        ));
        self
    }

    pub fn label(mut self, text: impl Into<String>) -> Self {
        self.label = true;
        self.label_text = text.into();
        self.label_width = self.label_text.chars().count() as u16;
        self
    }

    pub fn clear(mut self, should_clear: bool) -> Self {
        self.clear = should_clear;
        self
    }
}

pub fn prompt_with_options(
    prompt: impl Into<String>,
    prompt_options: PromptOptions,
) -> Result<String> {
    let options = TerminalOptions {
        viewport: Viewport::Inline(3),
    };

    let mut terminal = ratatui::try_init_with_options(options)?;
    let mut textarea = TextArea::default();

    if prompt_options.clear {
        clear_terminal(&mut terminal)?;
    }

    let input_title = prompt.into();

    loop {
        terminal.draw(|frame| {
            let area = frame.area();

            let container = Layout::vertical([Constraint::Fill(1)])
                .margin(1)
                .split(area);

            let [label_area, input_area] = Layout::horizontal([
                Constraint::Max(prompt_options.label_width + 2),
                Constraint::Fill(1),
            ])
            .spacing(1)
            .areas(area);

            let input_area_container = Layout::vertical([Constraint::Fill(1)])
                .margin(1)
                .split(input_area);

            let label_area_container = Layout::vertical([Constraint::Fill(1)])
                .margin(1)
                .split(label_area);

            let input_surface = Block::default()
                .style(Style::default().bg(prompt_options.bg).fg(prompt_options.fg))
                .title(
                    Span::from(input_title.as_str())
                        .style(Style::default().fg(prompt_options.title)),
                )
                .border_style(Style::default().fg(prompt_options.border))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);

            let label_surface = Block::default()
                .style(Style::default().bg(prompt_options.bg).fg(prompt_options.fg));

            match prompt_options.label {
                true => {
                    frame.render_widget(label_surface, area);
                    frame.render_widget(
                        Paragraph::new(prompt_options.label_text.as_str()).style(
                            Style::default()
                                .bg(prompt_options.bg)
                                .fg(prompt_options.fg)
                                .bold(),
                        ),
                        label_area_container[0],
                    );
                    frame.render_widget(input_surface, input_area);
                    frame.render_widget(&textarea, input_area_container[0]);
                }
                false => {
                    frame.render_widget(input_surface, area);
                    frame.render_widget(&textarea, container[0]);
                }
            }
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

    if prompt_options.clear {
        clear_terminal(&mut terminal)?;
        ratatui::restore();
        execute!(stdout(), MoveUp(2), MoveToColumn(0),)?;
    } else {
        ratatui::restore();
        execute!(stdout(), MoveDown(3), MoveToColumn(0),)?;
    }

    Ok(textarea.lines().join(""))
}
