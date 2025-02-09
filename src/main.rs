use image;
use std::{fs, path::Path};

use pdfium_render::prelude::*;

const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

fn main() {
    let input_dir_new = Path::new("input/new");
    fs::create_dir_all(&input_dir_new).unwrap();

    let input_dir_old = Path::new("input/old");
    fs::create_dir_all(&input_dir_old).unwrap();

    for dir_entry_result in fs::read_dir(input_dir_new).unwrap() {
        match dir_entry_result {
            Ok(dir_entry) => {
                let output_dir = Path::new("output/new");
                clear_output_dir(&output_dir);
                match export_pdf_to_jpegs(
                    &dir_entry.path(),
                    &output_dir,
                    String::from("new"),
                    Option::None,
                ) {
                    Ok(_) => println!("{}Success{}", GREEN, RESET),
                    Err(err) => println!("failed converting pdf to pngs: {}", err),
                }
            }
            Err(error) => println!("Failed to read directory entry {}", error),
        }
    }

    for dir_entry_result in fs::read_dir(input_dir_old).unwrap() {
        match dir_entry_result {
            Ok(dir_entry) => {
                let output_dir = Path::new("output/old");
                clear_output_dir(&output_dir);
                match export_pdf_to_jpegs(
                    &dir_entry.path(),
                    &output_dir,
                    String::from("old"),
                    Option::None,
                ) {
                    Ok(_) => println!("{}Success{}", GREEN, RESET),
                    Err(err) => println!("failed converting pdf to pngs: {}", err),
                }
            }
            Err(error) => println!("Failed to read directory entry {}", error),
        }
    }
}

fn clear_output_dir(output_directory: &std::path::Path) {
    if fs::exists(&output_directory).unwrap() {
        fs::remove_dir_all(&output_directory).expect(&format!(
            "Could not clear output directory {:?}",
            output_directory
        ));
    }
}

fn export_pdf_to_jpegs(
    pdf_path: &impl AsRef<Path>,
    output_path: &impl AsRef<Path>,
    output_name: String,
    password: Option<&str>,
) -> Result<(), PdfiumError> {
    fs::create_dir_all(&output_path).expect(&format!(
        "Could not create path {}",
        output_path.as_ref().display()
    ));

    let pdfium = Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./")).unwrap(),
    );

    let document = pdfium.load_pdf_from_file(pdf_path, password)?;
    let render_config = PdfRenderConfig::new();

    for (index, page) in document.pages().iter().enumerate() {
        println!(
            "Generating image {} for {}",
            index + 1,
            pdf_path.as_ref().display()
        );
        page.render_with_config(&render_config)?
            .as_image()
            .into_rgb8()
            .save_with_format(
                output_path
                    .as_ref()
                    .join(format!("{}-{}.png", output_name, index)),
                image::ImageFormat::Png,
            )
            .map_err(|e| {
                println!("Error generating image: {}", e);
                PdfiumError::ImageError
            })?;
    }

    Ok(())
}
