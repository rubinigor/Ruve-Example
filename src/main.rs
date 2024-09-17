use ruve::{run_app, WindowConfig};
use std::fs::read_to_string;
use std::error::Error;



fn run() -> Result<(), Box<dyn Error>> {
    let title = "Ruve Example".to_string();
    let width = 910.0;
    let height = 540.0;

    let html_content = read_to_string("index.ruve")?;
    
    let config = WindowConfig { 
        title, 
        width, 
        height, 
        icon_path: Some("logo.png".to_string()),
    };

    run_app(config, &html_content)?;

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
