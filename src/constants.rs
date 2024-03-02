use axum::http::Method;

pub struct UriInfo<'a> {
    pub uri: &'a str,
    pub method: Method,
}

pub const COOKIE_NAME: &str = "session_id";
pub const IMAGE_DIR: &str = "./images";
pub const URI_WITHOUT_AUTH: [UriInfo; 5] = [
    UriInfo {
        uri: "/user/login",
        method: Method::POST,
    },
    UriInfo {
        uri: "/system",
        method: Method::GET,
    },
    UriInfo {
        uri: "/system",
        method: Method::POST,
    },
    UriInfo {
        uri: "/user/logout",
        method: Method::POST,
    },
    UriInfo {
        uri: "/user/registration",
        method: Method::POST,
    },
];
