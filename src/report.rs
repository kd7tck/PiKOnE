use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

pub fn generate_pdf(
    filename: &str,
    choice_a: &str,
    choice_b: &str,
    total_sessions: usize,
    count_a: usize,
    count_b: usize,
    z_score: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    let (doc, page1, layer1) = PdfDocument::new("PiKOnE Report", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    // Font
    let font = doc.add_builtin_font(BuiltinFont::Helvetica)?;
    let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold)?;

    // Title
    current_layer.use_text("PiKOnE Simulation Report", 24.0, Mm(20.0), Mm(270.0), &font_bold);

    // Parameters
    current_layer.use_text("Simulation Parameters:", 14.0, Mm(20.0), Mm(250.0), &font_bold);
    current_layer.use_text(format!("Choice A: {}", choice_a), 12.0, Mm(30.0), Mm(240.0), &font);
    current_layer.use_text(format!("Choice B: {}", choice_b), 12.0, Mm(30.0), Mm(233.0), &font);
    current_layer.use_text(format!("Total Sessions: {}", total_sessions), 12.0, Mm(30.0), Mm(226.0), &font);

    // Results
    current_layer.use_text("Results:", 14.0, Mm(20.0), Mm(200.0), &font_bold);
    current_layer.use_text(format!("Selected A: {}", count_a), 12.0, Mm(30.0), Mm(190.0), &font);
    current_layer.use_text(format!("Selected B: {}", count_b), 12.0, Mm(30.0), Mm(183.0), &font);

    // Statistics
    current_layer.use_text("Statistical Analysis:", 14.0, Mm(20.0), Mm(160.0), &font_bold);
    current_layer.use_text(format!("Z-Score: {:.4}", z_score), 12.0, Mm(30.0), Mm(150.0), &font);

    let message = if z_score.abs() > 1.96 {
        "Result: Significant Anomaly Detected (p < 0.05)"
    } else {
        "Result: No Significant Anomaly Detected"
    };
    current_layer.use_text(message, 12.0, Mm(30.0), Mm(143.0), &font_bold);

    // Disclaimer
    current_layer.use_text("Generated using CURBy Quantum Randomness Beacon", 10.0, Mm(20.0), Mm(20.0), &font);

    let file = File::create(filename)?;
    doc.save(&mut BufWriter::new(file))?;

    Ok(())
}
