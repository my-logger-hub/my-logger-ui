mod load_logs_action;
pub use load_logs_action::*;
mod search_logs_action;
pub use search_logs_action::*;

use my_http_server::*;
use rest_api_shared::LogApiItem;

/// A response of log events smaller than this is sent as it is: gzip would not pay off
const COMPRESSION_THRESHOLD: usize = 1024;

/// The log events as JSON - gzipped when the client takes gzip and the body is worth it
fn log_events_result(
    ctx: &HttpContext,
    items: Vec<LogApiItem>,
) -> Result<HttpOkResult, HttpFailResult> {
    let output = HttpOutput::as_json(items).add_header("Vary", "Accept-Encoding");

    if accepts_gzip(ctx) {
        return output
            .with_compression(COMPRESSION_THRESHOLD)
            .into_ok_result(true);
    }

    output.into_ok_result(true)
}

fn accepts_gzip(ctx: &HttpContext) -> bool {
    match ctx
        .request
        .get_headers()
        .try_get_case_insensitive_as_str("accept-encoding")
    {
        Ok(Some(accept_encoding)) => takes_gzip(accept_encoding),
        _ => false,
    }
}

/// `gzip` is one of the tokens of `Accept-Encoding`, and not with `q=0`
fn takes_gzip(accept_encoding: &str) -> bool {
    accept_encoding.split(',').any(|token| {
        let mut parts = token.split(';');
        let name = parts.next().unwrap_or_default().trim();

        if !name.eq_ignore_ascii_case("gzip") {
            return false;
        }

        !parts.any(|param| {
            let param = param.trim();
            param
                .strip_prefix("q=")
                .map_or(false, |q| q.trim().parse::<f32>() == Ok(0.0))
        })
    })
}

#[cfg(test)]
mod tests {
    use super::takes_gzip;

    #[test]
    fn gzip_is_a_token_of_accept_encoding() {
        assert!(takes_gzip("gzip"));
        assert!(takes_gzip("gzip, deflate, br, zstd"));
        assert!(takes_gzip("br;q=1.0, GZIP;q=0.8"));

        assert!(!takes_gzip(""));
        assert!(!takes_gzip("br, zstd"));
        assert!(!takes_gzip("x-gzip-like"));
        assert!(!takes_gzip("gzip;q=0, br"));
    }
}
