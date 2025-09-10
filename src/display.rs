use crate::matcher::ColorMatch;
use colored::*;

pub struct PaletteDisplay;

impl PaletteDisplay {
    pub fn display_palette(matches: &[ColorMatch]) {
        if matches.is_empty() {
            println!("{}", "No colors found for the given mood.".red());
            return;
        }

        println!("\n{}", "🎨 Your Mood Palette:".bold().cyan());
        println!("{}", "─".repeat(50).bright_black());

        for (i, color_match) in matches.iter().enumerate() {
            let color = &color_match.color;
            let score = color_match.score;

            // Create a color block using the actual hex color
            let color_block = Self::create_color_block(&color.hex);

            println!(
                "{} {} {} | {} | Score: {:.3}",
                format!("{}.", i + 1).bright_black(),
                color_block,
                color.name.bold(),
                color.hex.bright_black(),
                format!("{:.3}", score).bright_green()
            );

            // Display description with word wrapping
            let wrapped_description = Self::wrap_text(&color.description, 60);
            for line in wrapped_description {
                println!("   {}", line.italic().bright_black());
            }

            if i < matches.len() - 1 {
                println!();
            }
        }

        println!("{}", "─".repeat(50).bright_black());
    }

    pub fn display_color_bar(matches: &[ColorMatch], no_names: bool) {
        if matches.is_empty() {
            return;
        }

        print!("\n🌈 ");
        for color_match in matches {
            let color_block = Self::create_color_block(&color_match.color.hex);
            print!("{}", color_block);
        }
        println!(" Your palette");

        if !no_names {
            // Display scores below the color bar
            for color_match in matches.iter() {
                let color_block = Self::create_color_block(&color_match.color.hex);
                print!(
                    "{}",
                    format!(
                        "{} {} ({}) - {:.3}\n",
                        color_block,
                        color_match.color.name,
                        color_match.color.hex,
                        color_match.score
                    )
                    .bright_green()
                );
            }
        }
        println!();
    }

    fn create_color_block(hex: &str) -> ColoredString {
        // Convert hex to RGB for terminal colors
        if let Some((r, g, b)) = Self::hex_to_rgb(hex) {
            "██".truecolor(r, g, b)
        } else {
            "██".white()
        }
    }

    fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return None;
        }

        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

        Some((r, g, b))
    }

    fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut lines = Vec::new();
        let mut current_line = String::new();

        for word in words {
            if current_line.is_empty() {
                current_line = word.to_string();
            } else if current_line.len() + word.len() + 1 <= max_width {
                current_line.push(' ');
                current_line.push_str(word);
            } else {
                lines.push(current_line);
                current_line = word.to_string();
            }
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        lines
    }

    pub fn display_header() {
        println!(
            "{}",
            "╔══════════════════════════════════════════════════╗".cyan()
        );
        println!(
            "{}",
            "║                   🎨 COLORIFY 🎨                 ║".cyan()
        );
        println!(
            "{}",
            "║            Mood to Color Palette CLI             ║".cyan()
        );
        println!(
            "{}",
            "╚══════════════════════════════════════════════════╝".cyan()
        );
    }

    pub fn display_examples() {
        println!("\n{}", "💡 Try these example moods:".bold().yellow());
        let examples = [
            "cozy cabin in winter",
            "tropical beach sunset",
            "cyberpunk city at night",
            "peaceful forest morning",
            "vintage romance",
            "energetic summer festival",
        ];

        for example in &examples {
            println!("   • {}", example.italic());
        }
    }
}
