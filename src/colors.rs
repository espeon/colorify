use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Color {
    pub name: String,
    pub hex: String,
    pub description: String,
}

#[allow(dead_code)]
impl Color {
    pub fn new(name: &str, hex: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            hex: hex.to_string(),
            description: description.to_string(),
        }
    }

    pub fn text_description(&self) -> String {
        format!("{}, {}", self.name, self.description)
    }

    pub fn get_rgb(&self) -> Option<(u8, u8, u8)> {
        let hex = self.hex.trim_start_matches('#');
        if hex.len() != 6 {
            return None;
        }

        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

        Some((r, g, b))
    }

    pub fn get_text_color(&self) -> &'static str {
        if let Some((r, g, b)) = self.get_rgb() {
            let luminance = 0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32;
            if luminance > 150.0 {
                "black"
            } else {
                "white"
            }
        } else {
            "white"
        }
    }
}

pub fn get_color_data() -> Vec<Color> {
    vec![
        // --- Core Colors ---
        Color::new(
            "Crimson",
            "#DC143C",
            "A deep, rich red color, leaning slightly towards purple.",
        ),
        Color::new(
            "Scarlet",
            "#FF2400",
            "A brilliant, vivid red with a hint of orange.",
        ),
        Color::new(
            "Coral",
            "#FF7F50",
            "A vibrant pinkish-orange reminiscent of marine invertebrates.",
        ),
        Color::new(
            "Tangerine",
            "#F28500",
            "A saturated, zesty orange, like the ripe citrus fruit.",
        ),
        Color::new(
            "Gold",
            "#FFD700",
            "A bright, metallic yellow associated with wealth and luxury.",
        ),
        Color::new(
            "Lemon Chiffon",
            "#FFFACD",
            "A pale, light yellow, as soft and airy as the dessert.",
        ),
        Color::new(
            "Lime Green",
            "#32CD32",
            "A bright green color, evoking freshness and zesty energy.",
        ),
        Color::new(
            "Forest Green",
            "#228B22",
            "A dark, shaded green, like the canopy of a dense forest.",
        ),
        Color::new(
            "Teal",
            "#008080",
            "A medium blue-green color, often seen as sophisticated and calming.",
        ),
        Color::new(
            "Cyan",
            "#00FFFF",
            "A vibrant greenish-blue, one of the primary subtractive colors.",
        ),
        Color::new(
            "Sky Blue",
            "#87CEEB",
            "A light, pale blue, like the color of a clear daytime sky.",
        ),
        Color::new(
            "Royal Blue",
            "#4169E1",
            "A deep, vivid blue that is both rich and bright.",
        ),
        Color::new(
            "Indigo",
            "#4B0082",
            "A deep, rich color between blue and violet in the spectrum.",
        ),
        Color::new(
            "Lavender",
            "#E6E6FA",
            "A light, pale purple with a bluish hue, named after the flower.",
        ),
        Color::new(
            "Plum",
            "#DDA0DD",
            "A reddish-purple color, like the ripe fruit it's named after.",
        ),
        Color::new(
            "Magenta",
            "#FF00FF",
            "A purplish-red color that lies between red and violet.",
        ),
        Color::new(
            "Hot Pink",
            "#FF69B4",
            "A bright, vivid pink that is both bold and energetic.",
        ),
        Color::new(
            "Ivory",
            "#FFFFF0",
            "An off-white color that resembles the material from tusks and teeth.",
        ),
        Color::new(
            "Beige",
            "#F5F5DC",
            "A pale sandy fawn color, often used as a warm, neutral tone.",
        ),
        Color::new(
            "Taupe",
            "#483C32",
            "A dark grayish-brown or brownish-gray color.",
        ),
        Color::new(
            "Slate Gray",
            "#708090",
            "A medium gray with a slight blue tinge, like the metamorphic rock.",
        ),
        Color::new(
            "Charcoal",
            "#36454F",
            "A dark, almost black gray, like burnt wood.",
        ),
        Color::new(
            "Onyx",
            "#353839",
            "A deep, rich black, often with a subtle hint of dark blue.",
        ),
        Color::new(
            "Emerald",
            "#50C878",
            "A brilliant green, named after the precious gemstone.",
        ),
        Color::new(
            "Sapphire",
            "#0F52BA",
            "A deep, lustrous blue, reminiscent of the valuable gemstone.",
        ),
        Color::new(
            "Ruby",
            "#E0115F",
            "A deep red color, inspired by the gemstone of the same name.",
        ),
        Color::new(
            "Amethyst",
            "#9966CC",
            "A moderate, violet-purple color, like the quartz gemstone.",
        ),
        Color::new(
            "Peridot",
            "#E6E200",
            "A light olive-green or yellowish-green, named for the gem.",
        ),
        Color::new(
            "Turquoise",
            "#40E0D0",
            "A greenish-blue color, often associated with tropical waters.",
        ),
        Color::new(
            "Silver",
            "#C0C0C0",
            "A metallic gray color that resembles polished silver.",
        ),
        Color::new(
            "Bronze",
            "#CD7F32",
            "A metallic brown color that resembles the alloy of copper and tin.",
        ),
        Color::new(
            "Obsidian",
            "#000000",
            "A pure, deep black, like the volcanic glass formed from cooled lava.",
        ),
        // --- Extended Red/Pink Palettes ---
        Color::new("Light Salmon", "#FFA07A", "A soft, pale orange‑pink."),
        Color::new("Dark Salmon", "#E9967A", "A muted, dusty salmon."),
        Color::new("Salmon", "#FA8072", "A warm, pinkish‑orange."),
        Color::new("Light Coral", "#F08080", "A gentle coral‑pink tone."),
        Color::new("Indian Red", "#CD5C5C", "A subdued, earthy red."),
        Color::new(
            "Fire Brick",
            "#B22222",
            "A rich, dark red, like heated clay.",
        ),
        Color::new("Deep Pink", "#FF1493", "A vivid, strong pink."),
        Color::new("Hot Pink", "#FF69B4", "A bold, energetic pink."),
        Color::new("Medium Violet Red", "#C71585", "A rich, deep pink-purple."),
        Color::new("Pale Violet Red", "#DB7093", "A soft, muted violet-red."),
        Color::new("Rosy Brown", "#BC8F8F", "A warm, muted brownish-pink."),
        Color::new("Misty Rose", "#FFE4E1", "A very pale, delicate rose pink."),
        // --- Extended Orange/Yellow Palettes ---
        Color::new(
            "Tomato",
            "#FF6347",
            "A vibrant red‑orange like ripe tomatoes.",
        ),
        Color::new("Orange Red", "#FF4500", "A fiery mix of orange and red."),
        Color::new("Dark Orange", "#FF8C00", "A saturated, strong orange."),
        Color::new("Light Yellow", "#FFFFE0", "A pale, soft yellow."),
        Color::new(
            "Light Goldenrod Yellow",
            "#FAFAD2",
            "A delicate, warm yellow.",
        ),
        Color::new("Papaya Whip", "#FFEFD5", "A creamy, soft pastel yellow."),
        Color::new("Peach Puff", "#FFDAB9", "A warm peach‑cream tone."),
        Color::new("Dark Khaki", "#BDB76B", "A muted, earthy greenish-yellow."),
        Color::new("Goldenrod", "#DAA520", "A deep, rich yellow-orange."),
        Color::new(
            "Peru",
            "#CD853F",
            "A reddish-brown, like the country's earth.",
        ),
        Color::new("Sandy Brown", "#F4A460", "A light, warm sandy brown."),
        // --- Extended Green/Cyan Palettes ---
        Color::new("Pale Green", "#98FB98", "A soft, light green."),
        Color::new(
            "Medium Spring Green",
            "#00FA9A",
            "A fresh, spring‑like green.",
        ),
        Color::new("Dark Sea Green", "#8FBC8F", "A muted, natural green."),
        Color::new("Aquamarine", "#7FFFD4", "A bright, blue‑green water tone."),
        Color::new("Azure", "#F0FFFF", "A very pale, almost white cyan."),
        Color::new("Light Cyan", "#E0FFFF", "A pale and refreshing cyan."),
        Color::new(
            "Medium Aquamarine",
            "#66CDAA",
            "A balanced medium blue-green.",
        ),
        Color::new("Dark Olive Green", "#556B2F", "A deep, muted olive green."),
        Color::new("Olive Drab", "#6B8E23", "A dull, yellowish-green."),
        Color::new(
            "Spring Green",
            "#00FF7F",
            "A bright, pure green, like new leaves.",
        ),
        Color::new("Sea Green", "#2E8B57", "A dark, natural sea-toned green."),
        Color::new(
            "Medium Sea Green",
            "#3CB371",
            "A vibrant, yet deep sea green.",
        ),
        // --- Extended Blue Palettes ---
        Color::new("Dodger Blue", "#1E90FF", "A strong, cheerful blue."),
        Color::new("Deep Sky Blue", "#00BFFF", "An intense, vibrant sky blue."),
        Color::new("Light Sky Blue", "#87CEFA", "A gentle, airy blue."),
        Color::new(
            "Cornflower Blue",
            "#6495ED",
            "A soft, periwinkle‑leaning blue.",
        ),
        Color::new(
            "Slate Blue",
            "#6A5ACD",
            "A medium, slightly muted blue‑purple.",
        ),
        Color::new(
            "Dark Slate Blue",
            "#483D8B",
            "A deep blue with purple hints.",
        ),
        Color::new(
            "Midnight Blue",
            "#191970",
            "A very dark, almost black blue.",
        ),
        Color::new("Steel Blue", "#4682B4", "A muted, grayish-blue."),
        Color::new(
            "Cadet Blue",
            "#5F9EA0",
            "A grayish-blue, often military-inspired.",
        ),
        Color::new("Powder Blue", "#B0E0E6", "A light, soft, dusty blue."),
        Color::new("Light Steel Blue", "#B0C4DE", "A pale, silvery blue."),
        Color::new("Deep Blue", "#00008B", "A very dark, rich blue."),
        Color::new("Medium Blue", "#0000CD", "A classic, vibrant blue."),
        // --- Extended Purple Palettes ---
        Color::new("Thistle", "#D8BFD8", "A faded, delicate purple."),
        Color::new("Violet", "#EE82EE", "A light, playful purple."),
        Color::new("Medium Orchid", "#BA55D3", "A warm, medium purple."),
        Color::new(
            "Blue Violet",
            "#8A2BE2",
            "A bright, vivid purple with blue side.",
        ),
        Color::new("Dark Violet", "#9400D3", "A strong, dark purple."),
        Color::new(
            "Rebecca Purple",
            "#663399",
            "A deep, muted purple (in tribute).",
        ),
        Color::new("Dark Orchid", "#9932CC", "A rich, deep orchid purple."),
        Color::new("Medium Purple", "#9370DB", "A balanced, vibrant purple."),
        Color::new(
            "Ghost White",
            "#F8F8FF",
            "A very pale, almost translucent white.",
        ),
        Color::new(
            "Medium Slate Blue",
            "#7B68EE",
            "A vibrant, yet soft blue-purple.",
        ),
        Color::new("Dark Magenta", "#8B008B", "A very deep, dark magenta."),
        // --- Extended Brown/Neutral/Grey Palettes ---
        Color::new("Antique White", "#FAEBD7", "A soft, aged‑cream off‑white."),
        Color::new("Bisque", "#FFE4C4", "A pale, warm beige."),
        Color::new("Moccasin", "#FFE4B5", "A gentle, tan‑cream."),
        Color::new("Burly Wood", "#DEB887", "A light, warm brown."),
        Color::new("Saddle Brown", "#8B4513", "A dark, rich earthy brown."),
        Color::new("Dark Goldenrod", "#B8860B", "A deep, earthy golden-brown."),
        Color::new("Chocolate", "#D2691E", "A rich, dark brown color."),
        Color::new("Brown", "#A52A2A", "A classic, medium brown."),
        Color::new("Sienna", "#A0522D", "A reddish-brown, like baked earth."),
        Color::new(
            "Linen",
            "#FAF0E6",
            "A soft, off-white with a slight beige tint.",
        ),
        Color::new("Old Lace", "#FDF5E6", "A very pale, creamy white."),
        Color::new("Seashell", "#FFF5EE", "A delicate, pinkish-white."),
        Color::new("Gainsboro", "#DCDCDC", "A light, silvery gray."),
        Color::new("Light Gray", "#D3D3D3", "A simple, soft gray."),
        Color::new("Dark Gray", "#A9A9A9", "A deeper, more prominent gray."),
        Color::new("Dim Gray", "#696969", "A dark, subdued gray."),
        Color::new("Dark Slate Gray", "#2F4F4F", "A very dark, greenish-gray."),
        Color::new("Black", "#000000", "The absence of light, pure black."),
        Color::new(
            "White",
            "#FFFFFF",
            "Pure white, the presence of all colors.",
        ),
        // --- New Red/Pink Variations ---
        Color::new("Dark Red", "#8B0000", "A very deep, intense red."),
        Color::new("Medium Red", "#CC0000", "A classic, balanced red."),
        Color::new(
            "Pale Red",
            "#FFC0CB",
            "A very light, delicate red, almost pink.",
        ),
        Color::new(
            "Tomato Red",
            "#FF6347",
            "A vibrant, warm red, like a ripe tomato.",
        ),
        Color::new(
            "Brick Red",
            "#B22222",
            "A strong, earthy red, reminiscent of brick.",
        ),
        Color::new(
            "Ruby Red",
            "#E0115F",
            "A deep, precious red, like the gemstone.",
        ),
        Color::new(
            "Rose Pink",
            "#FF66CC",
            "A soft, romantic pink with a hint of red.",
        ),
        Color::new(
            "Candy Apple Red",
            "#FF0800",
            "A bright, glossy, intense red.",
        ),
        Color::new(
            "Wine Red",
            "#722F37",
            "A rich, deep red with purplish-brown undertones.",
        ),
        Color::new("Rust", "#B7410E", "A reddish-brown, like oxidized metal."),
        // --- New Orange/Yellow Variations ---
        Color::new(
            "Dark Yellow",
            "#DAA520",
            "A deep, rich yellow, like goldenrod.",
        ),
        Color::new("Medium Yellow", "#FFD700", "A standard, bright yellow."),
        Color::new("Pale Yellow", "#FFFFAA", "A very light and soft yellow."),
        Color::new(
            "Mustard Yellow",
            "#FFDB58",
            "A dull, brownish-yellow, like mustard.",
        ),
        Color::new(
            "Golden Yellow",
            "#FFDF00",
            "A warm, bright yellow with a metallic sheen.",
        ),
        Color::new("Amber", "#FFBF00", "A warm, resinous yellow-orange."),
        Color::new("Apricot", "#FBCEB1", "A soft, peachy orange."),
        Color::new(
            "Mahogany",
            "#C04000",
            "A reddish-brown, like mahogany wood.",
        ),
        Color::new(
            "Pumpkin Orange",
            "#FF7518",
            "A bright, vivid orange, like a pumpkin.",
        ),
        Color::new(
            "Saffron",
            "#F4C430",
            "A vibrant, yellowish-orange spice color.",
        ),
        // --- New Green/Cyan Variations ---
        Color::new("Dark Green", "#006400", "A very deep, natural green."),
        Color::new("Medium Green", "#00B000", "A standard, vibrant green."),
        Color::new("Pale Green", "#98FB98", "A very light, soft green."),
        Color::new("Olive Green", "#6B8E23", "A dull, yellowish-green."),
        Color::new(
            "Hunter Green",
            "#355E3B",
            "A dark, rich green, like a hunter's cloak.",
        ),
        Color::new(
            "Mint Green",
            "#98FF98",
            "A light, refreshing, pastel green.",
        ),
        Color::new(
            "Kelly Green",
            "#4CBB17",
            "A strong, pure green, often associated with Ireland.",
        ),
        Color::new(
            "Dark Forest",
            "#014421",
            "An even deeper, almost black forest green.",
        ),
        Color::new("Electric Green", "#00FF00", "A very bright, neon green."),
        Color::new(
            "Aqua Green",
            "#00FF7F",
            "A vivid, clear green with blue undertones.",
        ),
        Color::new(
            "Ocean Green",
            "#4F8A8B",
            "A deep, calm blue-green, like ocean depths.",
        ),
        Color::new(
            "Tropical Blue",
            "#00FFCC",
            "A bright, clear, energetic blue-green.",
        ),
        Color::new("Deep Teal", "#005F6B", "A very dark, sophisticated teal."),
        // --- New Blue Variations ---
        Color::new("Dark Blue", "#00008B", "A very deep, rich blue."),
        Color::new("Medium Blue", "#0000CD", "A standard, vibrant blue."),
        Color::new("Pale Blue", "#ADD8E6", "A very light, soft blue."),
        Color::new(
            "Navy Blue",
            "#000080",
            "A very dark, almost black blue, like a naval uniform.",
        ),
        Color::new(
            "Carolina Blue",
            "#56A0D3",
            "A light, airy blue, often associated with universities.",
        ),
        Color::new(
            "Denim Blue",
            "#1560BD",
            "A medium-dark blue, like denim fabric.",
        ),
        Color::new(
            "Steel Blue Dark",
            "#2A52BE",
            "A darker, more intense steel blue.",
        ),
        Color::new(
            "Sky Blue Light",
            "#87CEEB",
            "A lighter, more pastel sky blue.",
        ),
        Color::new(
            "Air Force Blue",
            "#5D8AA8",
            "A medium blue with grayish undertones.",
        ),
        Color::new("Electric Blue", "#7DF9FF", "A brilliant, almost neon blue."),
        Color::new(
            "Cobalt Blue",
            "#0047AB",
            "A deep, intense blue with a hint of purple.",
        ),
        Color::new(
            "Iris",
            "#5A4FCF",
            "A rich, vibrant blue-purple, like the flower.",
        ),
        // --- New Purple Variations ---
        Color::new(
            "Dark Purple",
            "#301934",
            "A very deep, almost black purple.",
        ),
        Color::new("Medium Purple", "#9370DB", "A standard, balanced purple."),
        Color::new("Pale Purple", "#D8BFD8", "A very light, soft purple."),
        Color::new("Grape", "#6F2DA8", "A rich, deep purple, like ripe grapes."),
        Color::new(
            "Orchid Purple",
            "#DA70D6",
            "A soft, pinkish-purple, like an orchid.",
        ),
        Color::new("Fuchsia", "#FF00FF", "A vibrant, purplish-red."),
        Color::new("Mauve", "#E0B0FF", "A pale, grayish-purple."),
        Color::new("Plum Deep", "#660066", "A very deep, dark plum color."),
        Color::new(
            "Electric Purple",
            "#BF00FF",
            "A bright, vivid, almost neon purple.",
        ),
        Color::new("Royal Purple", "#7851A9", "A rich, regal purple."),
        // --- New Brown/Neutral/Grey Variations ---
        Color::new("Dark Brown", "#654321", "A deep, classic brown."),
        Color::new("Medium Brown", "#964B00", "A standard, earthy brown."),
        Color::new("Light Brown", "#C4A484", "A soft, pale brown."),
        Color::new(
            "Chestnut",
            "#954535",
            "A rich, reddish-brown, like a chestnut.",
        ),
        Color::new("Caramel", "#C68E17", "A warm, golden-brown."),
        Color::new("Khaki", "#C3B091", "A light, dusty brownish-yellow."),
        Color::new(
            "Vanilla",
            "#F3E5AB",
            "A creamy, soft off-white with yellow tones.",
        ),
        Color::new("Almond", "#EFDECD", "A warm, light brownish-white."),
        Color::new(
            "Ash Gray",
            "#B2BEB5",
            "A pale, cool gray with a hint of green.",
        ),
        Color::new("Smoke Gray", "#7C7C7C", "A medium, hazy gray."),
        Color::new("Lead Gray", "#71797E", "A dark, heavy gray."),
        Color::new("Jet Black", "#343434", "A very dark, almost pure black."),
        Color::new("Snow White", "#FFFAFA", "A very pure, slightly off-white."),
        Color::new("Old Gold", "#CFB53B", "A muted, dark gold."),
        Color::new("Pewter", "#899499", "A dull, metallic gray."),
        Color::new(
            "Carbon Fiber",
            "#656565",
            "A dark, patterned gray, like the material.",
        ),
        // --- More Red/Pink Variations ---
        Color::new(
            "Barn Red",
            "#7C0A02",
            "A deep, rustic red, like painted barns.",
        ),
        Color::new(
            "Cardinal Red",
            "#C41E3A",
            "A vivid, strong red, like the bird.",
        ),
        Color::new(
            "Cherry Red",
            "#DE3163",
            "A bright, juicy red, like ripe cherries.",
        ),
        Color::new(
            "Crimson Glory",
            "#B80000",
            "An even deeper, more majestic crimson.",
        ),
        Color::new("Folly", "#FF004F", "A lively, bold pink-red."),
        Color::new("Hollywood Cerise", "#F400A1", "A bright, glamorous pink."),
        Color::new(
            "Lipstick Red",
            "#C20B0B",
            "A classic, vibrant red, like lipstick.",
        ),
        Color::new("Maroon", "#800000", "A dark, brownish-red."),
        Color::new(
            "Raspberry",
            "#E30B5C",
            "A bright, slightly purplish-red, like the berry.",
        ),
        Color::new(
            "Rose Red",
            "#C21807",
            "A rich, vibrant red with rose undertones.",
        ),
        Color::new(
            "Shocking Pink",
            "#FC0FC0",
            "An extremely bright, artificial pink.",
        ),
        Color::new(
            "Terra Cotta",
            "#E2725B",
            "An earthy, brownish-orange-red, like clay pots.",
        ),
        // --- More Orange/Yellow Variations ---
        Color::new(
            "Burnt Orange",
            "#CC5500",
            "A dark, muted orange, like burnt embers.",
        ),
        Color::new(
            "Cadmium Yellow",
            "#FFF600",
            "A vivid, pure yellow, often used in art.",
        ),
        Color::new(
            "Canary Yellow",
            "#FFFF99",
            "A bright, cheerful yellow, like the bird.",
        ),
        Color::new(
            "Carrot Orange",
            "#ED7117",
            "A vibrant orange, like a fresh carrot.",
        ),
        Color::new(
            "Dandelion",
            "#F0E23D",
            "A bright, slightly muted yellow, like the flower.",
        ),
        Color::new(
            "Ecru",
            "#C2B280",
            "A light, yellowish-brown, like unbleached linen.",
        ),
        Color::new("Gamboge", "#E49B0F", "A deep, saffron-like yellow-orange."),
        Color::new(
            "Lemon Yellow",
            "#FFF700",
            "A pure, bright yellow, like a lemon.",
        ),
        Color::new("Mango Tango", "#FF8243", "A vibrant, tropical orange-pink."),
        Color::new(
            "Ochre",
            "#CC7722",
            "A natural, earthy yellow-orange-brown pigment.",
        ),
        Color::new(
            "Orange Peel",
            "#FFA000",
            "A bright, deep orange, like the peel of an orange.",
        ),
        Color::new("Sunglow", "#FFCC33", "A soft, warm, golden yellow."),
        Color::new("Tuscan Sun", "#FAD6A5", "A warm, radiant, golden-yellow."),
        // --- More Green/Cyan Variations ---
        Color::new(
            "Apple Green",
            "#8DB600",
            "A bright, fresh green, like a Granny Smith apple.",
        ),
        Color::new(
            "Artichoke Green",
            "#4B6F44",
            "A deep, grayish-green, like an artichoke.",
        ),
        Color::new(
            "Celadon",
            "#ACE1AF",
            "A pale, grayish-green, often found in ceramics.",
        ),
        Color::new("Dark Cyan", "#008B8B", "A deep, rich blue-green."),
        Color::new(
            "Emerald Green",
            "#046A38",
            "A very rich, dark emerald green.",
        ),
        Color::new(
            "Feldgrau",
            "#4D5D53",
            "A dull, grayish-green, historical military uniform color.",
        ),
        Color::new("Jade", "#00A86B", "A semi-precious green, like the stone."),
        Color::new("Jungle Green", "#29AB87", "A dark, vibrant tropical green."),
        Color::new("Laurel Green", "#A9BA9D", "A soft, muted, grayish-green."),
        Color::new(
            "Lime Green Pale",
            "#BFFF00",
            "A very bright, light lime green.",
        ),
        Color::new("Malachite", "#0BDA51", "A vibrant green, like the mineral."),
        Color::new("Moss Green", "#8A9A5B", "A dull, soft green, like moss."),
        Color::new(
            "Persian Green",
            "#00A693",
            "A strong, blue-green, associated with Persian art.",
        ),
        Color::new(
            "Pine Green",
            "#01796F",
            "A dark, deep green, like pine needles.",
        ),
        Color::new("Pistachio", "#93C572", "A light, slightly yellowish-green."),
        Color::new(
            "Shamrock Green",
            "#009E60",
            "A vivid, classic green, like a shamrock.",
        ),
        Color::new(
            "Viridian",
            "#40826D",
            "A dark shade of spring green with a slight blue tinge.",
        ),
        // --- More Blue Variations ---
        Color::new(
            "Aegean Blue",
            "#1C5678",
            "A deep, medium-dark blue, like the Aegean Sea.",
        ),
        Color::new(
            "Air Superiority Blue",
            "#72A0C1",
            "A grayish-blue, used by air forces.",
        ),
        Color::new(
            "Alice Blue",
            "#F0F8FF",
            "A very pale, light blue, almost white.",
        ),
        Color::new(
            "Baby Blue",
            "#89CFF0",
            "A light, soft blue, often associated with babies.",
        ),
        Color::new(
            "Bice Blue",
            "#327BA0",
            "A grayish-blue, often used in medieval art.",
        ),
        Color::new("Capri", "#00BFFF", "A vibrant, deep sky blue."),
        Color::new(
            "Cerulean",
            "#007BA7",
            "A deep sky blue, sometimes leaning green.",
        ),
        Color::new("Columbia Blue", "#C4D8E2", "A light, grayish-blue."),
        Color::new("Dark Turquoise", "#00CED1", "A rich, vibrant turquoise."),
        Color::new(
            "Denim",
            "#6F8EBD",
            "A classic blue, like faded denim jeans.",
        ),
        Color::new(
            "Duke Blue",
            "#00009C",
            "A very deep, dark blue, like Duke University colors.",
        ),
        Color::new("Indigo Dye", "#002F6C", "A very deep, dark indigo."),
        Color::new("Klein Blue", "#002FA7", "An intensely vibrant, pure blue."),
        Color::new(
            "Lapis Lazuli",
            "#26619C",
            "A deep, rich blue, like the gemstone.",
        ),
        Color::new("Light Cyan", "#E0FFFF", "A very pale, icy blue-green."),
        Color::new(
            "Maya Blue",
            "#73C2FB",
            "A bright, sky blue, historically used by Maya civilization.",
        ),
        Color::new("Munsell Blue", "#0093AF", "A vibrant, greenish-blue."),
        Color::new("Oxford Blue", "#002147", "A very dark, almost black blue."),
        Color::new("Persian Blue", "#1C39BB", "A brilliant, medium-dark blue."),
        Color::new("Prussian Blue", "#003153", "A dark, intense blue-green."),
        Color::new("Resolution Blue", "#002387", "A deep, pure blue."),
        Color::new(
            "Sapphire Blue",
            "#0F52BA",
            "A brilliant, deep blue, like the gemstone.",
        ),
        Color::new(
            "Ultramarine",
            "#3F00FF",
            "An intense, vivid blue, historically from lapis lazuli.",
        ),
        Color::new(
            "Yale Blue",
            "#0F4D92",
            "A deep, dark blue, like Yale University colors.",
        ),
        // --- More Purple Variations ---
        Color::new("Byzantium", "#702963", "A dark, reddish-purple."),
        Color::new(
            "Dark Lavender",
            "#734F96",
            "A muted, deeper shade of lavender.",
        ),
        Color::new("Dark Magenta", "#8B008B", "A very strong, pure magenta."),
        Color::new("Dark Orchid", "#9932CC", "A rich, deep orchid purple."),
        Color::new("Electric Violet", "#8F00FF", "A bright, intense violet."),
        Color::new("Heliotrope", "#DF73FF", "A light, reddish-purple."),
        Color::new(
            "Jacaranda",
            "#5C2E77",
            "A soft, pastel purple, like the flower.",
        ),
        Color::new(
            "KSU Purple",
            "#512888",
            "A deep, royal purple, like Kansas State University colors.",
        ),
        Color::new(
            "Lavender Blush",
            "#FFF0F5",
            "A very pale, pinkish-purple, almost white.",
        ),
        Color::new(
            "Lilac",
            "#C8A2C8",
            "A pale, delicate purple, like the flower.",
        ),
        Color::new("Orchid", "#DA70D6", "A light, vivid pink-purple."),
        Color::new("Periwinkle", "#CCCCFF", "A pale blue-violet."),
        Color::new("Puce", "#CC8899", "A brownish-purple color."),
        Color::new("Thistle Light", "#E6C9E6", "A softer, paler thistle color."),
        Color::new(
            "Tyrian Purple",
            "#66023C",
            "A rich, ancient reddish-purple.",
        ),
        Color::new("Wisteria", "#C9A0DC", "A soft, pale lilac-purple."),
        // --- More Brown/Neutral/Grey Variations ---
        Color::new("Bistre", "#3D2B1F", "A dark, yellowish-brown pigment."),
        Color::new("Bone", "#E3DAC9", "An off-white, light beige color."),
        Color::new(
            "Braun",
            "#964B00",
            "The German word for brown, often used for a deep brown.",
        ),
        Color::new(
            "Bronze Yellow",
            "#737000",
            "A greenish-yellow with bronze undertones.",
        ),
        Color::new("Buff", "#F0DC82", "A light, yellowish-brown color."),
        Color::new("Camel", "#C19A6B", "A light-brown, like camel hair."),
        Color::new("Cinnamon", "#D2691E", "A reddish-brown, like the spice."),
        Color::new(
            "Coffee",
            "#6F4E37",
            "A dark, rich brown, like roasted coffee beans.",
        ),
        Color::new("Copper", "#B87333", "A reddish-brown metallic color."),
        Color::new(
            "Cordovan",
            "#893F45",
            "A rich, dark reddish-brown leather color.",
        ),
        Color::new("Dark Sienna", "#3C1414", "A very deep, dark sienna."),
        Color::new(
            "Desert Sand",
            "#EDC9AF",
            "A light, yellowish-brown, like desert sand.",
        ),
        Color::new("Fallow", "#C19A6B", "A pale, yellowish-brown."), // Similar to Camel, but distinct
        Color::new(
            "Flax",
            "#EEDC82",
            "A pale, grayish-yellow, like flax fibers.",
        ),
        Color::new("Fuzzy Wuzzy", "#CC6666", "A playful, medium brownish-pink."),
        Color::new("Khaki Green", "#6B7D4E", "A muted, yellowish-green khaki."),
        Color::new("Light Taupe", "#B38B6D", "A softer, lighter brownish-gray."),
        Color::new("Mode Beige", "#967117", "A yellowish-brown beige."),
        Color::new(
            "Mocha",
            "#967969",
            "A dark, warm brown with a hint of gray.",
        ),
        Color::new("Nutmeg", "#714B23", "A warm, medium brown, like the spice."),
        Color::new(
            "Parchment",
            "#F0E6D0",
            "A light, yellowish-beige, like old paper.",
        ),
        Color::new(
            "Raw Umber",
            "#826644",
            "A natural, yellowish-brown pigment.",
        ),
        Color::new(
            "Rosy Brown Dark",
            "#A52A2A",
            "A darker, more intense rosy brown.",
        ),
        Color::new("Russet", "#80461B", "A reddish-brown."),
        Color::new("Seal Brown", "#321414", "A very dark, almost black brown."),
        Color::new("Sepia", "#704214", "A reddish-brown, like old photographs."),
        Color::new("Tan", "#D2B48C", "A light, yellowish-brown."),
        Color::new("Wheat", "#F5DEB3", "A pale, golden-yellowish-brown."),
        Color::new(
            "Warm Gray",
            "#808069",
            "A gray with a noticeable warm, brown undertone.",
        ),
        Color::new(
            "Cool Gray",
            "#80808F",
            "A gray with a noticeable cool, blue undertone.",
        ),
        Color::new(
            "Gunmetal",
            "#2A3439",
            "A dark, bluish-gray, like gun barrels.",
        ),
        Color::new("Platinum", "#E5E4E2", "A silvery-white metallic color."),
        Color::new("Nickel", "#727472", "A metallic gray color."),
        Color::new("Ash Grey", "#B2BEB5", "A lighter, cooler gray."),
        Color::new(
            "Eerie Black",
            "#1B1B1B",
            "An extremely dark, almost pure black.",
        ),
        // --- More Red/Pink Variations (cont.) ---
        Color::new(
            "Alizarin Crimson",
            "#E32636",
            "A deep, rich red with a slightly bluish tint.",
        ),
        Color::new(
            "Amaranth",
            "#E52B50",
            "A reddish-pink or reddish-rose color.",
        ),
        Color::new("Candy Pink", "#E4717A", "A medium-light, sweet pink."),
        Color::new("Carmine", "#960018", "A vivid crimson color."),
        Color::new("Cerise", "#DE3163", "A deep to vivid red or red-purple."),
        Color::new("Claret", "#811331", "A dark, purplish-red, like red wine."),
        Color::new(
            "Coquelicot",
            "#FF3800",
            "A brilliant red, like the poppy flower.",
        ),
        Color::new("Coral Red", "#FF4040", "A bright, intense reddish-orange."),
        Color::new("Cranberry", "#9F0022", "A deep, tart reddish-pink."),
        Color::new(
            "Cyclamen",
            "#F95D91",
            "A vibrant pinkish-purple, like the flower.",
        ),
        Color::new(
            "Dark Raspberry",
            "#872657",
            "A deep, rich, muted raspberry.",
        ),
        Color::new("Deep Cerise", "#DA3287", "A vivid, dark pink-red."),
        Color::new("English Red", "#C20000", "A traditional, deep, earthy red."),
        Color::new("Fandango", "#B53389", "A bright, purplish-pink."),
        Color::new(
            "Firebrick Red",
            "#B22222",
            "A dark, strong red, like heated brick.",
        ),
        Color::new(
            "Fluorescent Pink",
            "#FF1493",
            "An extremely vivid, glowing pink.",
        ),
        Color::new(
            "Fuchsia Pink",
            "#FF77FF",
            "A bright pink with a purplish tinge.",
        ),
        Color::new("Hibiscus", "#B7304F", "A vibrant, tropical red-pink."),
        Color::new("Hot Magenta", "#FF00CC", "An intense, glowing magenta."),
        Color::new("Imperial Red", "#ED2939", "A regal, brilliant red."),
        Color::new("Jam", "#660000", "A deep, dark red, like berry jam."),
        Color::new(
            "Magenta Dye",
            "#CA1F7B",
            "A deeper, more saturated magenta.",
        ),
        Color::new(
            "Mahogany Red",
            "#C04000",
            "A dark, reddish-brown, like the wood.",
        ),
        Color::new(
            "Mexican Pink",
            "#E4007C",
            "A bright, intense pink, often seen in Mexican culture.",
        ),
        Color::new("Opera Mauve", "#B784A7", "A pale, dusty purplish-pink."),
        Color::new("Persian Rose", "#FE28A6", "A bright, vibrant rose pink."),
        Color::new("Pompador", "#9D2A2A", "A deep, rich red, almost brown."),
        Color::new("Radical Red", "#FF355E", "A very bright, intense red."),
        Color::new("Rouge", "#A23B2A", "A reddish-brown or red color."),
        Color::new(
            "Rusty Red",
            "#DA2C43",
            "A deep, slightly dulled red with orange hints.",
        ),
        Color::new("Salmon Pink", "#FF91A4", "A lighter, softer salmon."),
        Color::new(
            "Sangria",
            "#92000A",
            "A dark, reddish-purple, like the drink.",
        ),
        Color::new("Strawberry", "#FC5A8D", "A bright, juicy pinkish-red."),
        Color::new("Taffy", "#EEA8C3", "A light, sweet pinkish-purple."),
        Color::new("Terra Rosa", "#C76060", "A reddish-brown earth pigment."),
        Color::new(
            "Venetian Red",
            "#C80815",
            "A warm, strong, historical red pigment.",
        ),
        Color::new("Watermelon", "#FD5B78", "A vibrant, sweet pinkish-red."),
        // --- More Orange/Yellow Variations (cont.) ---
        Color::new("Almond Frost", "#92877B", "A light, muted yellowish-brown."), // Close to beige but distinct
        Color::new(
            "Banana Yellow",
            "#FFE135",
            "A bright, pale yellow, like a ripe banana.",
        ),
        Color::new("Beer", "#F28E1C", "A rich, amber-orange, like beer."),
        Color::new(
            "Blonde",
            "#FAE6B1",
            "A pale, golden yellow, like blonde hair.",
        ),
        Color::new(
            "Bronze Gold",
            "#B27100",
            "A deep, metallic gold with bronze tones.",
        ),
        Color::new("Brown Sugar", "#AF673D", "A rich, warm brown."),
        Color::new("Buttermilk", "#FFFACD", "A pale, creamy yellow."),
        Color::new("Calico", "#EEB66B", "A light, warm, speckled tan."),
        Color::new(
            "Candy Corn",
            "#FFB531",
            "A gradient of yellow-orange-white, like the candy.",
        ),
        Color::new(
            "Citrine",
            "#E4D00A",
            "A pale yellow-orange, like the gemstone.",
        ),
        Color::new("Cream", "#FFFDD0", "A soft, pale yellow, like cream."),
        Color::new(
            "Dark Mustard",
            "#7B682D",
            "A deep, brownish mustard yellow.",
        ),
        Color::new(
            "Golden Brown",
            "#966919",
            "A rich, warm brown with gold undertones.",
        ),
        Color::new(
            "Goldenrod Dark",
            "#B8860B",
            "A deeper, more muted goldenrod.",
        ),
        Color::new("Honey", "#FFC30B", "A warm, golden yellow, like honey."),
        Color::new(
            "Jonquil",
            "#FADA5E",
            "A pale, delicate yellow, like the flower.",
        ),
        Color::new("Kumquat", "#E27B58", "A small, orange citrus fruit color."),
        Color::new("Light Ochre", "#EDD76D", "A paler, softer ochre."),
        Color::new(
            "Macaroni And Cheese",
            "#FFB97B",
            "A creamy, mild orange-yellow.",
        ),
        Color::new(
            "Marigold",
            "#EAA220",
            "A bright, deep orange-yellow, like the flower.",
        ),
        Color::new("Mellow Yellow", "#F8DE7E", "A soft, pleasant yellow."),
        Color::new("Mexican Yellow", "#FBB700", "A bright, vibrant yellow."),
        Color::new("Nectarine", "#FF9966", "A soft, peachy orange."),
        Color::new(
            "Orange Yellow",
            "#FFD200",
            "A pure yellow with orange tones.",
        ),
        Color::new("Papaya", "#FFEFD5", "A soft, pale orange-yellow."),
        Color::new(
            "Pineapple",
            "#563C0D",
            "A rich brown, hinting at the fruit's rind.",
        ),
        Color::new("Sand Yellow", "#F4A460", "A light, warm sandy color."),
        Color::new("Satin Sheen Gold", "#CBA135", "A muted, luxurious gold."),
        Color::new("Spice", "#6A4513", "A warm, earthy brown, like spices."),
        Color::new(
            "Sunset Orange",
            "#FD5E53",
            "A warm, vibrant orange, like a sunset.",
        ),
        Color::new(
            "Tangerine Orange",
            "#FFCC00",
            "A bright, pure tangerine color.",
        ),
        Color::new(
            "Topaz",
            "#FFC87C",
            "A pale, yellowish-orange, like the gemstone.",
        ),
        Color::new(
            "Turmeric",
            "#FE9A00",
            "A vibrant, yellowish-orange spice color.",
        ),
        Color::new("Vanilla Ice", "#F3DCDA", "A very pale, creamy peach."),
        // --- More Green/Cyan Variations (cont.) ---
        Color::new("Amazon Green", "#3B7A57", "A deep, rich jungle green."),
        Color::new("Bay Leaf", "#77966D", "A muted, grayish-green."),
        Color::new(
            "Bottle Green",
            "#006A4E",
            "A dark, deep green, like a glass bottle.",
        ),
        Color::new("Bright Green", "#66FF00", "An intense, glowing green."),
        Color::new("Cambridge Blue", "#A3C1AD", "A pale, grayish-green-blue."),
        Color::new("Chartreuse", "#7FFF00", "A vibrant, yellowish-green."),
        Color::new("Chrome Green", "#3D7A47", "A rich, dark green pigment."),
        Color::new("Dark Aqua", "#008B8B", "A deep, oceanic blue-green."),
        Color::new("Dark Cyan Blue", "#003366", "A very dark, deep blue-cyan."),
        Color::new(
            "Dark Jungle Green",
            "#1A2421",
            "An extremely dark, almost black green.",
        ),
        Color::new("Deep Green", "#056608", "A strong, pure deep green."),
        Color::new(
            "Emerald Palette",
            "#679B9B",
            "A muted, sophisticated emerald green.",
        ),
        Color::new("Green Sheen", "#6EAE7E", "A light, shimmering green."),
        Color::new("Jade Green", "#00A86B", "A vibrant, precious green."),
        Color::new(
            "Kelly Green Dark",
            "#008B00",
            "A deeper, more saturated kelly green.",
        ),
        Color::new("Light Jade", "#87D37C", "A pale, delicate jade green."),
        Color::new("Magic Mint", "#AAF0D1", "A light, refreshing pastel green."),
        Color::new(
            "Medium Jungle Green",
            "#1C332D",
            "A medium-dark jungle green.",
        ),
        Color::new(
            "Mantis",
            "#74C365",
            "A bright, medium green, like a praying mantis.",
        ),
        Color::new(
            "Neon Green",
            "#39FF14",
            "An artificial, extremely bright green.",
        ),
        Color::new("Pale Aqua", "#BCD4E6", "A very light, soft blue-green."),
        Color::new("Pastel Green", "#77DD77", "A soft, muted green."),
        Color::new(
            "Phthalo Green",
            "#123524",
            "A very dark, intense blue-green pigment.",
        ),
        Color::new(
            "Pine Green Dark",
            "#0A462C",
            "A very deep, dark pine green.",
        ),
        Color::new(
            "Robin Egg Blue",
            "#00CCCC",
            "A bright, clear blue-green, like the bird's egg.",
        ),
        Color::new(
            "Sage Green",
            "#8A9A8A",
            "A muted, grayish-green, like dried sage leaves.",
        ),
        Color::new(
            "Sea Foam Green",
            "#9FE2BF",
            "A light, airy green with blue undertones.",
        ),
        Color::new("Spanish Green", "#009B75", "A vibrant, classic green."),
        Color::new(
            "Spring Bud",
            "#A7FC00",
            "A very bright, yellowish-green, like a new bud.",
        ),
        Color::new("Teal Blue", "#367588", "A deeper, more blue-leaning teal."),
        Color::new(
            "Turquoise Blue",
            "#00C78C",
            "A vibrant, pure turquoise blue.",
        ),
        Color::new(
            "Verdigris",
            "#43B3AE",
            "A bluish-green, like oxidized copper.",
        ),
        // --- More Blue Variations (cont.) ---
        Color::new(
            "Amethyst Blue",
            "#939ACF",
            "A soft, muted blue with purple tones.",
        ),
        Color::new("Blue Sapphire", "#126180", "A deep, rich sapphire blue."),
        Color::new("Bright Blue", "#0099FF", "A vivid, clear blue."),
        Color::new(
            "Caribbean Blue",
            "#0066CC",
            "A vibrant, tropical ocean blue.",
        ),
        Color::new("Cerulean Blue", "#2A52BE", "A strong, clear sky blue."),
        Color::new(
            "Cornflower Blue Dark",
            "#3D59AB",
            "A deeper, more intense cornflower blue.",
        ),
        Color::new(
            "Dark Imperial Blue",
            "#001454",
            "An extremely dark, rich imperial blue.",
        ),
        Color::new(
            "Dark Midnight Blue",
            "#003366",
            "A very deep, almost black blue.",
        ),
        Color::new(
            "Egyptian Blue",
            "#1034A6",
            "A deep, historical blue pigment.",
        ),
        Color::new("Federal Blue", "#000080", "A deep, official navy blue."),
        Color::new(
            "French Blue",
            "#0072BB",
            "A vibrant, medium blue, associated with France.",
        ),
        Color::new("Gentian Blue", "#3C3D70", "A deep, dark purplish-blue."),
        Color::new(
            "Glaucous",
            "#6082B6",
            "A dull, grayish-blue or bluish-green.",
        ),
        Color::new("Indigo Blue", "#3F00FF", "A very vibrant indigo blue."),
        Color::new(
            "Iris Blue",
            "#0392CE",
            "A bright, clear blue, like the iris of an eye.",
        ),
        Color::new(
            "Light Cobalt Blue",
            "#88ACE0",
            "A softer, lighter cobalt blue.",
        ),
        Color::new(
            "Light Royal Blue",
            "#42A9FF",
            "A brighter, more vivid royal blue.",
        ),
        Color::new(
            "Majorelle Blue",
            "#6050DC",
            "An intense, vibrant blue-purple.",
        ),
        Color::new(
            "Medium Persian Blue",
            "#0067A5",
            "A balanced medium Persian blue.",
        ),
        Color::new("Navy", "#000080", "A very dark blue, almost black."),
        Color::new("Neon Blue", "#4D4DFF", "A bright, glowing blue."),
        Color::new("Olympic Blue", "#008CC9", "A clear, medium blue."),
        Color::new("Pacific Blue", "#1CA9C9", "A vibrant, oceanic blue."),
        Color::new(
            "Peacock Blue",
            "#0066CC",
            "A deep, lustrous blue with green hints.",
        ),
        Color::new("Persian Indigo", "#32127A", "A deep, dark purple-indigo."),
        Color::new("Picton Blue", "#3399FF", "A clear, bright sky blue."),
        Color::new(
            "Powder Blue Light",
            "#B0E0E6",
            "A paler, softer powder blue.",
        ),
        Color::new(
            "Robin's Egg Blue",
            "#00CCCC",
            "A very clear, bright greenish-blue.",
        ),
        Color::new(
            "Sapphire Blue Dark",
            "#082567",
            "A very deep, dark sapphire blue.",
        ),
        Color::new("Smalt", "#003399", "A vivid, glassy blue pigment."),
        Color::new("True Blue", "#0073CF", "A straightforward, honest blue."),
        Color::new("Turkish Blue", "#6D9DDE", "A medium, clear blue."),
        Color::new(
            "Ultramarine Blue",
            "#4166F5",
            "A highly intense, brilliant blue.",
        ),
        Color::new(
            "United Nations Blue",
            "#5B92E5",
            "A medium-light, official blue.",
        ),
        Color::new(
            "Violet Blue",
            "#324AB2",
            "A deep, strong blue with violet undertones.",
        ),
        // --- More Purple Variations (cont.) ---
        Color::new("African Violet", "#B284BE", "A soft, muted purplish-pink."),
        Color::new(
            "Byzantine",
            "#BD33A4",
            "A bright, purplish-pink with historical roots.",
        ),
        Color::new("Dark Byzantium", "#5D3954", "A deep, muted reddish-purple."),
        Color::new("Dark Lilac", "#9400D3", "A deep, strong lilac."),
        Color::new(
            "Dark Purple Blue",
            "#1F005B",
            "A very dark, almost black blue-purple.",
        ),
        Color::new("Deep Purple", "#360568", "A rich, intense deep purple."),
        Color::new(
            "Eggplant",
            "#614051",
            "A dark, brownish-purple, like the vegetable.",
        ),
        Color::new("Electric Indigo", "#6F00FF", "A vibrant, glowing indigo."),
        Color::new("Eminence", "#6C3082", "A rich, deep purple."),
        Color::new("Fairy Tale", "#F2C7E4", "A pale, dreamy pinkish-purple."),
        Color::new("Fuchsia Purple", "#CC397B", "A vivid purplish-pink."),
        Color::new("Grape Purple", "#573070", "A darker, richer grape color."),
        Color::new("Indigo Purple", "#4B0082", "A deep, true indigo color."),
        Color::new("Lavender Grey", "#C4C3D0", "A soft, grayish-purple."),
        Color::new("Lavender Purple", "#967BB6", "A medium, soft purple."),
        Color::new("Light Orchid", "#E6A8D7", "A soft, pale orchid."),
        Color::new("Magenta Haze", "#9F4576", "A muted, dusky magenta."),
        Color::new("Mauve Taupe", "#915F6D", "A muted, brownish-purple."),
        Color::new(
            "Mulberry",
            "#C54B8C",
            "A rich, reddish-purple, like the berry.",
        ),
        Color::new("Mystic Purple", "#5F4B8B", "A deep, enchanting purple."),
        Color::new("Pale Magenta", "#F984E5", "A very light, soft magenta."),
        Color::new(
            "Pansy Purple",
            "#78184A",
            "A deep, rich purple with red undertones.",
        ),
        Color::new(
            "Perrywinkle",
            "#CCCCFF",
            "A classic periwinkle blue-purple.",
        ),
        Color::new(
            "Plum (traditional)",
            "#8E4585",
            "A traditional, slightly muted plum.",
        ),
        Color::new("Pomp and Power", "#86608F", "A regal, grayish-purple."),
        Color::new(
            "Prune",
            "#701C1C",
            "A dark, reddish-brown, like dried plums.",
        ),
        Color::new("Purpureus", "#9A4EAE", "A strong, vibrant purple."),
        Color::new("Royal Fuchsia", "#CA2C92", "A bright, regal purplish-pink."),
        Color::new(
            "Royal Indigo",
            "#3D0C02",
            "A very deep, dark, reddish indigo.",
        ),
        Color::new("Seance", "#612F74", "A deep, mysterious purple."),
        Color::new("Twilight Lavender", "#8A496B", "A muted, dusky lavender."),
        Color::new("Tyrian Red", "#66023C", "A very deep, rich reddish-purple."),
        Color::new("Violet Red", "#F75394", "A vibrant, pinkish-purple."),
        Color::new("Wine", "#722F37", "A dark, brownish-red, like wine."),
        Color::new(
            "Wisteria Purple",
            "#C9A0DC",
            "A soft, pastel wisteria color.",
        ),
        // --- More Brown/Neutral/Grey Variations (cont.) ---
        Color::new(
            "Alabaster",
            "#F2F0E6",
            "A very pale, translucent off-white.",
        ),
        Color::new(
            "Almond White",
            "#F8F0E3",
            "A creamy, slightly yellowish white.",
        ),
        Color::new("Antiquewhite", "#FAEBD7", "A soft, aged-looking off-white."),
        Color::new("Argent", "#C0C0C0", "Another name for silver."),
        Color::new("Ash Brown", "#897D6E", "A muted, grayish-brown."),
        Color::new(
            "Avocado",
            "#568203",
            "A dull, brownish-green, like avocado flesh.",
        ),
        Color::new(
            "Beaver",
            "#9F8170",
            "A rich, medium-brown, like beaver fur.",
        ),
        Color::new("Bistre Brown", "#967117", "A dark, warm brown."),
        Color::new("Bone White", "#F9F6EE", "A very pale, soft, off-white."),
        Color::new("Bran", "#81411D", "A medium-dark, grainy brown."),
        Color::new(
            "Bronzy Yellow",
            "#737000",
            "A deep, muted yellow with bronze tones.",
        ),
        Color::new("Brown Derby", "#4A2C2A", "A dark, reddish-brown."),
        Color::new("Brown Tones", "#AF9B60", "A warm, earthy, medium brown."),
        Color::new("Burlap", "#A38D64", "A rough, textured light brown."),
        Color::new(
            "Cafe Noir",
            "#4B3621",
            "A very dark, almost black brown, like black coffee.",
        ),
        Color::new("Camel Brown", "#C19A6B", "A warm, yellowish-brown."),
        Color::new(
            "Carob Brown",
            "#6F492B",
            "A dark, rich brown, like carob pods.",
        ),
        Color::new(
            "Cedar Chest",
            "#C95A5F",
            "A reddish-brown, like cedar wood.",
        ),
        Color::new(
            "Champagne",
            "#F7E7CE",
            "A pale, yellowish-orange, like the drink.",
        ),
        Color::new("Chinchilla", "#9A9A8A", "A soft, grayish-brown."),
        Color::new("Cinder", "#4B4C53", "A dark, muted gray, like burnt ash."),
        Color::new("Clay", "#B66A50", "An earthy, reddish-brown."),
        Color::new("Coconut", "#965A3E", "A warm, tropical brown."),
        Color::new(
            "Coffee Bean",
            "#2A130C",
            "A very dark, rich brown, almost black.",
        ),
        Color::new(
            "Cool Black",
            "#002147",
            "A black with a noticeable blue undertone.",
        ),
        Color::new(
            "Copper Penny",
            "#AD6F69",
            "A reddish-brown, like an old copper coin.",
        ),
        Color::new(
            "Dark Khaki Green",
            "#737339",
            "A deeper, more saturated khaki green.",
        ),
        Color::new("Dark Wood", "#855E42", "A generic dark brown wood color."),
        Color::new(
            "Desert Brown",
            "#AA5C3F",
            "A warm, reddish-brown, like desert earth.",
        ),
        Color::new("Dirt", "#9B7653", "A light, earthy brown."),
        Color::new("Drab", "#967117", "A dull, light olive brown."),
        Color::new("Dusty Gray", "#A8A8A8", "A muted, somewhat faded gray."),
        Color::new("Earthen", "#A36D4C", "A natural, soil-like brown."),
        Color::new(
            "Fawn",
            "#E5AA70",
            "A light, yellowish-brown, like a fawn's coat.",
        ),
        Color::new("Flint", "#6F6C6C", "A hard, dark gray stone color."),
        Color::new("Fuzzy Brown", "#C45600", "A warm, medium brown."),
        Color::new("Gray Beige", "#E3DCCD", "A very pale, grayish beige."),
        Color::new("Grizzly", "#885834", "A dark, rugged brown."),
        Color::new(
            "Grullo",
            "#A99A86",
            "A dusty, grayish-brown, often used for horse coats.",
        ),
        Color::new("Hickory", "#725B4D", "A strong, medium-dark brown."),
        Color::new("Iron Gray", "#54534F", "A dark, metallic gray."),
        Color::new("Kelp", "#41403C", "A very dark, brownish-green."),
        Color::new("Light Khaki", "#F0E68C", "A pale, yellowish khaki."),
        Color::new("Lustre", "#7C6F5A", "A soft, muted metallic brown-gray."),
        Color::new("Mahogany Deep", "#411F1F", "A very deep, dark mahogany."),
        Color::new(
            "Maple",
            "#BF7F63",
            "A warm, reddish-brown, like maple wood.",
        ),
        Color::new("Marble", "#DDDDDD", "A light, variegated gray."),
        Color::new("Medium Taupe", "#674C47", "A balanced medium taupe."),
        Color::new("Mink", "#885B3F", "A soft, rich brown, like mink fur."),
        Color::new("Moose", "#964B00", "A strong, dark brown."),
        Color::new(
            "Mud Brown",
            "#604B32",
            "A deep, earthy, somewhat dirty brown.",
        ),
        Color::new("Musk", "#6C5441", "A warm, medium-dark brown."),
        Color::new("Nude", "#E3BC9A", "A light, peachy-beige skin tone."),
        Color::new(
            "Olive Brown",
            "#6B8E23",
            "A yellowish-brown with green undertones.",
        ),
        Color::new(
            "Paper Brown",
            "#A37A5D",
            "A light, flat brown, like brown paper.",
        ),
        Color::new("Pecan", "#4C3024", "A deep, rich brown, like a pecan nut."),
        Color::new("Pewter Gray", "#899499", "A metallic, medium gray."),
        Color::new("Putty", "#E7CDA2", "A soft, yellowish-beige."),
        Color::new("Raw Sienna", "#D68A59", "A yellowish-brown earth pigment."),
        Color::new("Sand", "#C2B280", "A light, natural sandy color."),
        Color::new("Seal", "#36160F", "A very dark, almost black brown."),
        Color::new("Shadow", "#8A795D", "A muted, brownish-gray."),
        Color::new("Silver Gray", "#B8B8B8", "A classic, light silvery gray."),
        Color::new("Smokey Gray", "#726D6D", "A medium, hazy gray."),
        Color::new("Snuff", "#826644", "A grayish-brown, like dried tobacco."),
        Color::new(
            "Stone Gray",
            "#85817E",
            "A solid, neutral gray, like stone.",
        ),
        Color::new("Sugar Cane", "#999966", "A light, brownish-green."),
        Color::new("Tuscan Brown", "#664228", "A rich, deep brown."),
        Color::new("Umber", "#635147", "A dark, natural brown pigment."),
        Color::new(
            "Warm Grey",
            "#8C8C7D",
            "A grey with subtle warm undertones.",
        ),
        Color::new(
            "White Smoke",
            "#F5F5F5",
            "A very pale, off-white, almost gray.",
        ),
        Color::new("Wood Brown", "#C19A6B", "A natural, medium wood color."),
        Color::new(
            "Zinnwaldite",
            "#EBC2AF",
            "A pale, brownish-pink, mineral color.",
        ),
    ]
}
