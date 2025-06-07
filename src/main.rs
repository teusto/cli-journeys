use color_eyre::Result;
use crossterm::event::{self, Event, KeyEvent};
use ratatui::{
    DefaultTerminal, Frame, 
    widgets::{Paragraph, Widget, Block, BorderType, List, ListItem, ListState, Padding, Borders, HighlightSpacing},
    layout::{Constraint, Direction, Layout},
    style::{Color, Stylize, Style}
};

#[derive(Debug)]
struct Journey {
    name: String,
    level: u8,
    description: String,
}

#[derive(Debug, Default)]
struct AppState {
    journeys: Vec<Journey>, // The list of journeys
    list_state: ListState, // The state of the list of journeys
    selected_journey: Option<usize>, // The selected journey passed to the journey context
    current_screen: CurrentScreen, // The current screen of the application
}

#[derive(Debug, Default)]
enum CurrentScreen {
    #[default]
    Journeys,
    JourneyContext,
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
            if handle_key(key, app_state){
                break;
            }
        }
    }
    Ok(())
}

fn handle_key(key: KeyEvent, app_state: &mut AppState) -> bool{
    match key.code {
        event::KeyCode::Esc => return true,
        event::KeyCode::Char(char) => {
            match char {
                'j' => {
                    app_state.list_state.select_next();
                },
                'k' => {
                    app_state.list_state.select_previous();
                },
                'p' => {
                    if let Some(selected) = app_state.list_state.selected() {
                        app_state.selected_journey = Some(selected);
                    }
                },
                _ => {}
            }
        },
        _ => {}
    }   
    false
}

fn render(frame: &mut Frame, app_state: &mut AppState) {
    let chunks = Layout::default().direction(Direction::Horizontal).constraints([
        Constraint::Percentage(20),
        Constraint::Percentage(80),
    ]).split(frame.area());

    let journeys = Block::default().title("Journeys").borders(Borders::ALL).border_type(BorderType::Rounded).style(Style::default().fg(Color::Green));
    let right_area = chunks[1];

    let journey_menu = Block::default().title("Journey Menu").borders(Borders::ALL).border_type(BorderType::Rounded);
    let journey_running = Block::default().title("Running Journey").borders(Borders::ALL).border_type(BorderType::Rounded);

    let list = List::new(app_state.journeys.iter().map(|journey| ListItem::from(journey.name.clone()))).block(journeys).highlight_symbol(">> ").highlight_spacing(HighlightSpacing::Always).highlight_style(Style::default().fg(Color::Yellow));
    
    frame.render_stateful_widget(list, chunks[0], &mut app_state.list_state);

    let journey_menu_chunks = Layout::vertical([
        Constraint::Percentage(25),
        Constraint::Percentage(75),
    ]).split(right_area);
    
    frame.render_widget(journey_menu, journey_menu_chunks[0]);
    frame.render_widget(journey_running, journey_menu_chunks[1]);
    
    // let journeys = Block::new().borders(Borders::TOP).title("Journeys").padding(Padding::uniform(1)).borders(Borders::ALL).border_type(BorderType::Rounded).fg(Color::Yellow);
    // let journey_menu = Block::new().borders(Borders::TOP).title("Journey Menu").padding(Padding::uniform(1)).borders(Borders::ALL).border_type(BorderType::Rounded).fg(Color::Green);
    // let running_journey = Block::new().borders(Borders::TOP).title("Running Journey").padding(Padding::uniform(1)).borders(Borders::ALL).border_type(BorderType::Rounded).fg(Color::Green);

    // let list = List::new(app_state.journeys.iter().map(|journey| ListItem::from(journey.name.clone())))
    //     .block(journeys)
    //     .highlight_symbol(">> ")
    //     .highlight_style(Style::default().fg(Color::Green));

    // let selected_journey = Paragraph::new(
    //         app_state.selected_journey.map(
    //             |journey| app_state.journeys[journey].description.clone()
    //         )
    //         .unwrap_or("No journey selected".to_string()))
    //         .block(journey_menu);

    // frame.render_stateful_widget(list, left, &mut app_state.list_state);
}
