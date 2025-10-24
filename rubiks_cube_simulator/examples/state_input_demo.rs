use rubiks_cube_simulator::{StateInputEditor, State};
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Enable TUI mode
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create and run the state input editor
    let mut editor = StateInputEditor::new();
    let result = editor.run(&mut terminal);

    // Disable TUI mode
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    // Display the results
    match result {
        Ok(Some((cp, co, ep, eo))) => {
            println!("\n✅ State input completed successfully!");
            println!("CP (Corner Permutation): {:?}", cp);
            println!("CO (Corner Orientation): {:?}", co);
            println!("EP (Edge Permutation):   {:?}", ep);
            println!("EO (Edge Orientation):   {:?}", eo);
            
            // Create a State object from the input
            let state = State::from_arrays(cp, co, ep, eo);
            println!("\nCreated State:");
            println!("{}", state);
            
            if state.is_solved() {
                println!("\n🎉 The state is SOLVED!");
            } else {
                println!("\n🔀 The state is scrambled.");
            }
        }
        Ok(None) => {
            println!("\n❌ State input was cancelled.");
        }
        Err(e) => {
            println!("\n⚠️  Error occurred: {}", e);
        }
    }

    Ok(())
}
