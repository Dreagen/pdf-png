use base64::{prelude::BASE64_STANDARD, Engine};
use image::ImageReader;
use serde::{Deserialize, Serialize};
use serde_json::{to_string, Value};
use std::{
    env,
    error::Error,
    fs::{self, File},
    io::{Cursor, Read},
    path::Path,
    vec,
};

#[derive(Serialize, Deserialize, Debug)]
struct ImageUrl {
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct MessageContent {
    #[serde(rename = "type")]
    content_type: String,
    text: Option<String>,
    image_url: Option<ImageUrl>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: String,
    content: Vec<MessageContent>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Payload {
    messages: Vec<Message>,
    temperature: f64,
    top_p: f64,
    max_tokens: u32,
}

pub async fn generate_comparisons() -> Result<String, Box<dyn Error>> {
    // println!("generating comparisons...");

    let old_images = get_images_as_base64(Path::new("output/old"));
    let new_images = get_images_as_base64(Path::new("output/new"));

    let json = create_payload(&old_images, &new_images);

    let key = env::var("AI_KEY").expect("Did not find auth key in environment variable AI_KEY");
    let client = reqwest::Client::new();
    let res = client.post("https://nick-openai-test-2.openai.azure.com/openai/deployments/gpt-4o/chat/completions?api-version=2024-02-15-preview")
        .header("Context-Type", "application/json")
        .header("api-key", format!("{}", key))
        .json(&json)
        .send()
        .await?
        .text()
        .await?;

    // println!("done generating comparisons");

    let v: Value = serde_json::from_str(&res)?;
    let v_clone = v.clone();

    if let Some(content) = v_clone["choices"][0]["message"]["content"].as_str() {
        return Ok(to_string(content)?);
    } else {
        panic!("Could not get content from response");
    }
}

fn get_images_as_base64(path: &Path) -> Vec<String> {
    let mut base64_images = vec![];
    for dir_entry_result in fs::read_dir(path).unwrap() {
        let dir_entry = dir_entry_result.unwrap();
        let image_path = dir_entry.path();
        let image = ImageReader::open(&image_path)
            .expect("file was not an image")
            .decode()
            .expect("could not decode image");

        let mut bytes = Cursor::new(Vec::new());
        image
            .write_to(&mut bytes, image::ImageFormat::Png)
            .expect("could not write image to bytes");
        let base64_encoded_data = BASE64_STANDARD.encode(&bytes.into_inner());

        base64_images.push(base64_encoded_data);
    }

    base64_images
}

fn create_payload(old_images: &Vec<String>, new_images: &Vec<String>) -> Payload {
    let mut messages = vec![];

    messages.push(Message {
        role: "system".to_string(),
        content: vec![MessageContent {
            content_type: "text".to_string(),
            text: Option::Some(get_prompt()),
            image_url: Option::None,
        }],
    });

    let mut message_content = vec![];
    message_content.push(MessageContent {
        content_type: "text".to_string(),
        text: Option::Some("Compare these images".to_string()),
        image_url: Option::None,
    });

    for image in old_images {
        message_content.push(MessageContent {
            content_type: "image_url".to_string(),
            text: Option::None,
            image_url: Option::Some(ImageUrl {
                url: format!("data:image/png;base64,{}", image.to_string()),
            }),
        });
    }

    for image in new_images {
        message_content.push(MessageContent {
            content_type: "image_url".to_string(),
            text: Option::None,
            image_url: Option::Some(ImageUrl {
                url: format!("data:image/png;base64,{}", image.to_string()),
            }),
        });
    }

    messages.push(Message {
        role: "user".to_string(),
        content: message_content,
    });

    Payload {
        messages,
        temperature: 0.7,
        top_p: 0.95,
        max_tokens: 800,
    }
}

fn get_prompt() -> String {
    let mut file = File::open("prompt.txt").expect("Please create prompt.txt in root directory");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}
