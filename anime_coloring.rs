// anime_coloring.rs — Rust версия

use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Write;

lazy_static::lazy_static! {
    static ref COLORS: HashMap<String, u8> = {
        let mut m = HashMap::new();
        m.insert("black".to_string(), 30);
        m.insert("red".to_string(), 31);
        m.insert("green".to_string(), 32);
        m.insert("yellow".to_string(), 33);
        m.insert("blue".to_string(), 34);
        m.insert("magenta".to_string(), 35);
        m.insert("cyan".to_string(), 36);
        m.insert("white".to_string(), 37);
        m.insert("gray".to_string(), 90);
        m.insert("bright_red".to_string(), 91);
        m.insert("bright_green".to_string(), 92);
        m.insert("bright_yellow".to_string(), 93);
        m.insert("bright_blue".to_string(), 94);
        m.insert("bright_magenta".to_string(), 95);
        m.insert("bright_cyan".to_string(), 96);
        m.insert("bright_white".to_string(), 97);
        m
    };
}

const TEMPLATE: &str = "
          {hair}██{hair}██{hair}██{hair}██{hair}██
        {hair}██{skin}      {skin}{hair}██
       {hair}██  {eye}██{hair}  {hair}██  {eye}██{hair}  {hair}██
       {hair}██  {eye}██{hair}  {hair}██  {eye}██{hair}  {hair}██
        {hair}██{skin}      {skin}{hair}██
         {hair}██{hair}██{hair}██{hair}██{hair}██{hair}██{hair}██{hair}
          {hair}██  {hair}██
         {hair}██{clothes}    {clothes}{hair}██
         {hair}██{clothes}    {clothes}{hair}██
          {hair}██{hair}██{hair}██{hair}██{hair}██";

fn colorize(text: &str, color_name: &str) -> String {
    let code = COLORS.get(color_name).unwrap_or(&37);
    format!("\x1b[{}m{}\x1b[0m", code, text)
}

fn generate_image(hair: &str, eyes: &str, skin: &str, clothes: &str) -> String {
    let mut result = TEMPLATE.to_string();
    result = result.replace("{hair}", &colorize("██", hair));
    result = result.replace("{eyes}", &colorize("██", eyes));
    result = result.replace("{skin}", &colorize("  ", skin));
    result = result.replace("{clothes}", &colorize("██", clothes));
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut hair = "magenta".to_string();
    let mut eyes = "cyan".to_string();
    let mut skin = "yellow".to_string();
    let mut clothes = "blue".to_string();
    let mut output = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--hair" => { hair = args[i+1].clone(); i += 2; }
            "--eyes" => { eyes = args[i+1].clone(); i += 2; }
            "--skin" => { skin = args[i+1].clone(); i += 2; }
            "--clothes" => { clothes = args[i+1].clone(); i += 2; }
            "--output" => { output = Some(args[i+1].clone()); i += 2; }
            _ => { i += 1; }
        }
    }

    println!("🎨 Anime Coloring (Rust)");
    println!("Цвета: волосы={}, глаза={}, кожа={}, одежда={}", hair, eyes, skin, clothes);
    println!();
    let image = generate_image(&hair, &eyes, &skin, &clothes);
    println!("{}", image);

    if let Some(path) = output {
        let mut file = fs::File::create(&path).unwrap();
        writeln!(file, "{}", image).unwrap();
        println!("💾 Сохранено в {}", path);
    }
}
