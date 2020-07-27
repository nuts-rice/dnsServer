//allows an enum for easy expansion
#[derive (Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[allow(dead_code)]
pub enum DnsRecord {
    UNKNOWN {
        domain: String,
        qtype: u16,
        data_len: u16,
        ttl: u32,
    },  //0
    A{
        domain: String,
        addr: Ipv4Addr,
        ttl: u32
    },  //1

    //TODO: implementation of DnsRecord 
    impl DnsRecord {

    }
}
