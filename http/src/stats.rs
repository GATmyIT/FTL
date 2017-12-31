use wrapper::{log, get_blocking_status, get_counters, get_clients, BlockingStatus};
use util::{Reply, reply_data};

#[get("/stats/summary")]
fn summary() -> Reply {
    let counters = get_counters();

    let blocked = counters.blocked + counters.wildcardblocked;
    let total = counters.queries - counters.invalidqueries;
    let percentage = if total > 0 { 100f32 * blocked as f32 / total as f32 } else { 0f32 };

    let status = match *get_blocking_status() {
        BlockingStatus::Disabled => { "disabled" },
        BlockingStatus::Enabled => { "enabled" },
        BlockingStatus::Unknown => { "unknown" }
    };

    let active_clients = get_clients().len();

    reply_data(json!({
        "domains_blocked": counters.gravity,
        "dns_queries": total,
        "ads_blocked": blocked,
        "ad_percentage": percentage,
        "unique_domains": counters.domains,
        "queries_forwarded": counters.forwardedqueries,
        "queries_cached": counters.cached,
        "clients_seen": active_clients,
        "status": status
    }))
}
