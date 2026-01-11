//! Tests for Webhooks parsing

use wacloudapi::webhooks::{WebhookEvent, WebhookPayload};

#[test]
fn test_parse_text_message_webhook() {
    let payload = r#"{
        "object": "whatsapp_business_account",
        "entry": [{
            "id": "WABA_ID",
            "changes": [{
                "value": {
                    "messaging_product": "whatsapp",
                    "metadata": {
                        "display_phone_number": "628123456789",
                        "phone_number_id": "PHONE_ID"
                    },
                    "contacts": [{
                        "profile": { "name": "John Doe" },
                        "wa_id": "628111222333"
                    }],
                    "messages": [{
                        "from": "628111222333",
                        "id": "wamid.HBgM...",
                        "timestamp": "1704067200",
                        "type": "text",
                        "text": { "body": "Hello, World!" }
                    }]
                },
                "field": "messages"
            }]
        }]
    }"#;

    let webhook: WebhookPayload = serde_json::from_str(payload).unwrap();
    let events = webhook.events();

    assert_eq!(events.len(), 1);
    match &events[0] {
        WebhookEvent::TextMessage {
            from,
            text,
            message_id,
        } => {
            assert_eq!(from, "628111222333");
            assert_eq!(text, "Hello, World!");
            assert_eq!(message_id, "wamid.HBgM...");
        }
        _ => panic!("Expected TextMessage event"),
    }
}

#[test]
fn test_parse_image_message_webhook() {
    let payload = r#"{
        "object": "whatsapp_business_account",
        "entry": [{
            "id": "WABA_ID",
            "changes": [{
                "value": {
                    "messaging_product": "whatsapp",
                    "metadata": {
                        "display_phone_number": "628123456789",
                        "phone_number_id": "PHONE_ID"
                    },
                    "contacts": [{
                        "profile": { "name": "John Doe" },
                        "wa_id": "628111222333"
                    }],
                    "messages": [{
                        "from": "628111222333",
                        "id": "wamid.IMG123",
                        "timestamp": "1704067200",
                        "type": "image",
                        "image": {
                            "id": "media_123",
                            "mime_type": "image/jpeg",
                            "sha256": "abc123",
                            "caption": "Check this out!"
                        }
                    }]
                },
                "field": "messages"
            }]
        }]
    }"#;

    let webhook: WebhookPayload = serde_json::from_str(payload).unwrap();
    let events = webhook.events();

    assert_eq!(events.len(), 1);
    match &events[0] {
        WebhookEvent::ImageMessage {
            from,
            media_id,
            caption,
            ..
        } => {
            assert_eq!(from, "628111222333");
            assert_eq!(media_id, "media_123");
            assert_eq!(caption.as_ref().unwrap(), "Check this out!");
        }
        _ => panic!("Expected ImageMessage event"),
    }
}

#[test]
fn test_parse_button_reply_webhook() {
    let payload = r#"{
        "object": "whatsapp_business_account",
        "entry": [{
            "id": "WABA_ID",
            "changes": [{
                "value": {
                    "messaging_product": "whatsapp",
                    "metadata": {
                        "display_phone_number": "628123456789",
                        "phone_number_id": "PHONE_ID"
                    },
                    "contacts": [{
                        "profile": { "name": "Jane Smith" },
                        "wa_id": "628999888777"
                    }],
                    "messages": [{
                        "from": "628999888777",
                        "id": "wamid.BTN123",
                        "timestamp": "1704067200",
                        "type": "interactive",
                        "interactive": {
                            "type": "button_reply",
                            "button_reply": {
                                "id": "btn_yes",
                                "title": "Yes"
                            }
                        }
                    }]
                },
                "field": "messages"
            }]
        }]
    }"#;

    let webhook: WebhookPayload = serde_json::from_str(payload).unwrap();
    let events = webhook.events();

    assert_eq!(events.len(), 1);
    match &events[0] {
        WebhookEvent::ButtonReply {
            from,
            button_id,
            button_title,
            ..
        } => {
            assert_eq!(from, "628999888777");
            assert_eq!(button_id, "btn_yes");
            assert_eq!(button_title, "Yes");
        }
        _ => panic!("Expected ButtonReply event"),
    }
}

#[test]
fn test_parse_list_reply_webhook() {
    let payload = r#"{
        "object": "whatsapp_business_account",
        "entry": [{
            "id": "WABA_ID",
            "changes": [{
                "value": {
                    "messaging_product": "whatsapp",
                    "metadata": {
                        "display_phone_number": "628123456789",
                        "phone_number_id": "PHONE_ID"
                    },
                    "contacts": [{
                        "profile": { "name": "User" },
                        "wa_id": "628555666777"
                    }],
                    "messages": [{
                        "from": "628555666777",
                        "id": "wamid.LIST123",
                        "timestamp": "1704067200",
                        "type": "interactive",
                        "interactive": {
                            "type": "list_reply",
                            "list_reply": {
                                "id": "prod_001",
                                "title": "Product A",
                                "description": "Premium product"
                            }
                        }
                    }]
                },
                "field": "messages"
            }]
        }]
    }"#;

    let webhook: WebhookPayload = serde_json::from_str(payload).unwrap();
    let events = webhook.events();

    assert_eq!(events.len(), 1);
    match &events[0] {
        WebhookEvent::ListReply {
            from,
            row_id,
            row_title,
            ..
        } => {
            assert_eq!(from, "628555666777");
            assert_eq!(row_id, "prod_001");
            assert_eq!(row_title, "Product A");
        }
        _ => panic!("Expected ListReply event"),
    }
}

#[test]
fn test_parse_message_status_webhook() {
    let payload = r#"{
        "object": "whatsapp_business_account",
        "entry": [{
            "id": "WABA_ID",
            "changes": [{
                "value": {
                    "messaging_product": "whatsapp",
                    "metadata": {
                        "display_phone_number": "628123456789",
                        "phone_number_id": "PHONE_ID"
                    },
                    "statuses": [{
                        "id": "wamid.STATUS123",
                        "status": "delivered",
                        "timestamp": "1704067200",
                        "recipient_id": "628111222333"
                    }]
                },
                "field": "messages"
            }]
        }]
    }"#;

    let webhook: WebhookPayload = serde_json::from_str(payload).unwrap();
    let events = webhook.events();

    assert_eq!(events.len(), 1);
    match &events[0] {
        WebhookEvent::MessageDelivered {
            message_id,
            recipient,
        } => {
            assert_eq!(message_id, "wamid.STATUS123");
            assert_eq!(recipient, "628111222333");
        }
        _ => panic!("Expected MessageDelivered event"),
    }
}

#[test]
fn test_parse_reaction_webhook() {
    let payload = r#"{
        "object": "whatsapp_business_account",
        "entry": [{
            "id": "WABA_ID",
            "changes": [{
                "value": {
                    "messaging_product": "whatsapp",
                    "metadata": {
                        "display_phone_number": "628123456789",
                        "phone_number_id": "PHONE_ID"
                    },
                    "contacts": [{
                        "profile": { "name": "Reactor" },
                        "wa_id": "628444555666"
                    }],
                    "messages": [{
                        "from": "628444555666",
                        "id": "wamid.REACT123",
                        "timestamp": "1704067200",
                        "type": "reaction",
                        "reaction": {
                            "message_id": "wamid.TARGET123",
                            "emoji": "👍"
                        }
                    }]
                },
                "field": "messages"
            }]
        }]
    }"#;

    let webhook: WebhookPayload = serde_json::from_str(payload).unwrap();
    let events = webhook.events();

    assert_eq!(events.len(), 1);
    match &events[0] {
        WebhookEvent::Reaction {
            from,
            message_id,
            emoji,
        } => {
            assert_eq!(from, "628444555666");
            assert_eq!(message_id, "wamid.TARGET123");
            assert_eq!(emoji, "👍");
        }
        _ => panic!("Expected Reaction event"),
    }
}

#[test]
fn test_parse_location_message_webhook() {
    let payload = r#"{
        "object": "whatsapp_business_account",
        "entry": [{
            "id": "WABA_ID",
            "changes": [{
                "value": {
                    "messaging_product": "whatsapp",
                    "metadata": {
                        "display_phone_number": "628123456789",
                        "phone_number_id": "PHONE_ID"
                    },
                    "contacts": [{
                        "profile": { "name": "Traveler" },
                        "wa_id": "628777888999"
                    }],
                    "messages": [{
                        "from": "628777888999",
                        "id": "wamid.LOC123",
                        "timestamp": "1704067200",
                        "type": "location",
                        "location": {
                            "latitude": -6.2088,
                            "longitude": 106.8456,
                            "name": "Jakarta",
                            "address": "Indonesia"
                        }
                    }]
                },
                "field": "messages"
            }]
        }]
    }"#;

    let webhook: WebhookPayload = serde_json::from_str(payload).unwrap();
    let events = webhook.events();

    assert_eq!(events.len(), 1);
    match &events[0] {
        WebhookEvent::LocationMessage {
            from,
            latitude,
            longitude,
            message_id,
        } => {
            assert_eq!(from, "628777888999");
            assert!((*latitude - -6.2088).abs() < 0.0001);
            assert!((*longitude - 106.8456).abs() < 0.0001);
            assert_eq!(message_id, "wamid.LOC123");
        }
        _ => panic!("Expected LocationMessage event"),
    }
}

#[test]
fn test_parse_video_message_webhook() {
    let payload = r#"{
        "object": "whatsapp_business_account",
        "entry": [{
            "id": "WABA_ID",
            "changes": [{
                "value": {
                    "messaging_product": "whatsapp",
                    "metadata": {
                        "display_phone_number": "628123456789",
                        "phone_number_id": "PHONE_ID"
                    },
                    "contacts": [{
                        "profile": { "name": "Video Sender" },
                        "wa_id": "628333444555"
                    }],
                    "messages": [{
                        "from": "628333444555",
                        "id": "wamid.VID123",
                        "timestamp": "1704067200",
                        "type": "video",
                        "video": {
                            "id": "video_media_123",
                            "mime_type": "video/mp4",
                            "sha256": "xyz789"
                        }
                    }]
                },
                "field": "messages"
            }]
        }]
    }"#;

    let webhook: WebhookPayload = serde_json::from_str(payload).unwrap();
    let events = webhook.events();

    assert_eq!(events.len(), 1);
    match &events[0] {
        WebhookEvent::VideoMessage {
            from,
            media_id,
            message_id,
            ..
        } => {
            assert_eq!(from, "628333444555");
            assert_eq!(media_id, "video_media_123");
            assert_eq!(message_id, "wamid.VID123");
        }
        _ => panic!("Expected VideoMessage event"),
    }
}

#[test]
fn test_empty_webhook() {
    let payload = r#"{
        "object": "whatsapp_business_account",
        "entry": [{
            "id": "WABA_ID",
            "changes": [{
                "value": {
                    "messaging_product": "whatsapp",
                    "metadata": {
                        "display_phone_number": "628123456789",
                        "phone_number_id": "PHONE_ID"
                    }
                },
                "field": "messages"
            }]
        }]
    }"#;

    let webhook: WebhookPayload = serde_json::from_str(payload).unwrap();
    let events = webhook.events();

    assert!(events.is_empty());
}
