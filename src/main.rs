use image;
use std::{fs, path::Path};

use pdfium_render::prelude::*;

fn main() {
    let input_path = Path::new("input");
    fs::create_dir_all(&input_path)
        .expect(&format!("Could not create input path {:?}", input_path));

    let input_directory = fs::read_dir(input_path).expect("couldn't read input directory");

    for (index, dir_entry_result) in input_directory.enumerate() {
        match dir_entry_result {
            Ok(dir_entry) => {
                println!("{:?}", dir_entry.path());
                let output_directory = Path::new("output").join(index.to_string());
                match export_pdf_to_jpegs(&dir_entry.path(), &output_directory, Option::None) {
                    Ok(_) => println!("success"),
                    Err(err) => println!("failed converting pdf to pngs: {}", err),
                }
            }
            Err(error) => println!("Failed to read directory entry {}", error),
        }
    }
}

fn export_pdf_to_jpegs(
    pdf_path: &impl AsRef<Path>,
    output_path: &impl AsRef<Path>,
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
        page.render_with_config(&render_config)?
            .as_image()
            .into_rgb8()
            .save_with_format(
                output_path
                    .as_ref()
                    .join(format!("test-page-{}.png", index)),
                image::ImageFormat::Png,
            )
            .map_err(|_| PdfiumError::ImageError)?;
    }

    Ok(())
}
