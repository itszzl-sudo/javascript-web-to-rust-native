//! JRust Code Generation Demo - CodeOnly mode
use jrust_runtime::director::{Director, BuildConfig, BuildMode};
use jrust_runtime::dom::document::Document;
use jrust_runtime::dom::element::Element;
use std::path::PathBuf;
use std::fs;

fn main() -> std::result::Result<(), String> {
    println!("=== JRust Code Generation Demo (CodeOnly Mode) ===\n");

    let mut director = Director::new();

    let mut config = director.config_mut();
    config.build.mode = BuildMode::CodeOnly;
    config.build.compile_lib = false;
    config.build.compile_exe = false;
    config.build.generate_snap = false;
    config.build.split_code = false;
    config.project.title = "JRust Cool Demo".to_string();
    config.project.name = "cool_vue_demo".to_string();
    drop(config);

    println!("Config: CodeOnly mode - code generation only\n");

    println!("--- 1. Create Mock DOM (Vue-like) ---\n");
    
    let mut document = Document::new();
    document.set_title("JRust Cool Demo");

    let mut body = document.body_mut();

    let mut app_div = Element::new("div");
    app_div.set_id("app");
    app_div.set_attribute("style", "min-height:100vh;background:linear-gradient(135deg,#0a0e27,#1a142e,#0f172a);padding:60px 20px;text-align:center;font-family:Segoe UI,sans-serif;");
    body.append_child(app_div.clone());

    let mut h1 = Element::new("h1");
    h1.set_attribute("style", "font-size:4rem;margin-bottom:10px;background:linear-gradient(135deg,#60a5fa,#a78bfa,#f472b6);-webkit-background-clip:text;color:transparent;");
    h1.set_text_content("JRust");
    app_div.append_child(h1);

    let mut subtitle = Element::new("p");
    subtitle.set_attribute("style", "font-size:1.5rem;color:#94a3b8;letter-spacing:2px;margin-bottom:50px;");
    subtitle.set_text_content("Vue -> Rust -> Native");
    app_div.append_child(subtitle);

    let mut cards_div = Element::new("div");
    cards_div.set_attribute("style", "display:flex;flex-wrap:wrap;justify-content:center;gap:30px;margin-bottom:60px;");

    let card_data = [
        ("Lightning", "Lightning Translate", "JavaScript -> Rust, very fast", true),
        ("DOM", "DOM Simulation", "Complete browser API", false),
        ("Servo", "Servo Integration", "Real rendering engine", false),
    ];

    for (icon, title, desc, active) in card_data {
        let mut card = Element::new("div");
        let card_style = format!(
            "background:rgba(255,255,255,0.05);backdrop-filter:blur(10px);border:1px solid rgba(255,255,255,0.1);border-radius:20px;padding:40px 30px;width:280px;cursor:pointer;transition:all 0.3s;{}",
            if active { "border-color:rgba(96,165,250,0.5);" } else { "" }
        );
        card.set_attribute("style", &card_style);

        let mut icon_div = Element::new("div");
        icon_div.set_attribute("style", "font-size:4rem;margin-bottom:15px;");
        icon_div.set_text_content(icon);
        card.append_child(icon_div);

        let mut h3 = Element::new("h3");
        h3.set_attribute("style", "color:#e2e8f0;font-size:1.4rem;margin-bottom:10px;");
        h3.set_text_content(title);
        card.append_child(h3);

        let mut p = Element::new("p");
        p.set_attribute("style", "color:#94a3b8;font-size:1rem;line-height:1.6;");
        p.set_text_content(desc);
        card.append_child(p);

        cards_div.append_child(card);
    }
    app_div.append_child(cards_div);

    let mut counter_section = Element::new("div");
    counter_section.set_attribute("style", "margin-top:40px;");

    let mut counter_h2 = Element::new("h2");
    counter_h2.set_attribute("style", "color:#e2e8f0;font-size:2rem;margin-bottom:30px;");
    counter_h2.set_text_content("Magic Counter: 0");
    counter_section.append_child(counter_h2);

    let mut buttons_div = Element::new("div");
    buttons_div.set_attribute("style", "display:flex;justify-content:center;gap:20px;");

    let button_styles = [
        ("padding:15px 40px;font-size:1.2rem;font-weight:600;border:none;border-radius:12px;cursor:pointer;background:linear-gradient(135deg,#f87171,#ef4444);color:white;transition:all 0.3s;", "-"),
        ("padding:15px 40px;font-size:1.2rem;font-weight:600;border:none;border-radius:12px;cursor:pointer;background:linear-gradient(135deg,#60a5fa,#3b82f6);color:white;transition:all 0.3s;", "+"),
        ("padding:15px 40px;font-size:1.2rem;font-weight:600;border:none;border-radius:12px;cursor:pointer;background:rgba(148,163,184,0.2);color:#cbd5e1;transition:all 0.3s;", "Reset"),
    ];

    for (style, text) in button_styles {
        let mut btn = Element::new("button");
        btn.set_attribute("style", style);
        btn.set_text_content(text);
        buttons_div.append_child(btn);
    }
    counter_section.append_child(buttons_div);
    app_div.append_child(counter_section);

    body.append_child(app_div);

    println!("Mock DOM created!");
    println!("   - Title: JRust");
    println!("   - 3 cards");
    println!("   - Magic counter");
    println!();

    println!("--- 2. Generate Mock JRust Code ---\n");
    
    let jrust_code = generate_mock_jrust_code();
    
    println!("Generated JRust code length: {} chars", jrust_code.len());
    println!();

    println!("--- 3. Save Config to File ---\n");
    
    let output_dir = PathBuf::from("dist").join("cool_vue_demo");
    fs::create_dir_all(&output_dir)
        .map_err(|e| format!("Failed to create output dir: {}", e))?;
    
    let config_path = output_dir.join("jrust.config.json");
    director.save_config(&config_path)?;
    println!("Config saved to: {:?}", config_path);
    println!();

    println!("--- 4. Call Director full_build_pipeline ---\n");
    
    let dist_path = PathBuf::from("mock_dist");
    let output = director.full_build_pipeline(&jrust_code, "cool_vue_demo", &dist_path)?;
    
    println!("Code generation complete!");
    println!();

    println!("--- 5. Show Generated Files ---\n");
    
    if let Ok(entries) = fs::read_dir(&output.source_dir) {
        let mut files: Vec<_> = entries.flatten().collect();
        files.sort_by_key(|e| e.file_name());
        for entry in files {
            let path = entry.path();
            if path.is_file() {
                let size = fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
                println!("   file: {:?} ({} bytes)", path.file_name().unwrap(), size);
            }
        }
    }

    println!("\n--- 6. JRust Code Preview ---\n");
    println!("{}", "=".repeat(60));
    
    let preview_lines: Vec<_> = jrust_code.lines().take(40).collect();
    for (i, line) in preview_lines.iter().enumerate() {
        println!("{:3}: {}", i + 1, line);
    }
    
    if jrust_code.lines().count() > 40 {
        println!("... ({} lines total)", jrust_code.lines().count());
    }
    
    println!("{}", "=".repeat(60));
    println!();

    println!("Demo complete!\n");
    println!("Output dir: {:?}", output.output_dir);
    println!("You can review the generated code to see translator output!");
    
    Ok(())
}

fn generate_mock_jrust_code() -> String {
    let mut code = String::new();
    code.push_str("//! JRust generated native code\n");
    code.push_str("use jrust_runtime::director::Director;\n");
    code.push_str("use jrust_runtime::dom::document::Document;\n");
    code.push_str("use jrust_runtime::dom::element::Element;\n");
    code.push_str("use jrust_runtime::comm::{ThreadChannel, ThreadMessage};\n\n");
    code.push_str("fn main() -> Result<(), String> {\n");
    code.push_str("    println!(\"JRust app started!\");\n");
    code.push_str("    let document = create_app()?;\n");
    code.push_str("    let (tx, rx) = ThreadChannel::new();\n");
    code.push_str("    tx.send(ThreadMessage::UpdateDocument(document.to_html()?))?;\n");
    code.push_str("    event_loop(rx)?;\n");
    code.push_str("    Ok(())\n");
    code.push_str("}\n\n");
    code.push_str("fn create_app() -> Result<Document, String> {\n");
    code.push_str("    let mut document = Document::new();\n");
    code.push_str("    document.set_title(\"JRust Cool Demo\");\n");
    code.push_str("    let mut body = document.body_mut();\n");
    code.push_str("    let mut app_div = Element::new(\"div\");\n");
    code.push_str("    app_div.set_id(\"app\");\n");
    code.push_str("    body.append_child(app_div);\n");
    code.push_str("    let mut h1 = Element::new(\"h1\");\n");
    code.push_str("    h1.set_text_content(\"JRust\");\n");
    code.push_str("    app_div.append_child(h1);\n");
    code.push_str("    create_cards(&mut app_div)?;\n");
    code.push_str("    create_counter(&mut app_div)?;\n");
    code.push_str("    Ok(document)\n");
    code.push_str("}\n\n");
    code.push_str("fn create_cards(parent: &mut Element) -> Result<(), String> {\n");
    code.push_str("    let cards_data = [\n");
    code.push_str("        (\"Lightning\", \"Lightning Translate\", \"JS -> Rust\"),\n");
    code.push_str("        (\"DOM\", \"DOM Simulation\", \"Complete browser API\"),\n");
    code.push_str("        (\"Servo\", \"Servo Integration\", \"Real rendering engine\"),\n");
    code.push_str("    ];\n");
    code.push_str("    let mut cards_div = Element::new(\"div\");\n");
    code.push_str("    cards_div.set_attribute(\"style\", \"display:flex;flex-wrap:wrap;gap:30px;\");\n");
    code.push_str("    for (icon, title, desc) in cards_data {\n");
    code.push_str("        let mut card = Element::new(\"div\");\n");
    code.push_str("        card.set_attribute(\"style\", \"background:rgba(...);border-radius:20px;padding:40px;\");\n");
    code.push_str("        let mut icon_div = Element::new(\"div\");\n");
    code.push_str("        icon_div.set_text_content(icon);\n");
    code.push_str("        card.append_child(icon_div);\n");
    code.push_str("        let mut h3 = Element::new(\"h3\");\n");
    code.push_str("        h3.set_text_content(title);\n");
    code.push_str("        card.append_child(h3);\n");
    code.push_str("        let mut p = Element::new(\"p\");\n");
    code.push_str("        p.set_text_content(desc);\n");
    code.push_str("        card.append_child(p);\n");
    code.push_str("        cards_div.append_child(card);\n");
    code.push_str("    }\n");
    code.push_str("    parent.append_child(cards_div);\n");
    code.push_str("    Ok(())\n");
    code.push_str("}\n\n");
    code.push_str("fn create_counter(parent: &mut Element) -> Result<(), String> {\n");
    code.push_str("    let mut section = Element::new(\"div\");\n");
    code.push_str("    section.set_attribute(\"style\", \"margin-top:40px;\");\n");
    code.push_str("    let mut h2 = Element::new(\"h2\");\n");
    code.push_str("    h2.set_text_content(\"Magic Counter: 0\");\n");
    code.push_str("    section.append_child(h2);\n");
    code.push_str("    let mut buttons_div = Element::new(\"div\");\n");
    code.push_str("    buttons_div.set_attribute(\"style\", \"display:flex;justify-content:center;gap:20px;\");\n");
    code.push_str("    for (label, color) in [(\"-\", \"red\"), (\"+\", \"blue\"), (\"Reset\", \"gray\")] {\n");
    code.push_str("        let mut btn = Element::new(\"button\");\n");
    code.push_str("        btn.set_attribute(\"style\", &format!(\"padding:15px 40px;background:{}\", color));\n");
    code.push_str("        btn.set_text_content(label);\n");
    code.push_str("        buttons_div.append_child(btn);\n");
    code.push_str("    }\n");
    code.push_str("    section.append_child(buttons_div);\n");
    code.push_str("    parent.append_child(section);\n");
    code.push_str("    Ok(())\n");
    code.push_str("}\n\n");
    code.push_str("fn event_loop(rx: ThreadChannel) -> Result<(), String> {\n");
    code.push_str("    println!(\"event_loop started...\");\n");
    code.push_str("    loop {\n");
    code.push_str("        match rx.recv() {\n");
    code.push_str("            Ok(ThreadMessage::Shutdown) => {\n");
    code.push_str("                println!(\"Received shutdown signal \");\n");
    code.push_str("                break;\n");
    code.push_str("            }\n");
    code.push_str("            Ok(ThreadMessage::UpdateDocument(html)) => {\n");
    code.push_str("                println!(\"Update document ({} chars)\", html.len());\n");
    code.push_str("            }\n");
    code.push_str("            Ok(ThreadMessage::SendEvent(event)) => {\n");
    code.push_str("                println!(\"Received event: {}\", event);\n");
    code.push_str("            }\n");
    code.push_str("            Err(_) => {\n");
    code.push_str("                println!(\"Channel error \");\n");
    code.push_str("                break;\n");
    code.push_str("            }\n");
    code.push_str("            _ => {}\n");
    code.push_str("        }\n");
    code.push_str("    }\n");
    code.push_str("    Ok(())\n");
    code.push_str("}\n\n");
    code.push_str("impl Document {\n");
    code.push_str("    fn to_html(&self) -> Result<String, String> {\n");
    code.push_str("        Ok(\"<html>...</html>\".to_string())\n");
    code.push_str("    }\n");
    code.push_str("}\n");
    code
}
