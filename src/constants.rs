use axum::http::Method;

pub struct UriInfo<'a> {
    pub uri: &'a str,
    pub method: Method,
}

pub const COOKIE_NAME: &str = "session_id";
pub const IMAGE_DIR: &str = "./images";
pub const URI_WITHOUT_AUTH: [UriInfo; 6] = [
    UriInfo {
        uri: r"\/api\/v1\/user\/login",
        method: Method::POST,
    },
    UriInfo {
        uri: r"\/api\/v1\/systems",
        method: Method::GET,
    },
    UriInfo {
        uri: r"\/api\/v1\/user\/logout",
        method: Method::POST,
    },
    UriInfo {
        uri: r"\/api\/v1\/user\/registration",
        method: Method::POST,
    },
    UriInfo {
        uri: r"\/swagger-ui\/api-docs\/openapi.json",
        method: Method::GET,
    },
    UriInfo {
        uri: r"/api/v1/systems/\d+/test",
        method: Method::GET,
    },
];
