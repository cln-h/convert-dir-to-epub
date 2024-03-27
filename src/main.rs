use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <pdf-directory> <epub-directory>", args[0]);
        return;
    }

    let pdf_dir = Path::new(&args[1]);
    let epub_dir = Path::new(&args[2]);

    println!("Converting...");

    fs::create_dir_all(&epub_dir).expect("Failed to create epub directory");

    let pdf_files: Vec<_> = fs::read_dir(&pdf_dir)
        .expect("Failed to read pdf directory")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            if let Some(ext) = path.extension() {
                if ext == "pdf" {
                    true
                } else {
                    eprintln!("Skipping non-PDF file: {:?}", path);
                    false
                }
            } else {
                eprintln!("Skipping file with no extension: {:?}", path);
                false
            }
        })
        .collect();

    println!();
    if pdf_files.is_empty() {
        println!("No PDF files found in {:?}", pdf_dir);
        return;
    }


    for pdf_file in pdf_files {
        let output_file = pdf_file
            .file_stem()
            .expect("Failed to get file stem")
            .to_string_lossy()
            .to_string()
            + ".epub";
        let output_path = epub_dir.join(&output_file);

        let mut command = Command::new("ebook-convert");
        command
            .arg(pdf_file.clone())
            .arg(output_path)
            .arg("--keep-ligatures")
            .arg("--pretty-print")
            .arg("--enable-heuristics");

        let status = command.status().expect("Failed to convert");

        if status.success() {
            println!("Converted {:?} to {:?}", pdf_file, output_file);
        } else {
            eprintln!("Conversion failed for {:?}", pdf_file);
        }
    }
}
