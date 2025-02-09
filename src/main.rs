use image;
use std::{env, fs, path::Path};

use pdfium_render::prelude::*;

fn main() {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    println!("Current working directory: {:?}", current_dir);

    let input_directory = Path::new("input");
    fs::create_dir_all(&input_directory).expect(&format!(
        "Could not create input directory {:?}",
        input_directory
    ));

    let input_directory = fs::read_dir(input_directory).expect("couldn't read input directory");

    for (index, dir_entry_result) in input_directory.enumerate() {
        match dir_entry_result {
            Ok(dir_entry) => {
                println!("{:?}", dir_entry.path());
                let output_directory = Path::new("output").join(index.to_string());
                if fs::exists(&output_directory).unwrap() {
                    fs::remove_dir_all(&output_directory).expect(&format!(
                        "Could not clear output directory {:?}",
                        output_directory
                    ));
                }
                match export_pdf_to_jpegs(&dir_entry.path(), &output_directory, Option::None) {
                    Ok(_) => println!("successfully created images of pfd: {:?}", dir_entry),
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

    println!("pdf path: {}", pdf_path.as_ref().display());

    let document = pdfium.load_pdf_from_file(pdf_path, password)?;
    let render_config = PdfRenderConfig::new();

    for (index, page) in document.pages().iter().enumerate() {
        let filename = pdf_path
            .as_ref()
            .file_stem()
            .expect("no filename")
            .to_str()
            .expect("could not get filename as string");

        page.render_with_config(&render_config)?
            .as_image()
            .into_rgb8()
            .save_with_format(
                output_path
                    .as_ref()
                    .join(format!("{}-{}.png", filename, index)),
                image::ImageFormat::Png,
            )
            .map_err(|e| {
                println!("Error generating image: {}", e);
                PdfiumError::ImageError
            })?;
    }

    Ok(())
}
