use crate::colors::Color;
use crate::config::Config;
use crate::embedding::EmbeddingGenerator;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct ColorMatch {
    pub color: Color,
    pub score: f32,
}

pub struct MoodPaletteGenerator {
    colors: Vec<Color>,
    config: Config,
    embedder: EmbeddingGenerator,
    color_embeddings: Vec<Vec<f32>>,
}

impl MoodPaletteGenerator {
    pub async fn new(config: Config, colors: Vec<Color>) -> Result<Self> {
        let mut embedder = EmbeddingGenerator::new().await?;

        println!("🔄 Pre-computing color embeddings...");
        let color_texts: Vec<String> = colors
            .iter()
            .map(|color| format!("{}, {}", color.name, color.description))
            .collect();

        let color_embeddings = embedder.generate_embeddings(&color_texts).await?;
        println!(
            "✅ Pre-computed {} color embeddings",
            color_embeddings.len()
        );

        Ok(Self {
            colors,
            config,
            embedder,
            color_embeddings,
        })
    }

    pub async fn generate_palette(&mut self, mood_text: &str) -> Result<Vec<ColorMatch>> {
        if mood_text.trim().is_empty() {
            return Ok(vec![]);
        }

        // Generate embedding for the mood text
        let mood_embedding = self
            .embedder
            .generate_embeddings(&[mood_text.to_string()])
            .await?;

        if mood_embedding.is_empty() {
            return Ok(vec![]);
        }

        let mood_vec = &mood_embedding[0];

        // Calculate cosine similarity with all color embeddings
        let mut matches: Vec<ColorMatch> = self
            .colors
            .iter()
            .zip(self.color_embeddings.iter())
            .map(|(color, color_embedding)| {
                let score = EmbeddingGenerator::cosine_similarity(mood_vec, color_embedding);
                ColorMatch {
                    color: color.clone(),
                    score,
                }
            })
            .collect();

        // Sort by similarity score (highest first)
        matches.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        // Return top k matches
        Ok(matches.into_iter().take(self.config.top_k).collect())
    }
}
