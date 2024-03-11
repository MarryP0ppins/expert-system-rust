use super::{
    answer::Answer,
    attribute::AttributeWithAttributeValues,
    attribute_value::AttributeValue,
    clause::Clause,
    error::CustomErrors,
    history::HistoryWithSystemAndUser,
    object::ObjectWithAttributesValues,
    question::QuestionWithAnswers,
    rule::RuleWithClausesAndEffects,
    system::{System, SystemData},
    user::UserWithoutPassword,
};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use serde_json::json;
use utoipa::ToSchema;

#[derive(ToSchema, Clone)]
pub struct ResponseBodyError {
    #[schema(value_type=u16)]
    pub status: StatusCode,
    pub error: String,
    pub extra: Option<String>,
}

impl Serialize for ResponseBodyError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ResponseBodyError", 3)?;
        state.serialize_field("status", &self.status.as_u16())?;
        state.serialize_field("error", &self.error)?;
        state.serialize_field("extra", &self.extra)?;
        state.end()
    }
}

#[derive(Serialize, ToSchema, Clone)]
#[aliases(
    ResponseBodyEmpty = ResponseBody<String>,
    ResponseBodyAnswers = ResponseBody<Vec<Answer>>,
    ResponseBodyAttributeValues = ResponseBody<Vec<AttributeValue>>,
    ResponseBodyAttributes = ResponseBody<Vec<AttributeWithAttributeValues>>,
    ResponseBodyClauses = ResponseBody<Vec<Clause>>,
    ResponseBodyHistory = ResponseBody<HistoryWithSystemAndUser>,
    ResponseBodyHistories = ResponseBody<Vec<HistoryWithSystemAndUser>>,
    ResponseBodyObjects = ResponseBody<Vec<ObjectWithAttributesValues>>,
    ResponseBodyQuestions = ResponseBody<Vec<QuestionWithAnswers>>,
    ResponseBodyRule = ResponseBody<RuleWithClausesAndEffects>,
    ResponseBodyRules = ResponseBody<Vec<RuleWithClausesAndEffects>>,
    ResponseBodyUser = ResponseBody<UserWithoutPassword>,
    ResponseBodyStartSystem = ResponseBody<SystemData>,
    ResponseBodySystem = ResponseBody<System>,
    ResponseBodySystems = ResponseBody<Vec<System>>
)]
pub struct ResponseBody<T: Clone> {
    pub succsess: bool,
    pub data: Option<T>,
    pub error: Option<ResponseBodyError>,
}

impl<T: Clone> ResponseBody<T> {
    pub fn unauthorized_example() -> ResponseBody<T> {
        ResponseBody::<T> {
            succsess: false,
            data: None,
            error: Some(ResponseBodyError {
                status: StatusCode::UNAUTHORIZED,
                error: "Not authorized".to_string(),
                extra: None,
            }),
        }
    }
}

impl<T: Clone> From<CustomErrors> for ResponseBody<T> {
    fn from(error: CustomErrors) -> ResponseBody<T> {
        ResponseBody {
            succsess: false,
            data: None,
            error: Some(error.into()),
        }
    }
}

impl<T: Serialize + Clone> From<T> for ResponseBody<T> {
    fn from(result: T) -> ResponseBody<T> {
        ResponseBody {
            succsess: true,
            data: Some(result),
            error: None,
        }
    }
}

impl<T: Serialize + Clone> IntoResponse for ResponseBody<T> {
    fn into_response(self) -> Response {
        let response = match (&self.succsess, &self.data, &self.error) {
            (true, _, _) => (StatusCode::OK, Json(json!(self))),
            (false, _, Some(error)) => (error.status, Json(json!(self))),
            (false, _, None) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!(self))),
        };
        response.into_response()
    }
}
