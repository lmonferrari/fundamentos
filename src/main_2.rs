use std::fs;
use std::io::{self, Read, Write};

fn cria_arquivo(filename: &str, content: &str) -> io::Result<()> {
    let mut file = fs::File::create(filename)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn le_arquivo(filename: &str) -> io::Result<String> {
    let mut file = fs::File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn deleta_arquivo(filename: &str) -> io::Result<()> {
    fs::remove_file(filename)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = "src/arquivo.txt";
    let content = "Conteúdo do arquivo";
    cria_arquivo(filename, content)?;

    let file = le_arquivo(filename);
    println!("O conteudo do arquivo é: {:?}", file);

    deleta_arquivo(filename)?;
    Ok(())
}
