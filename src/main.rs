mod journeys;

use color_eyre::Result;
use crossterm::event::{self, Event, KeyEvent};
use ratatui::{
    DefaultTerminal, Frame, 
    widgets::{Paragraph, Widget, Block, BorderType, List, ListItem, ListState, Wrap, Padding, Borders, HighlightSpacing},
    layout::{Constraint, Direction, Layout},
    style::{Color, Stylize, Style}
};
use journeys::helpers::{run_journey, JourneyOutput};


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
    journey_output: JourneyOutput,
}

#[derive(Debug, Default)]
enum CurrentScreen {
    #[default]
    Journeys,
    JourneyContext,
    JourneyRunning,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut state = AppState::default();

    state.journeys.push(Journey{
        name: "Keypair Generation".to_string(),
        level: 1,
        description: "Implement secure cryptographic key generation".to_string(),
    });
    state.journeys.push(Journey{
        name: "Mnemonics Generation".to_string(),
        level: 1,
        description: "Implement mnemonic phrase backup and checksum validation.".to_string(),
    });
    state.journeys.push(Journey{
        name: "Account Rent Calculator".to_string(),
        level: 1,
        description: "Develop tool for calculating minimum balance requirements across account types.".to_string(),
    });
    state.journeys.push(Journey{
        name: "Data Serialization".to_string(),
        level: 1,
        description: "Practice Borsh serialization/deserialization with custom data structures.".to_string(),
    });
    state.journeys.push(Journey{
        name: "Balance Checker".to_string(),
        level: 1,
        description: "Command-line tool for querying account balances across networks.".to_string(),
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
     // Common key handling for all screens
     match key.code {
        event::KeyCode::Esc => return true, // Exit the app
        event::KeyCode::Char('q') => {
            // Go back to previous screen (default to Journeys)
            app_state.current_screen = CurrentScreen::Journeys;
            return false;
        },
        _ => {}
    }

    // Screen-specific key handling
    match app_state.current_screen {
        CurrentScreen::Journeys => {
            // Journey list navigation and selection
            if let event::KeyCode::Char(char) = key.code {
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
                            app_state.current_screen = CurrentScreen::JourneyContext;
                        }
                    },
                    _ => {}
                }
            }
        },
        CurrentScreen::JourneyContext => {
            // Journey context screen navigation
            if let event::KeyCode::Char(char) = key.code {
                match char {
                    'r' => {
                        // Move to journey running screen
                        // Based on the journey selected call one of the helpers functions
                        if let Some(idx) = app_state.selected_journey {
                            let name = &app_state.journeys[idx].name;
                            app_state.journey_output = run_journey(name);
                        }
                        app_state.current_screen = CurrentScreen::JourneyRunning;
                        
                    },
                    'j' => {
                        // Different behavior for 'j' in this screen
                        // For example, navigate through journey details
                    },
                    'k' => {
                        // Different behavior for 'k' in this screen
                    },
                    'm' => {
                        // 
                    }
                    _ => {}
                }
            }
        },
        CurrentScreen::JourneyRunning => {
            // Journey running screen navigation
            if let event::KeyCode::Char(char) = key.code {
                match char {
                    'b' => {
                        // Example: stop the running journey
                        app_state.current_screen = CurrentScreen::JourneyContext;
                    },
                    'p' => {
                        // Example: pause/play the journey
                    },
                    _ => {}
                }
            }
        }
    }
    false
}

fn get_style_for_screen(current_screen: &CurrentScreen, target_screen: &CurrentScreen) -> Style {
    if std::mem::discriminant(current_screen) == std::mem::discriminant(target_screen) {
        Style::default().fg(Color::Green)
    } else {
        Style::default().fg(Color::White)
    }
}

fn render(frame: &mut Frame, app_state: &mut AppState) {
    let chunks = Layout::default().direction(Direction::Horizontal).constraints([
        Constraint::Percentage(30),
        Constraint::Percentage(70),
    ]).split(frame.area());
    

    let journey_style = get_style_for_screen(&app_state.current_screen, &CurrentScreen::Journeys);
    let journeys = Block::default().title("Journeys").borders(Borders::ALL).border_type(BorderType::Rounded).style(journey_style);
    let right_area = chunks[1];

    let journey_context_style = get_style_for_screen(&app_state.current_screen, &CurrentScreen::JourneyContext);
    let journey_menu = Block::default().title("Journey Menu").borders(Borders::ALL).border_type(BorderType::Rounded).style(journey_context_style);

    let journey_running_style = get_style_for_screen(&app_state.current_screen, &CurrentScreen::JourneyRunning);
    let journey_running = Block::default().title("Running Journey").borders(Borders::ALL).border_type(BorderType::Rounded).style(journey_running_style);

    let list = List::new(app_state.journeys.iter().map(|journey| ListItem::from(journey.name.clone()))).block(journeys).highlight_symbol(">> ").highlight_spacing(HighlightSpacing::Always).highlight_style(Style::default().fg(Color::Yellow));
    
    frame.render_stateful_widget(list, chunks[0], &mut app_state.list_state);

    let journey_menu_chunks = Layout::vertical([
        Constraint::Percentage(25),
        Constraint::Percentage(75),
    ]).split(right_area);

    match app_state.current_screen {
        CurrentScreen::JourneyContext => {
            let menu_block = Block::default().title("Journey Menu").borders(Borders::ALL);
            let journey_description = Paragraph::new(app_state.selected_journey.map(|journey| app_state.journeys[journey].description.clone()).unwrap_or("No journey selected".to_string())).block(menu_block).wrap(Wrap { trim: true });
            frame.render_widget(journey_description, journey_menu_chunks[0]);
        },
        CurrentScreen::JourneyRunning => {
            let running_block = Block::default().borders(Borders::ALL);
            let content = match &app_state.journey_output {
                JourneyOutput::Keypair(keypair) => {
                    format!("Generated Keypair\nPublic Key: {:?}", keypair)
                },
                JourneyOutput::Mnemonic(phrase) => {
                    format!("Mnemonic Phrase\n\n{}\n\nPress 'b' to go back", phrase)
                },
                JourneyOutput::BalanceChecker(balances) => {
                    format!("Account Balance Checker\n\n{}\n\nPress 'b' to go back", balances)
                },
                JourneyOutput::None => {
                    "No output yet.".into()
                }
            };

            let paragraph = Paragraph::new(content).block(running_block).wrap(Wrap {trim: true});

            frame.render_widget(paragraph, journey_menu_chunks[1]);
        },
        _ => {}
    }
    
   

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
