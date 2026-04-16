//! Motore di generazione

use crate::config::GenConfig;
use crate::generator::CodeGenerator;

pub struct GenerationEngine {
    config: GenConfig,
    generator: CodeGenerator,
}

impl GenerationEngine {
    pub fn new(config: GenConfig) -> Result<Self, String> {
        let generator = CodeGenerator::new(&config)?;
        Ok(Self { config, generator })
    }
    
    pub fn generate_all(&self) -> Result<(), String> {
        std::fs::create_dir_all(&self.config.output_dir)
            .map_err(|e| e.to_string())?;
        
        for input_path in &self.config.files_to_process {
            if self.config.verbose {
                println!("\n📄 Processo: {:?}", input_path);
            }
            
            // Usa il metodo che non sovrascrive _impl.rs
            self.generator.generate_to_file(input_path, &self.config.output_dir)?;
            
            if self.config.verbose {
                let stem = input_path.file_stem().unwrap().to_string_lossy();
                let output_path = self.config.output_dir.join(format!("{}.rs", stem));
                println!("  ✅ Generato: {:?}", output_path);
            }
        }
        
        Ok(())
    }
}
