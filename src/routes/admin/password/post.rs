use crate::session_state::TypedSession;
use crate::utils::{e500, see_other};
use actix_web::{HttpResponse, web};
use secrecy::SecretString;

#[derive(serde::Deserialize)]
pub struct FormData {
    current_password: SecretString,
    new_password: SecretString,
    confirm_new_password: SecretString,
}

pub async fn change_password(
    _form: web::Form<FormData>,
    session: TypedSession,
) -> Result<HttpResponse, actix_web::Error> {
    if session.get_user_id().map_err(e500)?.is_none() {
        return Ok(see_other("/login"));
    }
    todo!()
}
