# CodeRag  
**Herramienta CLI en Rust** para indexar codebases en fragmentos estructurados, optimizados para RAG (Retrieval-Augmented Generation).  

- **Problema que resuelve**:  
  - Los LLMs pierden contexto en codebases grandes.  
  - Procesamiento manual de código es lento y no escalable.  

- **Solución**:  
  - Automatiza la extracción, chunking y serialización de código.  
  - Metadatos enriquecidos (lenguaje, ubicación, función).  
  - Compatible con motores de embeddings (FAISS, Chroma).  

- **Diferencial**:  
  - Velocidad de Rust + precisión en chunking (AST-aware).  
