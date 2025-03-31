
# CodeRag 🦀

[![License: MPL 2.0]](https://img.shields.io/badge/License-MPL%202.0-brightgreen.svg)


> **Tu codebase, indexada y lista para Retrieval-Augmented Generation (RAG) en segundos.**

**CodeRag** es una herramienta CLI rápida y eficiente escrita en Rust, diseñada para escanear directorios de proyectos, extraer código fuente, documentación y archivos de configuración, y estructurarlos en un formato optimizado para alimentar pipelines de RAG con LLMs.

---

## ¿Qué Problema Resuelve?

Los Large Language Models (LLMs) a menudo tienen dificultades para comprender el contexto completo de codebases grandes debido a las limitaciones de su ventana de contexto. Preparar manualmente estos datos es tedioso y no escala bien.

**CodeRag** automatiza este proceso, permitiendo a los LLMs tener una "visión" más estructurada y relevante del código fuente del proyecto.

---

## ✨ Features (MVP Actual)

*   **Interfaz CLI Simple:** Un comando `index` fácil de usar para iniciar el proceso.
*   **Escaneo Rápido de Directorios:** Utiliza `ignore` para respetar las reglas de `.gitignore` (si existe) y recorrer eficientemente el proyecto.
*   **Detección de Lenguajes/Archivos:** Identifica archivos comunes de código (`.rs`, `.py`, `.js`, etc.), configuración (`.json`, `.toml`, `.yaml`) y documentación (`.md`).
*   **Extracción de Contenido:** Lee el contenido de los archivos detectados.
*   **Barra de Progreso Visual:** Muestra el progreso del escaneo y procesamiento con `indicatif`.
*   **Chunking Básico de Markdown:** Divide archivos `.md` en fragmentos basados en encabezados (`#`, `##`, etc.) para mantener la coherencia temática.
*   **Salida Estructurada en JSON:** Genera un archivo JSON que contiene un array de "chunks", cada uno con:
    *   `file_path`: Ruta relativa del archivo.
    *   `language`: Lenguaje o tipo de archivo detectado.
    *   `content`: Contenido del archivo (o fragmento, en el caso de Markdown).

---

## 🚀 Instalación

Puedes instalar `CodeRag` de las siguientes maneras:

**1. Desde GitHub (Recomendado por ahora):**
Necesitas tener Rust y Cargo instalados.

```bash
   cargo install --git https://github.com/gatzer455/CodeRag.git
```
(Reemplaza TU_USUARIO con tu nombre de usuario de GitHub)

2. Compilando desde Fuente:


```bash
# Clona el repositorio
git clone https://github.com/gatzer455/CodeRag.git
cd CodeRag

# Compila para producción (optimizado)
cargo build --release

# El ejecutable estará en ./target/release/coderag
# Puedes copiarlo a una ubicación en tu $PATH, como /usr/local/bin
# cp ./target/release/coderag /usr/local/bin/
```

K>Uso

El comando principal es index. Necesita saber qué directorio escanear y (opcionalmente) dónde guardar el archivo JSON resultante.


```bash
coderag index --input-dir <RUTA_AL_PROYECTO> [--output-file <NOMBRE_ARCHIVO_SALIDA.json>]

Ejemplos:


```bash
# Indexar el directorio actual y guardar en output.json (por defecto)
coderag index -i .

# Indexar un proyecto específico y guardar en un archivo llamado project_data.json
coderag index --input-dir /ruta/a/mi/proyecto --output-file project_data.json

# Usar la ruta completa al ejecutable si no está en el PATH
/ruta/completa/a/coderag index -i .
```

📄 Formato de Salida (JSON)
El archivo de salida (output.json por defecto) contiene un array JSON. Cada elemento del array es un objeto CodeChunk con la siguiente estructura:


```json
[
  {
    "file_path": "src/main.rs",
    "language": "Rust",
    "content": "fn main() {\n    println!(\"Hello, world!\");\n}"
  },
  {
    "file_path": "README.md",
    "language": "Markdown",
    "content": "CodeRag\n Tu codebase, indexada..." // (Puede ser un fragmento si el archivo es grande)
  },
  // ... más chunks
]
```
Este formato está diseñado para ser fácilmente parseado y utilizado para generar embeddings o alimentar directamente a un LLM dentro de un pipeline RAG.

🗺️ Roadmap (Próximos Pasos)
- Chunking Inteligente: Usar tree-sitter para dividir archivos de código por funciones, clases o bloques lógicos.
- Configuración de Exclusiones: Permitir ignorar archivos/directorios específicos mediante un archivo de configuración o flags.
- Tests: Añadir tests unitarios y de integración.
- Comando search: Implementar funcionalidad para buscar en el índice generado (potencialmente usando embeddings).
- Publicación en Crates.io: Hacer que la instalación sea más sencilla con cargo install coderag.


🤝 Contribuciones
¡Las contribuciones son bienvenidas! Si tienes ideas, encuentras bugs o quieres añadir nuevas funcionalidades, por favor abre un Issue o envía un Pull Request.