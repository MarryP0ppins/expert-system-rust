use axum::http::Method;

pub struct UriInfo<'a> {
    pub uri: &'a str,
    pub method: Method,
}

pub const COOKIE_NAME: &str = "session_id";
pub const IMAGE_DIR: &str = "./images";
pub const URI_WITHOUT_AUTH: [UriInfo; 5] = [
    UriInfo {
        uri: "/api/v1/users/login",
        method: Method::POST,
    },
    UriInfo {
        uri: "/api/v1/systems",
        method: Method::GET,
    },
    UriInfo {
        uri: "/api/v1/users/logout",
        method: Method::POST,
    },
    UriInfo {
        uri: "/api/v1/users/registration",
        method: Method::POST,
    },
    UriInfo {
        uri: "/swagger-ui/api-docs/openapi.json",
        method: Method::GET,
    },
];
