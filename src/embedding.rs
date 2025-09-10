use anyhow::Result;
use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};

pub struct EmbeddingGenerator {
    embedder: TextEmbedding,
}

impl EmbeddingGenerator {
    pub async fn new() -> Result<Self> {
        // Try AllMiniLML12V2 first, fallback to L6V2
        let embedder = match TextEmbedding::try_new(
            InitOptions::new(EmbeddingModel::AllMiniLML12V2).with_show_download_progress(true),
        ) {
            Ok(model) => model,
            Err(e) => {
                println!("⚠️  Failed to load AllMiniLML12V2: {}", e);
                println!("🔄 Falling back to AllMiniLML6V2...");
                TextEmbedding::try_new(
                    InitOptions::new(EmbeddingModel::AllMiniLML6V2)
                        .with_show_download_progress(true),
                )?
            }
        };

        Ok(Self { embedder })
    }

    pub async fn generate_embeddings(&mut self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        let text_refs: Vec<&str> = texts.iter().map(|s| s.as_str()).collect();
        let embeddings = self.embedder.embed(text_refs, None)?;
        Ok(embeddings)
    }

    pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        if a.len() != b.len() {
            return 0.0;
        }

        let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm_a == 0.0 || norm_b == 0.0 {
            return 0.0;
        }

        dot_product / (norm_a * norm_b)
    }
}
