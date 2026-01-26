use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, web};

#[derive(serde::Deserialize)]
pub struct QueryParams {
    error: Option<String>,
}

pub async fn login_form(query: web::Query<QueryParams>) -> HttpResponse {
    let error_html = match query.0.error {
        None => "".into(),
        Some(error_message) => format!(
            "<p><i>{}</i></p>",
            htmlescape::encode_minimal(&error_message)
        ),
    };
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>Login</title>
</head>
<body>
    {error_html}
    <form action="/login" method="post">
        <label>Username&nbsp;
            <input type="text" placeholder="Enter Username" name="username" autocomplete="username" required>
        </label>
        <label>Password&nbsp;
            <input type="password" placeholder="Enter Password" name="password" autocomplete="current-password" required>
        </label>
        <button type="submit">Login</button>
    </form>
</body>
</html>"#
        ))
}
