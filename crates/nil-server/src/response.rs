#[macro_export]
macro_rules! err {
  ($status:ident, $error:expr) => {{
    use axum::body::Body;
    use axum::http::StatusCode;
    use axum::response::Response;

    #[cfg(feature = "tracing")]
    {
      let error = &$error;
      tracing::error!(%error);
    }

    Response::builder()
      .status(StatusCode::$status)
      .body(Body::from($error.to_string()))
      .unwrap()
  }};
}
