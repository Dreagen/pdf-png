use dotenv::dotenv;
use std::env;

mod ai;
mod pdf_generation;

fn main() {
    dotenv().ok();
    let key = env::var("AI_KEY").expect("Did not find auth key in environment variable AI_KEY");
    let url =
        env::var("AI_ENDPOINT").expect("Did not find endpoint in environment variable AI_ENDPOINT");
    let deployment_name = env::var("AI_DEPLOYMENT_NAME")
        .expect("Did not find ai deployment name in environment variable AI_DEPLOYMENT_NAME");

    println!("Key {}", key);
    println!("Url {}", url);
    println!("Deployment name {}", deployment_name);
    println!();

    pdf_generation::convert_pdfs_to_pngs();
    ai::generate_comparisons();
}
