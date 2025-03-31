use assert_cmd::Command;
use std::path::Path;
use coderag::processor; 


#[test]
fn test_cli_exclusions() {
    // Setup: Crear estructura de prueba
    let test_dir = "test_exclude_cli";
    let test_file = Path::new(test_dir).join("ignore.tmp");
    
    std::fs::create_dir_all(test_dir).unwrap();
    std::fs::write(&test_file, "content").unwrap();

    // Ejecutar comando
    let mut cmd = Command::cargo_bin("coderag").unwrap();
    let assert = cmd.args(&[
        "index", 
        "-i", test_dir, 
        "--exclude", "*.tmp"  // Cambiado a un solo patrón
    ]).assert();
    
    // Verificar
    assert.success();
    
    // Opcional: Verificar que el archivo fue excluido
    let output = std::fs::read_to_string("output.json").unwrap();
    assert!(!output.contains("ignore.tmp"));
    
    // Limpieza
    std::fs::remove_dir_all(test_dir).unwrap();
    let _ = std::fs::remove_file("output.json");
}


#[test]
fn test_ignore_file() {
    // Crear estructura de test
    let test_dir = "test_data";
    std::fs::create_dir_all(test_dir).unwrap();
    std::fs::write(Path::new(test_dir).join(".coderagignore"), "*.bak\n# Comentario\ntemp/").unwrap();
    
    // Crear archivo que debería ser ignorado
    std::fs::write(Path::new(test_dir).join("test.bak"), "content").unwrap();

    let files = processor::find_code_files(test_dir, &[]).unwrap();
    assert!(!files.iter().any(|p| p.to_string_lossy().contains("test.bak")));
    
    // Limpieza (opcional)
    std::fs::remove_dir_all(test_dir).unwrap();
}