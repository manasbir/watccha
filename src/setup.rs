use toml::{to_string};
use ethers::types::{H160, Transaction};
use std::{io, thread, time::Duration, fs::File};
use std::io::Write;
use tui::{
    backend::CrosstermBackend,
    widgets::{Widget, Block, Borders},
    layout::{Layout, Constraint, Direction},
    Terminal
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use eyre::{Result, ErrReport};
use serde::{Serialize, Deserialize};
pub mod events;

#[derive(Debug, Serialize)]

struct Config {
    general: General,
    email: Email,
    events: Vec<Listener>,
}
#[derive(Debug, Serialize)]

struct General {
    rpc_url: String,
    p_key: String,
    etherscan_key: String,
}
#[derive(Debug, Serialize)]

struct Email {
    email: String,
    app_email: String,
    app_password: String,
}
#[derive(Debug, Serialize)]

struct Listener {
    function: String,
    email: bool,
    address: H160,
}
fn main() -> Result<(), io::Error> {

    let config = Config {
        general: General {
            rpc_url: "https://goerli.infura.io/v3/".to_string(),
            p_key: "0x".to_string(),
            etherscan_key: "E_KEY".to_string(),
        },
        email: Email {
            email: "email".to_string(),
            app_email: "app_email".to_string(),
            app_password: "app_password".to_string(),
        },
        events: vec![
            Listener {
                function: "from".to_string(),
                email: true,
                address: "0x7b5C526B7F8dfdff278b4a3e045083FBA4028790".parse().unwrap(),
            },
            Listener {
                function: "to".to_string(),
                email: true,
                address: "0x7b5C526B7F8dfdff278b4a3e045083FBA4028790".parse().unwrap(),
            },
        ],
    };

    let config = toml::to_string(&config).unwrap();
    let mut file = File::create("config.toml").unwrap();
    file.write_all(config.as_bytes()).unwrap();


    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()
            .title("Watcca")
            .borders(Borders::ALL);
        f.render_widget(block, size);
    })?;

    // thread::sleep(Duration::from_millis(5000));

    // restore terminal
    // disable_raw_mode()?;
    Ok(())
}