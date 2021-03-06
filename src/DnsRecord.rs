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
        pub fn read(buffer: &mut BytePacketBuffer) -> Result<DnsRecord>{
            let mut domain = String::new();
            buffer.read_qname(&mut domain)?;

            let qtype_num = buffer.read_u16()?;
            let qtype = QueryType::from_num(qtype_num);
            let _ = buffer.read_u16()?;
            let ttl = buffer.read_u32()?;
            let data_len = buffer.read_u16()?;

            match qtype {
                //Query Type is A lookup
                QueryType::A => {
                    let raw_addr = buffer.read_u32()?;
                    let addr = Ipv4Addr::new(
                        ((raw_addr >> 24) & 0xFF) as u8,
                        ((raw_addr >> 16) & 0XFF) as u8,
                        ((raw_addr >> 8) & 0xFF) as u8,
                        ((raw_addr >> 0) & 0xFF) as u8,
                    );
                    Ok(DnsRecord::A{
                        domain: domain,
                        addr: addr,
                        ttl: ttl,
                    })
                }
                QueryType::UNKNOWN(_) => {
                    buffer.step(data_len as usize)?;
                    Ok(DnsRecord::UNKNOWN{
                        domain: domain,
                        qtype: qtype_num,
                        data_len: data_len,
                        ttl: ttl,
                    })
                }
            }
        }
    }
}
