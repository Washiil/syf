mod app;
mod components;
mod keyboard_mapping;

use std::{error::Error, io};
use std::rc::Rc;
use app::App;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::prelude::SliceRandom;
use ratatui::{prelude::*, widgets::*};

use crate::components::prompt_box::prompt_widget;
// use crate::components::keyboard::keyboard_widget;
use crate::components::centered_rect::centered_rect;

enum InputMode {
    Normal,
    Editing,
}

fn generate_prompts() -> Vec<String> {
    // Read from the words.txt file and randomly select 2000 words
    let words = include_str!("../words.txt");
    let mut words: Vec<&str> = words.lines().collect();
    let mut rng = rand::thread_rng();
    // Generate 2000 prompts
    let mut prompts = Vec::new();
    for _ in 0..2000 {
        prompts.push(words.choose(&mut rng).unwrap().to_string());
    }
    prompts
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::default();
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('e') => {
                        app.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    KeyCode::Char('p') => {
                        app.keyboard_type = match app.keyboard_type {
                            keyboard_mapping::KeyboardType::Dvorak => keyboard_mapping::KeyboardType::Qwerty,
                            keyboard_mapping::KeyboardType::Qwerty => keyboard_mapping::KeyboardType::Azerty,
                            keyboard_mapping::KeyboardType::Azerty => keyboard_mapping::KeyboardType::Colemark,
                            keyboard_mapping::KeyboardType::Colemark => keyboard_mapping::KeyboardType::Dvorak,
                        };
                    }
                    _ => {}
                },
                InputMode::Editing if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => app.submit_message(),
                    KeyCode::Char(to_insert) => {
                        if to_insert == ' ' {
                            app.submit_message();
                        }
                        else {
                            app.enter_char(app.keyboard_type.map_key(to_insert));
                        }
                    }
                    KeyCode::Backspace => {
                        app.delete_char();
                    }
                    KeyCode::Left => {
                        app.move_cursor_left();
                    }
                    KeyCode::Right => {
                        app.move_cursor_right();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}

fn layout(f: &Frame) -> Rc<[Rect]> {
    let popup_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ])
        .split(f.size());

    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Length(6),
            Constraint::Length(1),
            Constraint::Percentage(25),
        ])
        .split(popup_layout[1])
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = layout(f);

    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                format!("Viewing: Current layout: {}", app.keyboard_type).into(),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                format!("Editing: Current layout: {}", app.keyboard_type).into(),
            ],
            Style::default(),
        ),
    };

    let mut text = Text::from(Line::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let prompt = prompt_widget(app);
    f.render_widget(prompt, chunks[1]);

    // let input = Paragraph::new(app.input.as_str())
    //     .style(match app.input_mode {
    //         InputMode::Normal => Style::default(),
    //         InputMode::Editing => Style::default().fg(Color::Yellow),
    //     })
    //     .block(Block::default().borders(Borders::ALL));
    // f.render_widget(input, chunks[3]);
    // let k = keyboard_widget(app);
    // f.render_widget(k, chunks[3]);

    f.render_widget(Block::default().borders(Borders::all()).title(format!("{}", &app.keyboard_type)), centered_rect(chunks[3], 80, 90));

    match app.input_mode {
        InputMode::Normal =>
        // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        InputMode::Editing => {
            // Make the cursor visible and ask ratatui to put it at the specified coordinates after
            // rendering
            f.set_cursor(
                // Draw the cursor at the current position in the input field.
                // This position is can be controlled via the left and right arrow key
                chunks[1].x + app.cursor_position as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[1].y + 1,
            )
        }
    }
}