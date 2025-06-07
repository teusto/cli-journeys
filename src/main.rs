use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{
    DefaultTerminal, Frame, 
    widgets::{Paragraph, Widget, Block, BorderType, List, ListItem},
    layout::{Constraint, Layout},
    style::{Color, Stylize}
};

#[derive(Debug)]
struct Journey {
    name: String,
    level: u8,
    description: String,
}

#[derive(Debug, Default)]
struct AppState {
    journeys: Vec<Journey>,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut state = AppState::default();

    state.journeys.push(Journey{
        name: "Journey 1".to_string(),
        level: 1,
        description: "Description 1".to_string(),
    });
    state.journeys.push(Journey{
        name: "Journey 2".to_string(),
        level: 2,
        description: "Description 2".to_string(),
    });
    state.journeys.push(Journey{
        name: "Journey 3".to_string(),
        level: 3,
        description: "Description 3".to_string(),
    });

    let terminal = ratatui::init();
    let result = run(terminal, &mut state);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, app_state: &mut AppState) -> Result<()> {
    loop {
        //Rendering the UI
        terminal.draw(|frame| render(frame, app_state))?;
        
        //Handling inputs
        if let Event::Key(key) = event::read()? {
            match key.code {
                event::KeyCode::Esc => break,
                _ => {}
            }   
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, app_state: &AppState) {
    let [border_area] = Layout::vertical([Constraint::Fill(1)]).margin(1).areas(frame.area());
    let [inner_area] = Layout::vertical([Constraint::Fill(1)]).margin(1).areas(border_area);

    Block::bordered().border_type(BorderType::Rounded).fg(Color::Yellow).render(border_area, frame.buffer_mut());
    List::new(app_state.journeys.iter().map(|journey| ListItem::from(journey.name.clone()))).render(inner_area, frame.buffer_mut());

    Paragraph::new("Hello from application").render(frame.area(), frame.buffer_mut());
}
