//! Test del motore di inferenza IRCM

use sapri_ai::{Brain, Reasoner};

fn main() -> Result<(), String> {
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║              SAPRI AI - TEST INFERENZA IRCM                ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    let brain = Brain::new()?;
    // Ora knowledge() è un metodo pubblico
    let reasoner = Reasoner::new(brain.knowledge().clone());

    let test_queries = vec![
        "cos'è un gatto",
        "gatto",
        "felino",
        "animale",
    ];

    println!("🔍 Test query di inferenza:\n");
    for query in test_queries {
        let result = reasoner.query(query);
        println!("Q: {}", query);
        println!("A: {}", result.answer);
        println!("   (confidence: {:.2}, S: {:.2})\n", result.confidence, result.s_score);
    }

    Ok(())
}
