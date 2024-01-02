use ratatui::{prelude::*, widgets::*};
use crate::app::App;
use crate::InputMode;

pub fn prompt_widget(app: &App) -> Paragraph {
    let mut fmt_prompt: Vec<Span> = Vec::new();

    let temp_prompt: Vec<char> = app.prompts[0].clone().chars().collect();
    let input: Vec<char> = app.input.clone().chars().collect();

    for i in 0..temp_prompt.len() {
        if let Some(p) = input.get(i) {
            if p == temp_prompt.get(i).unwrap() {
                fmt_prompt.push(Span::styled(p.to_string(), Style::default().fg(Color::Green)));
            }
            else {
                fmt_prompt.push(Span::styled(p.to_string(), Style::default().fg(Color::Red)));
            }
        }
        else
        {
            fmt_prompt.push(Span::styled(temp_prompt.get(i).unwrap().to_string(), Style::default().fg(Color::White)));
        }
    }

    for i in 1..20 {
        if let Some(p) = app.prompts.get(i) {
            fmt_prompt.push(Span::styled(" ".to_string() + p, Style::default().fg(Color::Gray)));
        }
    }

    let prompt = Paragraph::new(Line::from(fmt_prompt))
        .wrap(Wrap { trim: true })
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default(),
        })
        .block(Block::default().borders(Borders::ALL));

    return prompt;
}

