#[utoipa::path(
    tag = "test",
    get,
    path = "/auth/test_shit",
    responses(
        (status = 200, description = "this shit wroks", body = str)
    ),
    params(
        ("id" = u64, Path, description = "id to test shit"),
    )
)]
pub fn test_shit() -> &'static str {
    "hello"
}
