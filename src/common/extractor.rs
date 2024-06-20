use axum::{
    async_trait,
    extract::{
        rejection::{FormRejection, JsonRejection},
        FromRequest, Request,
    },
    http::header::CONTENT_TYPE,
    Form, Json,
};

use super::error::ApiError;

pub struct JsonOrForm<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for JsonOrForm<T>
where
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
    Form<T>: FromRequest<S, Rejection = FormRejection>,
    T: 'static,
{
    type Rejection = ApiError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let (parts, body) = req.into_parts();
        let content_type = parts
            .headers
            .get(CONTENT_TYPE)
            .and_then(|value| value.to_str().ok().map(|s| s.to_owned()));

        let req = Request::from_parts(parts, body);

        if let Some(content_type) = content_type {
            if content_type.starts_with("application/json") {
                match Json::<T>::from_request(req, state).await {
                    Ok(value) => return Ok(Self(value.0)),
                    Err(rejection) => {
                        return Err(ApiError::new_validation_error(vec![format!(
                            "{:?}",
                            rejection.body_text()
                        )]))
                    }
                }
            } else if content_type.starts_with("application/x-www-form-urlencoded") {
                match Form::<T>::from_request(req, state).await {
                    Ok(value) => return Ok(Self(value.0)),
                    Err(rejection) => {
                        return Err(ApiError::new_validation_error(vec![format!(
                            "{:?}",
                            rejection.body_text()
                        )]))
                    }
                }
            }
        }

        Err(ApiError::new_unprocessable_entity(
            "Resource not valid".into(),
        ))
    }
}
// async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
//     let content_type_header = req.headers().get(CONTENT_TYPE);
//     let content_type = content_type_header.and_then(|value| value.to_str().ok());

//     if let Some(content_type) = content_type {
//         if content_type.starts_with("application/json") {
//             let Json(payload) = req
//                 .extract()
//                 .await
//                 .map_err(|_| ApiError::new_validation_error(vec![format!("Invalid JSON")]))?;

//             return Ok(Self(payload));
//         } else {
//             let Form(payload) = req
//                 .extract()
//                 .await
//                 .map_err(|_| ApiError::new_validation_error(vec![format!("Invalid JSON")]))?;

//             return Ok(Self(payload));
//         }
//     }

//     Err(ApiError::new_unprocessable_entity(
//         "Resource not valid".into(),
//     ))
// }
