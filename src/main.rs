mod colors;
mod config;
mod display;
mod embedding;
mod matcher;

use clap::{Arg, Command};
use colored::*;
use colors::get_color_data;
use config::Config;
use display::PaletteDisplay;
use matcher::MoodPaletteGenerator;
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    let matches = Command::new("colorify")
        .version("0.1.0")
        .about(
            "Generate color palettes based on mood descriptions using advanced semantic matching",
        )
        .author("Colorify CLI")
        .arg(
            Arg::new("mood")
                .help("The mood or scene to generate colors for")
                .value_name("MOOD")
                .index(1),
        )
        .arg(
            Arg::new("count")
                .short('n')
                .long("count")
                .help("Number of colors to generate")
                .value_name("COUNT")
                .default_value("5"),
        )
        .arg(
            Arg::new("interactive")
                .short('i')
                .long("interactive")
                .help("Run in interactive mode")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("bar")
                .short('b')
                .long("bar")
                .help("Display a compact color bar")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("no_names")
                .long("no-names")
                .help("Hide color names in bar mode")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("examples")
                .long("examples")
                .help("Show example mood descriptions")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    if matches.get_flag("examples") {
        PaletteDisplay::display_examples();
        return;
    }

    let count: usize = matches
        .get_one::<String>("count")
        .unwrap()
        .parse()
        .unwrap_or(5);

    let config = Config::new().with_top_k(count);
    let colors = get_color_data();

    // Initialize the generator with advanced semantic matching
    let mut generator = match MoodPaletteGenerator::new(config, colors).await {
        Ok(gen) => gen,
        Err(e) => {
            eprintln!("❌ Failed to initialize semantic matching system: {}", e);
            std::process::exit(1);
        }
    };

    if matches.get_flag("interactive") {
        run_interactive_mode(
            &mut generator,
            matches.get_flag("bar"),
            matches.get_flag("no_names"),
        )
        .await;
    } else if let Some(mood) = matches.get_one::<String>("mood") {
        generate_and_display(
            &mut generator,
            mood,
            matches.get_flag("bar"),
            matches.get_flag("no_names"),
        )
        .await;
    } else {
        PaletteDisplay::display_header();
        println!(
            "\n{}",
            "Please provide a mood description or use --interactive mode.".red()
        );
        println!("Usage: colorify \"cozy winter cabin\" or colorify --interactive");
        println!("Use --examples to see example mood descriptions.");
    }
}

async fn run_interactive_mode(
    generator: &mut MoodPaletteGenerator,
    show_bar: bool,
    no_names: bool,
) {
    PaletteDisplay::display_header();
    println!(
        "\n{}",
        "🎨 Interactive Mode - Enter mood descriptions (Ctrl+C to exit)"
            .bold()
            .green()
    );
    PaletteDisplay::display_examples();

    loop {
        print!("\n🎭 Enter mood: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let mood = input.trim();
                if mood.is_empty() {
                    println!("{}", "Please enter a mood description.".yellow());
                    continue;
                }

                if mood.to_lowercase() == "quit" || mood.to_lowercase() == "exit" {
                    println!("{}", "Goodbye! 🌈".cyan());
                    break;
                }

                generate_and_display(generator, mood, show_bar, no_names).await;
            }
            Err(_) => {
                println!("\n{}", "Goodbye! 🌈".cyan());
                break;
            }
        }
    }
}

async fn generate_and_display(
    generator: &mut MoodPaletteGenerator,
    mood: &str,
    show_bar: bool,
    no_names: bool,
) {
    println!(
        "\n{} {}",
        "🔍 Analyzing mood:".bright_blue(),
        mood.italic().white()
    );

    let palette = match generator.generate_palette(mood).await {
        Ok(palette) => palette,
        Err(e) => {
            println!("❌ Error generating palette: {}", e);
            return;
        }
    };

    if palette.is_empty() {
        println!(
            "{}",
            "No matching colors found. Try a different mood description.".red()
        );
        return;
    }

    if show_bar {
        PaletteDisplay::display_color_bar(&palette, no_names);
    } else {
        // Clear the terminal before displaying the new palette
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        PaletteDisplay::display_palette(&palette);
    }
}
