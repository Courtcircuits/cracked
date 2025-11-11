use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;

mod app;
mod challenge;
mod cli;
mod errors;
mod event;
mod search;
mod ui;

use app::App;
use clap::Parser;
use cli::Cli;
use search::{get_challenge_list, get_search_token};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse CLI arguments
    let cli = Cli::parse();

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Get tokens
    let (cookie_token, html_token) = match get_search_token().await {
        Ok(tokens) => tokens,
        Err(_) => {
            cleanup_terminal(&mut terminal)?;
            eprintln!("Failed to get search tokens");
            return Ok(());
        }
    };

    // Create app
    let mut app = App::new();

    // Load initial challenges with CLI parameters
    let params = cli.to_search_params(html_token.clone());

    // Debug: print search params

    match get_challenge_list(params, cookie_token.clone()).await {
        Ok(challenges) => {
            if challenges.is_empty() {
                app.set_status(String::from("No challenges found matching criteria"));
            } else {
                app.set_challenges(challenges);
            }
        }
        Err(e) => {
            cleanup_terminal(&mut terminal)?;
            eprintln!("Failed to load challenges: {:?}", e);
            return Ok(());
        }
    }

    // Main loop
    loop {
        terminal.draw(|f| ui::render(f, &app))?;

        event::handle_events(&mut app)?;

        // Handle download request
        if app.should_download {
            if let Some(challenge) = app.get_selected_challenge() {
                // app.set_status(format!("Downloading {}...", challenge.name));
                terminal.draw(|f| ui::render(f, &app))?;

                match challenge.download().await {
                    Ok(_) => {
                        app.set_status(format!("Successfully downloaded {}.zip", challenge.name));
                    }
                    Err(_) => {
                        app.set_status(format!("Failed to download {}", challenge.name));
                    }
                }
            }
            app.reset_download_flag();
        }

        if app.should_quit {
            break;
        }
    }

    // Cleanup
    cleanup_terminal(&mut terminal)?;
    Ok(())
}

fn cleanup_terminal(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
