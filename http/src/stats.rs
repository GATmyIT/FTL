use wrapper::*;
use util::{Reply, reply_data};

#[get("/stats/summary")]
pub fn summary() -> Reply {
    let count = get_counters();

    let blocked = count.blocked + count.wildcardblocked;
    let total = count.queries - count.invalidqueries;
    let percentage = if total > 0 { 100f32 * blocked as f32 / total as f32 } else { 0f32 };

    let status = match *get_blocking_status() {
        BlockingStatus::Disabled => { "disabled" },
        BlockingStatus::Enabled => { "enabled" },
        BlockingStatus::Unknown => { "unknown" }
    };

    let active_clients = get_clients().len();

    reply_data(json!({
        "domains_blocked": count.gravity,
        "dns_queries": total,
        "ads_blocked": blocked,
        "ad_percentage": percentage,
        "unique_domains": count.domains,
        "queries_forwarded": count.forwardedqueries,
        "queries_cached": count.cached,
        "clients_seen": active_clients,
        "status": status
    }))
}

#[get("/stats/history")]
pub fn history() -> Reply {
    let history: Vec<Query> = get_queries().iter()
        .filter(|query| query.valid)
        .map(|query| {
            Query(
                query.timestamp,
                match query.query_type {
                    QueryType::IPv4 => "IPv4",
                    QueryType::IPv6 => "IPv6",
                    QueryType::Unknown => "Unknown"
                },
                get_domain_from_id(query.domain_id as usize).unwrap(),
                get_client_from_id(query.client_id as usize).unwrap(),
                query.status as u8
            )
        })
        .collect();

    reply_data(json!({
        "history": history
    }))
}

#[derive(Serialize)]
struct Query<'a>(i32, &'a str, &'a str, &'a str, u8);
