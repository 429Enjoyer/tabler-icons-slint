mod generate;

fn main() {
    let command = std::env::args().nth(1);
    match command.as_deref() {
        Some("generate") => {
            if let Err(error) = generate::run() {
                eprintln!("Failed to generate Tabler Slint icons: {error}");
                std::process::exit(1);
            }
        }
        _ => {
            eprintln!("Usage: cargo run -p builder -- generate");
            std::process::exit(2);
        }
    }
}
