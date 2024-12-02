use std::{fs::File, io::Read as _};

use byte_packet_buffer::BytePacketBuffer;
use dns_packet::DnsPacket;
use types::Result;

pub mod types;
pub mod query_type;
pub mod result_code;
pub mod dns_header;
pub mod dns_question;
pub mod dns_record;
pub mod byte_packet_buffer;
pub mod dns_packet;

fn main() -> Result<()> {
    let mut f = File::open("response_packet.txt")?;
    let mut buffer = BytePacketBuffer::new();
    f.read(&mut buffer.buf)?;

    let packet = DnsPacket::from_buffer(&mut buffer)?;
    println!("{:#?}", packet.header);

    for q in packet.questions {
        println!("{:#?}", q);
    }
    for rec in packet.answers {
        println!("{:#?}", rec);
    }
    for rec in packet.authorities {
        println!("{:#?}", rec);
    }
    for rec in packet.resources {
        println!("{:#?}", rec);
    }

    Ok(())
}
