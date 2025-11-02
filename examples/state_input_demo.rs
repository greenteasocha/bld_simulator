use rubiks_cube_simulator::{State, StateInputEditor};
use rubiks_cube_simulator::workflow::CombinedNearbySearchWorkflow;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};
use std::{fs, io};

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
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    // Process the results
    match result {
        Ok(Some((scramble, cp, co, ep, eo))) => {
            println!("\n✅ Input completed successfully!\n");

            // Check if scramble is provided
            if scramble.trim().is_empty() {
                println!("⚠️  No scramble provided. Exiting without search.");
                return Ok(());
            }

            // Create target state
            let target_state = State::from_arrays(cp, co, ep, eo);

            // Load JSON resources
            let ufr_expanded = fs::read_to_string("resources/ufr_expanded.json")?;
            let ufr_parity = fs::read_to_string("resources/ufr_parity.json")?;
            let ufr_twist = fs::read_to_string("resources/ufr_twist.json")?;
            let uf_expanded = fs::read_to_string("resources/uf_expanded.json")?;
            let uf_flip = fs::read_to_string("resources/uf_flip.json")?;

            // Create workflow
            let workflow = match CombinedNearbySearchWorkflow::from_json(
                &ufr_expanded,
                &ufr_parity,
                &ufr_twist,
                &uf_expanded,
                &uf_flip,
            ) {
                Ok(w) => w,
                Err(e) => {
                    eprintln!("✗ Failed to create workflow: {}", e);
                    return Ok(());
                }
            };

            // Run search
            match workflow.search_from_scramble(&scramble, &target_state) {
                Ok(result) => {
                    // Display detailed results using display_detailed
                    println!("{}", result.display_detailed(5));
                }
                Err(e) => {
                    eprintln!("✗ Search failed: {}", e);
                }
            }
        }
        Ok(None) => {
            println!("\n❌ Input was cancelled.");
        }
        Err(e) => {
            eprintln!("\n⚠️  Error occurred: {}", e);
        }
    }

    Ok(())
}
