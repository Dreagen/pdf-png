use dotenv::dotenv;
use std::error::Error;

mod ai;
mod pdf_generation;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    pdf_generation::convert_pdfs_to_pngs();
    let generate_comparisons_result = ai::generate_comparisons().await?;

    println!("{}", generate_comparisons_result);

    Ok(())
}
