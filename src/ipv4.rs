use crate::protocol::Protocol;

pub struct IPv4Header<'a> {
    // raw_data is raw protocol frame
    raw_data: &'a [u8],
    // version contains version of IP frame (currently only 4th)
    version: u8,
    // Internet header length
    ihl: u8,
}

///  3.2 Frame format:
//   If flag IFF_NO_PI is not set each frame format is:
//      Flags [2 bytes]
//      Proto [2 bytes]
//      Raw Protocol(IP, IPv6, etc) frame.
//  https://en.wikipedia.org/wiki/EtherType
impl<'a> IPv4Header<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        IPv4Header {
            raw_data: data,
            version: 0,
            ihl: 0,
        }
    }

    // 0-th octet
    // lower 4 bits
    pub fn version(&mut self) -> u8 {
        self.version = self.raw_data[0] >> 4;
        self.version
    }

    // 0-th octet
    // upper 4 bits
    // The minimum value for this field is 5,[29] which indicates a length of 5 × 32 bits = 160 bits = 20 bytes.
    // As a 4-bit field, the maximum value is 15,
    // this means that the maximum size of the IPv4 header is 15 × 32 bits, or 480 bits = 60 bytes.
    pub fn ihl(&mut self) -> u8 {
        self.ihl = self.raw_data[0] & 0b0000_1111;
        self.ihl
    }

    // 9-th octet (byte)
    // Protocol number
    // https://en.wikipedia.org/wiki/List_of_IP_protocol_numbers
    pub fn protocol(&self) -> Protocol {
        match self.raw_data[9] {
            1 => {
                Protocol::ICMP
            }
            2 => {
                Protocol::IGMP
            }
            17 => {
                Protocol::UDP
            }
            number => {
                println!("number is: {}", number);
                Protocol::Unknown
            }
        }
    }
}