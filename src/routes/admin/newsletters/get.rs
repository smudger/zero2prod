use actix_web::{http::header::ContentType, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn send_newsletter_form(
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(format!(
        r#"<!DOCTYPE html>
        <html lang="en">
        <head>
            <meta http-equiv="content-type" content="text/html; charset=utf-8">
            <title>Send Newsletter</title>
        </head>
        <body>
            {msg_html}
            <form action="/admin/newsletters" method="post">
                <label>Title
                    <input type="text" placeholder="Enter newsletter title" name="title">
                </label>
                <br>
                <label>Text content
                    <input type="text" placeholder="Enter newsletter text content" name="text_content">
                </label>
                <br>
                <label>HTML content
                    <input type="text" placeholder="Enter newsletter HTML content" name="html_content">
                </label>
                <br>
                <button type="submit">Send Newsletter</button>
            </form>
            <p><a href="/admin/dashboard">&lt;- Back</a></p>
        </body>
        </html>"#)))
}
