use serde::{Deserialize, Serialize};
use std::{env, error::Error};

#[derive(Serialize, Deserialize, Debug)]
struct MessageContent {
    #[serde(rename = "type")]
    content_type: String,
    text: String,
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
    let key = env::var("AI_KEY").expect("Did not find auth key in environment variable AI_KEY");
    let json = create_payload();
    let client = reqwest::Client::new();

    println!("Key {}", key);

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

fn create_payload() -> Payload {
    Payload {
        messages: vec![
            Message {
                role: "system".to_string(),
                content: vec![MessageContent {
                    content_type: "text".to_string(),
                    text: "You are a story teller who has a flavor for short sci fi stories"
                        .to_string(),
                }],
            },
            Message {
                role: "user".to_string(),
                content: vec![MessageContent {
                    content_type: "text".to_string(),
                    text: "tell me a short story".to_string(),
                }],
            },
        ],
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
