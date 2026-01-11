//! Tests for Messages API

mod common;

use common::*;
use wacloudapi::messages::{
    Button, Contact, ContactName, ContactPhone, ListRow, ListSection, TemplateComponent,
    TemplateParameter,
};
use wiremock::matchers::{body_json, header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_send_text() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/messages", TEST_PHONE_ID)))
        .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
        .and(body_json(serde_json::json!({
            "messaging_product": "whatsapp",
            "recipient_type": "individual",
            "to": "628123456789",
            "type": "text",
            "text": {
                "preview_url": false,
                "body": "Hello, World!"
            }
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(message_response("wamid.test123")))
        .mount(&mock_server)
        .await;

    let response = client
        .messages()
        .send_text("628123456789", "Hello, World!")
        .await
        .unwrap();

    assert_eq!(response.messages[0].id, "wamid.test123");
}

#[tokio::test]
async fn test_send_text_with_preview() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/messages", TEST_PHONE_ID)))
        .and(body_json(serde_json::json!({
            "messaging_product": "whatsapp",
            "recipient_type": "individual",
            "to": "628123456789",
            "type": "text",
            "text": {
                "preview_url": true,
                "body": "Check this: https://example.com"
            }
        })))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(message_response("wamid.preview123")),
        )
        .mount(&mock_server)
        .await;

    let response = client
        .messages()
        .send_text_with_preview("628123456789", "Check this: https://example.com")
        .await
        .unwrap();

    assert_eq!(response.messages[0].id, "wamid.preview123");
}

#[tokio::test]
async fn test_send_reply() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/messages", TEST_PHONE_ID)))
        .and(body_json(serde_json::json!({
            "messaging_product": "whatsapp",
            "recipient_type": "individual",
            "to": "628123456789",
            "context": {
                "message_id": "wamid.original123"
            },
            "type": "text",
            "text": {
                "preview_url": false,
                "body": "This is a reply"
            }
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(message_response("wamid.reply123")))
        .mount(&mock_server)
        .await;

    let response = client
        .messages()
        .send_reply("628123456789", "This is a reply", "wamid.original123")
        .await
        .unwrap();

    assert_eq!(response.messages[0].id, "wamid.reply123");
}

#[tokio::test]
async fn test_send_reaction() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/messages", TEST_PHONE_ID)))
        .and(body_json(serde_json::json!({
            "messaging_product": "whatsapp",
            "recipient_type": "individual",
            "to": "628123456789",
            "type": "reaction",
            "reaction": {
                "message_id": "wamid.target123",
                "emoji": "👍"
            }
        })))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(message_response("wamid.reaction123")),
        )
        .mount(&mock_server)
        .await;

    let response = client
        .messages()
        .send_reaction("628123456789", "wamid.target123", "👍")
        .await
        .unwrap();

    assert_eq!(response.messages[0].id, "wamid.reaction123");
}

#[tokio::test]
async fn test_remove_reaction() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/messages", TEST_PHONE_ID)))
        .and(body_json(serde_json::json!({
            "messaging_product": "whatsapp",
            "recipient_type": "individual",
            "to": "628123456789",
            "type": "reaction",
            "reaction": {
                "message_id": "wamid.target123",
                "emoji": ""
            }
        })))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(message_response("wamid.unreact123")),
        )
        .mount(&mock_server)
        .await;

    let response = client
        .messages()
        .remove_reaction("628123456789", "wamid.target123")
        .await
        .unwrap();

    assert_eq!(response.messages[0].id, "wamid.unreact123");
}

#[tokio::test]
async fn test_send_image_url() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/messages", TEST_PHONE_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(message_response("wamid.image123")))
        .mount(&mock_server)
        .await;

    let response = client
        .messages()
        .send_image_url(
            "628123456789",
            "https://example.com/image.jpg",
            Some("Nice image"),
        )
        .await
        .unwrap();

    assert_eq!(response.messages[0].id, "wamid.image123");
}

#[tokio::test]
async fn test_send_image_id() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/messages", TEST_PHONE_ID)))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(message_response("wamid.imageid123")),
        )
        .mount(&mock_server)
        .await;

    let response = client
        .messages()
        .send_image_id("628123456789", "media_id_123", Some("Caption"))
        .await
        .unwrap();

    assert_eq!(response.messages[0].id, "wamid.imageid123");
}

#[tokio::test]
async fn test_send_video_url() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/messages", TEST_PHONE_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(message_response("wamid.video123")))
        .mount(&mock_server)
        .await;

    let response = client
        .messages()
        .send_video_url(
            "628123456789",
            "https://example.com/video.mp4",
            Some("Watch this!"),
        )
        .await
        .unwrap();

    assert_eq!(response.messages[0].id, "wamid.video123");
}

#[tokio::test]
async fn test_send_video_id() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/messages", TEST_PHONE_ID)))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(message_response("wamid.videoid123")),
        )
        .mount(&mock_server)
        .await;

    let response = client
        .messages()
        .send_video_id("628123456789", "video_media_id", Some("Video"))
        .await
        .unwrap();

    assert_eq!(response.messages[0].id, "wamid.videoid123");
}

#[tokio::test]
async fn test_send_audio_url() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/messages", TEST_PHONE_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(message_response("wamid.audio123")))
        .mount(&mock_server)
        .await;

    let response = client
        .messages()
        .send_audio_url("628123456789", "https://example.com/audio.mp3")
        .await
        .unwrap();

    assert_eq!(response.messages[0].id, "wamid.audio123");
}

#[tokio::test]
async fn test_send_audio_id() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/messages", TEST_PHONE_ID)))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(message_response("wamid.audioid123")),
        )
        .mount(&mock_server)
        .await;

    let response = client
        .messages()
        .send_audio_id("628123456789", "audio_media_id")
        .await
        .unwrap();

    assert_eq!(response.messages[0].id, "wamid.audioid123");
}

#[tokio::test]
async fn test_send_document_url() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/messages", TEST_PHONE_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(message_response("wamid.doc123")))
        .mount(&mock_server)
        .await;

    let response = client
        .messages()
        .send_document_url(
            "628123456789",
            "https://example.com/doc.pdf",
            Some("report.pdf"),
            Some("Here's the report"),
        )
        .await
        .unwrap();

    assert_eq!(response.messages[0].id, "wamid.doc123");
}

#[tokio::test]
async fn test_send_document_id() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/messages", TEST_PHONE_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(message_response("wamid.docid123")))
        .mount(&mock_server)
        .await;

    let response = client
        .messages()
        .send_document_id(
            "628123456789",
            "doc_media_id",
            Some("file.pdf"),
            Some("Doc"),
        )
        .await
        .unwrap();

    assert_eq!(response.messages[0].id, "wamid.docid123");
}

#[tokio::test]
async fn test_send_sticker_url() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/messages", TEST_PHONE_ID)))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(message_response("wamid.sticker123")),
        )
        .mount(&mock_server)
        .await;

    let response = client
        .messages()
        .send_sticker_url("628123456789", "https://example.com/sticker.webp")
        .await
        .unwrap();

    assert_eq!(response.messages[0].id, "wamid.sticker123");
}

#[tokio::test]
async fn test_send_sticker_id() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/messages", TEST_PHONE_ID)))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(message_response("wamid.stickerid123")),
        )
        .mount(&mock_server)
        .await;

    let response = client
        .messages()
        .send_sticker_id("628123456789", "sticker_media_id")
        .await
        .unwrap();

    assert_eq!(response.messages[0].id, "wamid.stickerid123");
}

#[tokio::test]
async fn test_send_location() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/messages", TEST_PHONE_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(message_response("wamid.loc123")))
        .mount(&mock_server)
        .await;

    let response = client
        .messages()
        .send_location(
            "628123456789",
            -6.2088,
            106.8456,
            Some("Jakarta"),
            Some("Indonesia"),
        )
        .await
        .unwrap();

    assert_eq!(response.messages[0].id, "wamid.loc123");
}

#[tokio::test]
async fn test_send_contacts() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/messages", TEST_PHONE_ID)))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(message_response("wamid.contact123")),
        )
        .mount(&mock_server)
        .await;

    let contacts = vec![Contact {
        name: ContactName {
            formatted_name: "John Doe".to_string(),
            first_name: Some("John".to_string()),
            last_name: Some("Doe".to_string()),
            middle_name: None,
            suffix: None,
            prefix: None,
        },
        phones: Some(vec![ContactPhone {
            phone: "+1234567890".to_string(),
            phone_type: Some("MOBILE".to_string()),
            wa_id: None,
        }]),
        emails: None,
        urls: None,
        addresses: None,
        org: None,
        birthday: None,
    }];

    let response = client
        .messages()
        .send_contacts("628123456789", contacts)
        .await
        .unwrap();

    assert_eq!(response.messages[0].id, "wamid.contact123");
}

#[tokio::test]
async fn test_send_template() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/messages", TEST_PHONE_ID)))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(message_response("wamid.template123")),
        )
        .mount(&mock_server)
        .await;

    let components = vec![TemplateComponent {
        component_type: "body".to_string(),
        sub_type: None,
        index: None,
        parameters: Some(vec![TemplateParameter {
            param_type: "text".to_string(),
            text: Some("John".to_string()),
            currency: None,
            date_time: None,
            image: None,
            document: None,
            video: None,
        }]),
    }];

    let response = client
        .messages()
        .send_template("628123456789", "hello_world", "en_US", Some(components))
        .await
        .unwrap();

    assert_eq!(response.messages[0].id, "wamid.template123");
}

#[tokio::test]
async fn test_send_list() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/messages", TEST_PHONE_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(message_response("wamid.list123")))
        .mount(&mock_server)
        .await;

    let sections = vec![ListSection {
        title: "Products".to_string(),
        rows: vec![
            ListRow::new("prod_1", "Product 1").with_description("$10.00"),
            ListRow::new("prod_2", "Product 2").with_description("$20.00"),
        ],
    }];

    let response = client
        .messages()
        .send_list(
            "628123456789",
            Some("Our Products"),
            "Choose a product",
            Some("Powered by WA"),
            "View Products",
            sections,
        )
        .await
        .unwrap();

    assert_eq!(response.messages[0].id, "wamid.list123");
}

#[tokio::test]
async fn test_send_buttons() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/messages", TEST_PHONE_ID)))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(message_response("wamid.buttons123")),
        )
        .mount(&mock_server)
        .await;

    let buttons = vec![
        Button::reply("btn_yes", "Yes"),
        Button::reply("btn_no", "No"),
        Button::reply("btn_maybe", "Maybe"),
    ];

    let response = client
        .messages()
        .send_buttons(
            "628123456789",
            Some("Question"),
            "Do you want to proceed?",
            Some("Tap a button"),
            buttons,
        )
        .await
        .unwrap();

    assert_eq!(response.messages[0].id, "wamid.buttons123");
}

#[tokio::test]
async fn test_mark_as_read() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server);

    Mock::given(method("POST"))
        .and(path(format!("/v21.0/{}/messages", TEST_PHONE_ID)))
        .and(body_json(serde_json::json!({
            "messaging_product": "whatsapp",
            "status": "read",
            "message_id": "wamid.toread123"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(success_response()))
        .mount(&mock_server)
        .await;

    let response = client
        .messages()
        .mark_as_read("wamid.toread123")
        .await
        .unwrap();

    assert!(response.success);
}
