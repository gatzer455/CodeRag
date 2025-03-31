use clap::{Parser, Subcommand};
use anyhow::Result;
use std::fs::File;
use std::io::Write;

mod processor;

//Define los subcomandos (index, search, etc)
#[derive(Debug, Subcommand)]
enum Command {
    /// Indexa un directorio de código y genera un archivo estructurado
    Index {
        #[arg(short, long)]
        input_dir: String,
        #[arg(short, long, default_value = "output.json")]
        output_file: String,
        #[arg(long = "exclude", value_delimiter = ',')]
        exclude_patterns: Vec<String>,
    },
    // (Más comandos se añadirán luego, como `search` o `preprocess`)
}

// Estructura principal del CLI
#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Index {
            input_dir,
            output_file,
            exclude_patterns,
        } => {
            print!("[+] Indexing codebase in: {}", input_dir);
            let code_files = processor::find_code_files(&input_dir, &exclude_patterns)?;
            println!("[+] Found {} code files", code_files.len());
            let chunks = processor::extract_code_chunks(code_files)?; // Se Muestra la barra de progreso aquí en la función misma

            // Serializa a JSON y guarda en el archivo
            let json_data = serde_json::to_string_pretty(&chunks)?;
            let mut file = File::create(&output_file)?;
            file.write_all(json_data.as_bytes())?;

            println!("[✔] Extracted {} chunks", chunks.len());
            println!("[✔] Successfully saved to: {}", output_file)
        }
    }
    Ok(())
}