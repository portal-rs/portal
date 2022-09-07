use crate::{
    binary, constants,
    packing::error::UnpackError,
    types::{
        dns::{Header, Message, Question, RawHeader},
        rr::{Class, RRHeader, Record, ResourceRecord, Type},
    },
};

pub type UnpackResult<T> = Result<(T, usize), UnpackError>;

/// Savely unpacks a u16 from a vector of bytes.
pub fn unpack_u16(data: &Vec<u8>, offset: usize) -> UnpackResult<u16> {
    if offset + 2 > data.len() {
        return Err(UnpackError::new("Offset overflow unpacking u16"));
    }

    return match binary::read_u16(&data[offset..], binary::Endianness::Big) {
        Ok(int) => Ok((int, offset + 2)),
        Err(_) => Err(UnpackError::new("Slice too short to unpack u16")),
    };
}

/// Savely unpacks a u32 from a vector of bytes.
pub fn unpack_u32(data: &Vec<u8>, offset: usize) -> UnpackResult<u32> {
    if offset + 4 > data.len() {
        return Err(UnpackError::new("Offset overflow unpacking u32"));
    }

    return match binary::read_u32(&data[offset..], binary::Endianness::Big) {
        Ok(int) => Ok((int, offset + 4)),
        Err(_) => Err(UnpackError::new("Slice too short to unpack u32")),
    };
}

/// Savely unpacks a u64 from a vector of bytes.
pub fn unpack_u64(data: &Vec<u8>, offset: usize) -> UnpackResult<u64> {
    if offset + 8 > data.len() {
        return Err(UnpackError::new("Offset overflow unpacking u64"));
    }

    return match binary::read_u64(&data[offset..], binary::Endianness::Big) {
        Ok(int) => Ok((int, offset + 8)),
        Err(_) => Err(UnpackError::new("Slice too short to unpack u64")),
    };
}

/// Unpacks the first 12 octects from the DNS message. The DNS header is fixed in size. The function returns the
/// [`Header`] it self and the offset (which will always be 12). This function is usually the first step in unpacking
/// the whole message.
pub fn unpack_header(data: &Vec<u8>) -> UnpackResult<Header> {
    let (id, offset) = match unpack_u16(&data, 0) {
        Ok(id) => id,
        Err(_) => return Err(UnpackError::new("Failed to unpack DNS header ID")),
    };

    let (flags, offset) = match unpack_u16(&data, offset) {
        Ok(flags) => flags,
        Err(_) => return Err(UnpackError::new("Failed to unpack DNS header flags")),
    };

    let (qdcount, offset) = match unpack_u16(&data, offset) {
        Ok(qdcount) => qdcount,
        Err(_) => return Err(UnpackError::new("Failed to unpack DNS header QDCOUNT")),
    };

    let (ancount, offset) = match unpack_u16(&data, offset) {
        Ok(ancount) => ancount,
        Err(_) => return Err(UnpackError::new("Failed to unpack DNS header ANCOUNT")),
    };

    let (nscount, offset) = match unpack_u16(&data, offset) {
        Ok(nscount) => nscount,
        Err(_) => return Err(UnpackError::new("Failed to unpack DNS header NSCOUNT")),
    };

    let (arcount, offset) = match unpack_u16(&data, offset) {
        Ok(arcount) => arcount,
        Err(_) => return Err(UnpackError::new("Failed to unpack DNS header ARCOUNT")),
    };

    let header = Header::from(RawHeader {
        id,
        flags,
        qdcount,
        ancount,
        nscount,
        arcount,
    });

    Ok((header, offset))
}

/// Unpack the complete DNS [`Message`] based on the already unpacked [`Header`].
pub fn unpack_message(
    header: Header,
    data: Vec<u8>,
    offset: usize,
) -> Result<Message, UnpackError> {
    let mut message = Message::new_with_header(header);

    // Immediatly return if the message only consists of header data without
    // any body data
    if offset == data.len() {
        return Err(UnpackError::new("No body data"));
    }

    // We cannot trust the values of QDCOUNT, ANCOUNT, NSCOUNT and ARCOUNT,
    // as these values can be manipulated by potential attackers. The first
    // step is to assume the values are correct and if we detect a wrong
    // offset we can be pretty sure the count is wrong.
    //
    // Loop over the questions. Usually there is only one question, but the
    // spec accounts for the possibility to ask multiple questions at once.
    let (mut questions, offset, qdcount) =
        match unpack_questions(message.header.qdcount, &data, offset) {
            Ok(result) => result,
            Err(_) => todo!(),
        };
    message.header.qdcount = qdcount.unwrap_or(message.header.qdcount);
    message.question.append(&mut questions);

    // Unpack list of answer RRS in the answer section
    let (mut answers, offset, ancount) = match unpack_rrs(message.header.ancount, &data, offset) {
        Ok(result) => result,
        Err(_) => todo!(),
    };
    message.header.ancount = ancount.unwrap_or(message.header.ancount);
    message.answer.append(&mut answers);

    // Unpack list of nameserver RRs in the authority section
    let (mut nameservers, offset, nscount) = match unpack_rrs(message.header.nscount, &data, offset)
    {
        Ok(result) => result,
        Err(_) => todo!(),
    };
    message.header.nscount = nscount.unwrap_or(message.header.nscount);
    message.authority.append(&mut nameservers);

    // Unpack list of additional RRs in the additional section
    let (mut additional, _, arcount) = match unpack_rrs(message.header.arcount, &data, offset) {
        Ok(result) => result,
        Err(_) => todo!(),
    };
    message.header.arcount = arcount.unwrap_or(message.header.arcount);
    message.additional.append(&mut additional);

    Ok(message)
}

/// Unpacks a list of [`Question`]s and returns the optional correct QDCOUNT.
fn unpack_questions(
    count: u16,
    data: &Vec<u8>,
    mut offset: usize,
) -> Result<(Vec<Question>, usize, Option<u16>), UnpackError> {
    let mut questions: Vec<Question> = Vec::new();

    for i in 0..count {
        let initial_offset = offset;

        let (question, new_offset) = match unpack_question(&data, offset) {
            Ok(result) => result,
            Err(_) => todo!(),
        };

        offset = new_offset;

        // If the initial offset and the offset after unwrapping the question
        // match we know that QDCOUNT is wrong.
        if new_offset == initial_offset {
            return Ok((questions, offset, Some(i)));
        }

        questions.push(question);
    }

    Ok((questions, offset, None))
}

/// Unpacks a single [`Question`]. Returns the unpacked question and new offset.
fn unpack_question(data: &Vec<u8>, offset: usize) -> Result<(Question, usize), UnpackError> {
    let (name, offset) = match unpack_domain_name(data, offset) {
        Ok(name) => name,
        Err(_) => return Err(UnpackError::new("Failed to unpack question domain name")),
    };

    let (typ, offset) = match unpack_u16(data, offset) {
        Ok(typ) => typ,
        Err(_) => return Err(UnpackError::new("Failed to unpack question RR type")),
    };

    let (class, offset) = match unpack_u16(data, offset) {
        Ok(class) => class,
        Err(_) => return Err(UnpackError::new("Failed to unpack question RR class")),
    };

    let question = Question {
        name,
        typ: Type::from(typ),
        class: Class::from(class),
    };

    Ok((question, offset))
}

/// Unpacks a list of [`ResourceRecord`]s and returns the optional correct COUNT.
fn unpack_rrs(
    count: u16,
    data: &Vec<u8>,
    mut offset: usize,
) -> Result<(Vec<ResourceRecord>, usize, Option<u16>), UnpackError> {
    let mut rrs: Vec<ResourceRecord> = Vec::new();

    // Nothing to do, return
    if count == 0 {
        return Ok((rrs, offset, None));
    }

    for i in 0..count {
        let initial_offset = offset;

        let (rr, new_offset) = match unpack_rr(data, offset) {
            Ok(result) => result,
            Err(err) => return Err(UnpackError::new(format!("Failed to unpack RRs: {}", err))),
        };

        offset = new_offset;

        // If the initial offset and the offset after unwrapping the RR
        // match we know that count is wrong.
        if new_offset == initial_offset {
            return Ok((rrs, offset, Some(i)));
        }

        rrs.push(rr)
    }

    Ok((rrs, offset, None))
}

/// Unpacks a single [`ResourceRecord`].
fn unpack_rr(data: &Vec<u8>, offset: usize) -> Result<(ResourceRecord, usize), UnpackError> {
    let (header, offset) = match unpack_rr_header(data, offset) {
        Ok(result) => result,
        Err(_) => todo!(),
    };

    let rr = ResourceRecord::from(header);

    let offset = match rr.unpack(data, offset) {
        Ok(offset) => offset,
        Err(_) => todo!(),
    };

    Ok((rr, offset))
}

fn unpack_rr_header(data: &Vec<u8>, offset: usize) -> Result<(RRHeader, usize), UnpackError> {
    let (name, offset) = match unpack_domain_name(data, offset) {
        Ok(result) => result,
        Err(_) => todo!(),
    };

    let (typ, offset) = match unpack_u16(data, offset) {
        Ok(result) => result,
        Err(_) => todo!(),
    };

    let (class, offset) = match unpack_u16(data, offset) {
        Ok(result) => result,
        Err(_) => todo!(),
    };

    let (ttl, offset) = match unpack_u32(data, offset) {
        Ok(result) => result,
        Err(_) => todo!(),
    };

    let (rdlen, offset) = match unpack_u16(data, offset) {
        Ok(result) => result,
        Err(_) => todo!(),
    };

    let header = RRHeader {
        name,
        typ: Type::from(typ),
        class: Class::from(class),
        ttl,
        rdlen,
    };

    Ok((header, offset))
}

/// Unpacks a domain name (e.g. 'example.com').
fn unpack_domain_name(data: &Vec<u8>, offset: usize) -> Result<(String, usize), UnpackError> {
    let data_length = data.len();
    if offset > data_length {
        return Err(UnpackError::new("Offset overflow unpacking domain name"));
    }

    // If we immediatly encounter a null byte, the name is root (.)
    if data[offset] == 0x00 {
        return Ok((".".to_string(), offset + 1));
    }

    // Setup initial data
    let mut buf: Vec<u8> = Vec::new();
    let mut initial_offset = 0;
    let mut followed = false;
    let mut offset = offset;

    loop {
        let b = data[offset] as usize;
        offset += 1;

        // Check if we have a pointer (11000000 => 0xC0). Pointers point to
        // domain names previously defined in some part of the message. We
        // follow the pointer (by updating the offset) and reading in the
        // domain name as usual. After encountering the terminating null byte
        // we jump back by updating the offset.
        match b & 0xC0 {
            0x00 => {
                // We encountered the terminating null byte, break
                if b == 0x00 {
                    break;
                }

                // The label length is bigger than the complete DNS message
                if b > data_length {
                    return Err(UnpackError::new("Offset overflow unpacking domain name"));
                }

                // The maximum label length can only be 0x3F (63) as the first
                // two bits are reserved (e.g. for compression pointers).
                if b > constants::dns::MAX_LABEL_LENGTH {
                    return Err(UnpackError::new(
                        "Invalid label length while unpacking domain name",
                    ));
                }

                // If the current buffer size + the new label length exceed
                // the maximum domain length.
                if buf.len() + b > constants::dns::MAX_DOMAIN_LENGTH {
                    return Err(UnpackError::new(
                        "Max domain length exceeded while unpacking domain name",
                    ));
                }

                // Extract the number of octets indicated by the length octet.
                // We then append it to the buffer and also add an additional
                // full stop ('.' or 0x2E).
                let mut label = data[offset..offset + b].to_vec();
                buf.append(&mut label);
                buf.push(0x2E);
                offset += b;
            }
            0xC0 => {
                if offset + b > data_length {
                    return Err(UnpackError::new("Offset overflow unpacking domain name"));
                }

                // Save the initial offset to later return the correct offset
                // to further unpack data after the domain name.
                if !followed {
                    initial_offset = offset + 1;
                }

                // Follow the pointer by updating the offset. We AND with 0x3F
                // (00111111) to get the pointer target value.
                offset = b & constants::dns::COMPRESSION_POINTER_MASK;
                followed = true;
            }
            _ => {
                // This state is impossible to reach, but we need this branch
                // to satisfy the Rust compiler.
                return Err(UnpackError::new(
                    "Impossible to reach: Unpacking domain name",
                ));
            }
        }
    }

    // If we followed any compression pointers, we need to set the offset to
    // the initial value to ensure we continue at the correct offset location.
    if followed {
        offset = initial_offset;
    }

    // From UTF-8 works here, as ASCII characters are valid UTF-8
    let domain_name = match String::from_utf8(buf) {
        Ok(name) => name,
        Err(_) => {
            return Err(UnpackError::new(
                "Failed to convert raw byte buffer to string while unpacking domain name",
            ))
        }
    };

    Ok((domain_name, offset))
}
