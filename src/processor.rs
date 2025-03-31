use anyhow::{Context, Result};
use ignore::WalkBuilder;
use ignore::overrides::OverrideBuilder;
use std::path::{Path, PathBuf};
use std::fs;
use serde::Serialize;
use indicatif::ProgressBar;
use pulldown_cmark::{Parser as MdParser, Event, Tag};



#[derive(Serialize)] // Añade esta línea
/// Representa un chunk de codigo con metadatos.
pub struct CodeChunk {
    pub file_path: PathBuf,
    pub content: String,
    pub language: String,
}

/// Detecta archvos de código en un directorio y devuelve sus rutas.

pub fn find_code_files(dir: &str, exclude_patterns: &[String]) -> Result<Vec<PathBuf>> {
    let project_root = Path::new(dir);
    let file_patterns = load_ignore_patterns(project_root)?;


    let mut overrides = OverrideBuilder::new(project_root);
    // Procesar patrones de exclusión de CLI
    for pattern in file_patterns.iter().chain(exclude_patterns.iter()) {
    overrides.add(&format!("!{}", pattern))?;
    }

    let overrides = overrides.build()?; 

    let walker = WalkBuilder::new(dir)
        .overrides(overrides)
        .add_custom_ignore_filename(".coderagignore") // Nuevo archivo de ignore
        .git_ignore(true)
        .build();
    let mut code_files = Vec::new();

    for entry in walker {
        let entry = entry.context(format!("Failed to read entry in directory {}", dir))?;
        let path = entry.path();
        if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) && is_code_file(path) {
            code_files.push(entry.into_path());
        }
    }


    Ok(code_files)
}

/// Determina si un archivo es de código (basado en su extensión).
fn is_code_file(path: &Path) -> bool {

    // Para depuración
    println!("Checking file: {:?}", path);

    match path.extension().and_then(|ext| ext.to_str()) {
        // Lenguajes de programación
        Some ("rs") | Some("py") | Some("js") | Some("ts") | Some("go") |
        // Archivos de documentación/configuración
        Some("md") | Some("json") | Some("toml") | Some("yaml") | Some("yml") => true,
        _ => false,
    }
}





/// Extrae el contenido de los archivos y genera chunks.

pub fn extract_code_chunks(files: Vec<PathBuf>) -> Result<Vec<CodeChunk>> {
    let mut chunks = Vec::new();
    let total_files = files.len();
    
    // Crea una barra de progreso 
    let pb = ProgressBar::new(total_files as u64);
    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} files ({eta})")?
            .progress_chars("#>-"),
    );

    for file in files {

        pb.set_message(format!("Processing: {}", file.display()));  // Actualiza el mensaje dinámico
        pb.inc(1); // Incrementa el progreso

        let content = fs::read_to_string(&file)?;        
        let processed_content = match file.extension().and_then(|ext| ext.to_str()) {
            Some("md") => process_markdown(&content).join("\n---\n"),
            _ => content
        };

        let language = match file.extension().and_then(|ext| ext.to_str()) {
            // Lenguajes de programación conocidos
            Some("rs") => "Rust",
            Some("py") => "Python",
            Some("js") => "JavaScript",
            Some("ts") => "TypeScript",
            Some("go") => "Go",
            // Archivos de documentación/configuración
            Some("md") => "Markdown",
            Some("json") => "JSON",
            Some("toml") => "TOML",
            Some("yaml") | Some("yml") => "YAML",
            _ => "Unknown"
        }.to_string();

        chunks.push(CodeChunk {
            file_path: file,
            content: processed_content,
            language,
        });
    }
    pb.finish_with_message("Done processing files");
    Ok(chunks)
}


/// Divide archivos Markdown en chunks por secciones
fn process_markdown(content: &str) -> Vec<String> {
    let parser = MdParser::new(content);
    let mut chunks = Vec::new();
    let mut current_chunk = String::new();

    for event in parser {
        match event {
            Event::Start(Tag::Heading{ level: _, ..}) => {
                if !current_chunk.is_empty() {
                    chunks.push(current_chunk.trim().to_string());
                    current_chunk = String::new();
                }
            }
            Event::Text(text) => current_chunk.push_str(&text),
            _ => {}
        }
    }

    if !current_chunk.is_empty() {
        chunks.push(current_chunk.trim().to_string());
    }

    chunks
}

pub fn load_ignore_patterns(project_root: &Path) -> Result<Vec<String>> {
    let ignore_file = project_root.join(".coderagignore");

    if ignore_file.exists() {
        let content = std::fs::read_to_string(ignore_file)?;
        Ok(content.lines()
            .filter(|l| !l.trim_start().starts_with('#') && !l.trim().is_empty())
            .map(String::from)
            .collect())
    } else {
        Ok(Vec::new())
    }
}