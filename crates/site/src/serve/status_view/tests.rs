use std::path::{Path, PathBuf};

use http_body_util::BodyExt as _;
use n333_core::epoch::unix_now_millis;

use super::super::machine::Release;
use super::super::pages;
use super::super::respond::Reply;
use super::super::state::State;
use crate::history::{self, Sample};

/// A server over the repository's pages, with an observation of now and `samples` as
/// its history.
fn state(name: &str, samples: &[Sample]) -> (State, PathBuf) {
    let dir = std::env::temp_dir().join(format!("n333-site-status-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let now = unix_now_millis();
    let observation = serde_json::json!({
        "format": 1, "as_of": now, "epoch": epoch_now(), "running": true,
        "status": { "answering": 4, "roll": 2 },
        "nodes": [
            { "id": "f", "founder": true, "admitted": null },
            { "id": "a", "admitted": 30 },
            { "id": "b", "admitted": 31 },
            { "id": "s", "admitted": null },
        ],
    });
    std::fs::write(dir.join("network.json"), observation.to_string()).unwrap();
    let lines: String = samples
        .iter()
        .map(|sample| serde_json::to_string(sample).unwrap() + "\n")
        .collect();
    std::fs::write(dir.join(history::FILE), lines).unwrap();
    let site = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../site");
    let state = State::open(
        &site,
        &dir.join("state"),
        dir.join("network.json"),
        "abc1234".to_owned(),
    )
    .unwrap()
    .with_release(Release {
        commit: Some("a".repeat(40)),
        deployed_at: Some(1_759_000_000_000),
    });
    (state, dir)
}

fn epoch_now() -> u64 {
    n333_core::Epoch::at_unix_seconds(unix_now_millis() / 1000).0
}

/// The epochs before now, oldest first, leaving out `gaps`; each with its own numbers.
fn samples(count: u64, gaps: &[u64]) -> Vec<Sample> {
    let now = epoch_now();
    (now - count..now)
        .filter(|epoch| !gaps.contains(epoch))
        .map(|epoch| Sample {
            epoch,
            as_of: epoch * 19_980_000,
            roll: 2 + epoch % 3,
            answering: Some(epoch % 2),
            saying: None,
            tor: None,
            site_node_running: true,
        })
        .collect()
}

async fn body(reply: Reply) -> String {
    let bytes = reply.into_body().collect().await.unwrap().to_bytes();
    String::from_utf8(bytes.to_vec()).unwrap()
}

async fn page(state: &State) -> String {
    let reply = pages::any(state, "/status");
    assert_eq!(reply.status(), hyper::StatusCode::OK);
    body(reply).await
}

const TOO_FEW: &str = "Fewer than two epochs are written down for this stretch";

#[tokio::test]
async fn with_no_history_the_page_says_so_and_still_gives_the_numbers_now() {
    let (state, dir) = state("none", &[]);
    let html = page(&state).await;
    assert_eq!(html.matches(TOO_FEW).count(), 2, "{html}");
    assert!(!html.contains("<svg viewBox=\"0 0 560"));
    assert!(html.contains("<dt>On the roll, founder included</dt><dd>3</dd>"));
    assert!(html.contains("<dt>Answering</dt><dd>4</dd>"));
    assert!(html.contains(&format!("<dd>{} <span class=\"line-epoch\">", epoch_now())));
    assert!(html.contains(&format!(
        r#"<a href="https://github.com/needmoretruth/333/commit/{}"><code>abc1234</code></a>"#,
        "a".repeat(40)
    )));
    assert!(html.contains(r#"<time datetime="2025-09-27T19:06:40Z">2025-09-27T19:06:40Z</time>"#));
    assert!(html.contains(" ago. It was running."));
    assert!(html.contains(r#"<link rel="canonical" href="https://the333.dev/status">"#));
    assert!(!html.contains("noindex"));
    assert!(!html.contains("{{"), "every token is filled");
    std::fs::remove_dir_all(&dir).unwrap();
}

#[tokio::test]
async fn one_sample_is_not_a_line() {
    let (state, dir) = state("one", &samples(1, &[]));
    let html = page(&state).await;
    assert_eq!(html.matches(TOO_FEW).count(), 2);
    std::fs::remove_dir_all(&dir).unwrap();
}

#[tokio::test]
async fn many_samples_are_drawn_and_written_out() {
    let now = epoch_now();
    let (state, dir) = state("many", &samples(400, &[now - 10, now - 8]));
    let html = page(&state).await;
    assert!(!html.contains(TOO_FEW));
    assert_eq!(html.matches("<path class=\"series roll\"").count(), 2);
    assert_eq!(html.matches("<path class=\"series answering\"").count(), 2);
    // Epoch now-9 stands alone between two gaps: a dot, not a line.
    assert_eq!(html.matches("<circle class=\"point roll\"").count(), 2);
    let last = now - 1;
    let latest = (2 + last % 3, last % 2);
    let said = |first: u64| {
        format!(
            "Epochs {first} to {last}, from {} to {}. On the roll: lowest 2, highest 4, latest {}. \
             Answering: lowest 0, highest 1, latest {}.",
            super::chart::day(first),
            super::chart::day(last),
            latest.0,
            latest.1
        )
    };
    assert!(
        html.contains(&said(now - 332)),
        "the last 333 epochs: {html}"
    );
    assert!(html.contains(&said(now - 400)), "everything");
    assert!(html.contains(&format!(">{}</text>", now - 400)));
    std::fs::remove_dir_all(&dir).unwrap();
}

#[tokio::test]
async fn the_json_says_what_the_page_says_and_the_sitemap_lists_the_page() {
    let (state, dir) = state("json", &samples(3, &[]));
    let json: serde_json::Value =
        serde_json::from_str(&body(pages::status_api(&state)).await).unwrap();
    assert_eq!(json["now"]["roll"], 3);
    assert_eq!(json["now"]["answering"], 4);
    assert_eq!(json["now"]["site_node_running"], true);
    assert_eq!(json["machine"]["version"], "abc1234");
    assert_eq!(json["history"].as_array().unwrap().len(), 3);
    let sitemap = body(pages::sitemap(&state)).await;
    assert!(sitemap.contains("<loc>https://the333.dev/status</loc>"));
    std::fs::remove_dir_all(&dir).unwrap();
}
