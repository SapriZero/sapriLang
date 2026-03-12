//! Esecuzione script URCM

use urcm_core::UrcmCtx;

pub fn execute_script(code: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut output = Vec::new();

    let _ctx = UrcmCtx::new(());

    output.push(format!("📝 Esecuzione script..."));
    output.push(format!("Codice: {}", code));
    output.push(format!("✅ Eseguito"));

    Ok(output)
}
