use libc;
use std::ffi::{CString, CStr};
use std::str::Utf8Error;
use std::slice::from_raw_parts;

//noinspection RsStaticConstNaming
extern {
    static debug: bool;
    static counters: Counters;
    static blockingstatus: BlockingStatus;
    static queries: *mut QueryData;
    static clients: *mut ClientData;
    static domains: *mut DomainData;

    fn logg(format: *const libc::c_char, ...);
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Counters {
    pub queries: i32,
    pub invalidqueries: i32,
    pub blocked: i32,
    pub wildcardblocked: i32,
    pub cached: i32,
    pub unknown: i32,
    pub forwarded: i32,
    pub clients: i32,
    pub domains: i32,
    pub queries_MAX: i32,
    pub forwarded_MAX: i32,
    pub clients_MAX: i32,
    pub domains_MAX: i32,
    pub overTime_MAX: i32,
    pub gravity: i32,
    pub overTime: i32,
    pub IPv4: i32,
    pub IPv6: i32,
    pub PTR: i32,
    pub SRV: i32,
    pub wildcarddomains: i32,
    pub forwardedqueries: i32
}

#[repr(C)]
pub struct ClientData {
    pub magic: u8,
    pub count: i32,
    pub ip: *mut libc::c_char,
    pub name: *mut libc::c_char
}

#[repr(C)]
#[derive(Debug)]
pub struct QueryData {
    pub magic: u8,
    pub timestamp: i32,
    pub timeidx: i32,
    pub query_type: QueryType,
    pub status: QueryStatus,
    pub domain_id: i32,
    pub client_id: i32,
    pub forward_id: i32,
    pub valid: bool,
    pub db: bool
}

#[repr(C)]
pub struct DomainData {
    pub magic: u8,
    pub count: i32,
    pub blockedcount: i32,
    pub domain: *mut libc::c_char,
    pub wildcard: bool
}

#[repr(u8)]
pub enum BlockingStatus {
    Disabled,
    Enabled,
    Unknown
}

#[repr(u8)]
#[derive(Debug)]
pub enum QueryType {
    Unknown, IPv4, IPv6
}

#[repr(u8)]
#[derive(Serialize, Copy, Clone, Debug)]
pub enum QueryStatus {
    Unknown, ExactBlock, Reply, Cache, WildBlock
}

pub fn is_debug() -> bool {
    unsafe { debug }
}

pub fn get_counters() -> &'static Counters {
    unsafe { &counters }
}

pub fn get_queries() -> &'static [QueryData] {
    unsafe {
        from_raw_parts(queries, counters.queries as usize)
    }
}

pub fn get_clients() -> &'static [ClientData] {
    unsafe {
        from_raw_parts(clients, counters.clients as usize)
    }
}

pub fn get_domains() -> &'static [DomainData] {
    unsafe {
        from_raw_parts(domains, counters.domains as usize)
    }
}

pub fn get_domain_from_id(id: usize) -> Result<&'static str, Utf8Error> {
    unsafe {
        CStr::from_ptr(get_domains()[id].domain).to_str()
    }
}

pub fn get_client_from_id(id: usize) -> Result<&'static str, Utf8Error> {
    unsafe {
        CStr::from_ptr(get_clients()[id].name).to_str()
    }
}

pub fn get_blocking_status() -> &'static BlockingStatus {
    unsafe { &blockingstatus }
}

pub fn log(msg: &str) {
    unsafe {
        let c_str = CString::new(msg).unwrap();
        logg(c_str.as_ptr());
    }
}
