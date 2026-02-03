use actix_web::HttpResponse;
use actix_web::http::header::ContentType;
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn publish_newsletter_form(
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta content="text/html; charset=utf-8" http-equiv="content-type">
    <title>Publish News Letter</title>
</head>
<body>
{msg_html}
<form action="/admin/newsletters" method="post">
    <label>Title:<br>
        <input name="title"
               id="title"
               placeholder="Enter the issue title"
               type="text"
        >
    </label>
    <br>
    <label>Plain text content:<br>
        <textarea cols="50"
                  name="text_content"
                  placeholder="Enter the content in plain text"
                  rows="20"
        ></textarea>
    </label>
    <br>
    <label>HTML content:<br>
        <textarea cols="50"
                  name="html_content"
                  placeholder="Enter the content in HTML format"
                  rows="20"
        ></textarea>
    </label>
    <br>
    <button type="submit">Publish</button>
</form>
<p><a href="/admin/dashboard">&lt;- Back</a></p>
</body>
</html>    
    "#,
        )))
}
