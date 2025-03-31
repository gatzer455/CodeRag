// src/processor.rs

use anyhow::{Context, Result};
use ignore::Walk;
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

pub fn find_code_files(dir: &str) -> Result<Vec<PathBuf>> {

    println!("Scanning directory: {}", dir); //Mensaje de depuración
    let mut code_files = Vec::new();

    for entry in Walk::new(dir) {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();

        if path.is_file() && is_code_file(path) {
            code_files.push(path.to_path_buf());
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