use base64::{prelude::BASE64_STANDARD, Engine};
use image::ImageReader;
use serde::{Deserialize, Serialize};
use std::{env, error::Error, fs, path::Path, vec};

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
    println!("generating comparisons...");

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

    println!("done generating comparisons");

    Ok(res)
}

fn get_images_as_base64(path: &Path) -> Vec<String> {
    let mut base64_images = vec![];
    for dir_entry_result in fs::read_dir(path).unwrap() {
        let dir_entry = dir_entry_result.unwrap();
        let image = ImageReader::open(dir_entry.path())
            .expect("file was not an image")
            .decode()
            .expect("could not decode image");

        let bytes = image.into_bytes();
        let encode = BASE64_STANDARD.encode(&bytes);

        base64_images.push(encode);
    }

    return base64_images;
}

fn create_payload(old_images: &Vec<String>, new_images: &Vec<String>) -> Payload {
    let mut messages = vec![];

    messages.push(Message {
        role: "system".to_string(),
        content: vec![MessageContent {
            content_type: "text".to_string(),
            text: Option::Some(
                r#"
You are responsible for checking financial documents specifically KID documents, to check the latest version of the document against the previous version and come up with a summary of the differences if there are any. These will come as png images representing the pdf, you will get up to 3 png images for the old document called old-0.png, old-1.png etc and up to 3 png images for the new document called new-0.png, new-1.png
 
If the documents are not for the same fund, then please return an error
 
I'd like you to respond with the following format, your summary should include the changes to the values descibed in a simple paragraph, then changes should also be included in the changes array of the json
 
{
"Summary": <Input your summary here>
"Changes":
[
{
"Key": <Make this some descriptive key of the item that changed>
"OldValue": <The value in the old document>
"NewValue": <The value in the new document>
}
]
}
 
unless it's a error, then just a plain text error will do"#.to_string(),
            ),
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
                url: image.to_string(),
            }),
        });
    }

    for image in new_images {
        message_content.push(MessageContent {
            content_type: "image_url".to_string(),
            text: Option::None,
            image_url: Option::Some(ImageUrl {
                url: image.to_string(),
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

// payload="{\"messages\":[{\"role\":\"system\",\"content\":[{\"type\":\"text\",\"text\":\"You are responsible for checking financial documents specifically KID documents, to check the latest version of the document against the previous version and come up with a summary of the differences if there are any. These will come as png images representing the pdf, you will get up to 3 png images for the old document called old-0.png, old-1.png etc and up to 3 png images for the new document called new-0.png, new-1.png\\n \\nIf the documents are not for the same fund, then please return an error\\n \\nI'd like you to respond with the following format, your summary should include the changes to the values descibed in a simple paragraph, then changes should also be included in the changes array of the json\\n \\n{\\n\\\"Summary\\\": <Input your summary here>\\n\\\"Changes\\\":\\n[\\n{\\n\\\"Key\\\": <Make this some descriptive key of the item that changed>\\n\\\"OldValue\\\": <The value in the old document>\\n\\\"NewValue\\\": <The value in the new document>\\n}\\n]\\n}\\n \\nunless it's a error, then just a plain text error will do\"}]}],\"temperature\":0.7,\"top_p\":0.95,\"max_tokens\":800}"
//    curl "https://nick-openai-test-2.openai.azure.com/openai/deployments/gpt-4o/chat/completions?api-version=2024-02-15-preview" \
//   -H "Content-Type: application/json" \
//   -H "api-key: YOUR_API_KEY" \
//   -d "$payload"
