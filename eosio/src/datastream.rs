use eosio_types::Name;

#[derive(Debug)]
pub enum ReadError {
    NotEnoughBytes,
}

pub fn read_u16(bytes: &[u8]) -> Result<u16, ReadError> {
    if bytes.len() < 2 {
        return Err(ReadError::NotEnoughBytes);
    }
    let num = ((bytes[0] as u16) << 0) | ((bytes[1] as u16) << 8);
    Ok(num)
}

pub fn read_u32(bytes: &[u8]) -> Result<u32, ReadError> {
    if bytes.len() < 4 {
        return Err(ReadError::NotEnoughBytes);
    }
    let num = ((bytes[0] as u32) << 0)
        | ((bytes[1] as u32) << 8)
        | ((bytes[2] as u32) << 16)
        | ((bytes[3] as u32) << 24);
    Ok(num)
}

pub fn read_u64(bytes: &[u8]) -> Result<u64, ReadError> {
    if bytes.len() < 8 {
        return Err(ReadError::NotEnoughBytes);
    }
    let num = ((bytes[0] as u64) << 0)
        | ((bytes[1] as u64) << 8)
        | ((bytes[2] as u64) << 16)
        | ((bytes[3] as u64) << 24)
        | ((bytes[4] as u64) << 32)
        | ((bytes[5] as u64) << 40)
        | ((bytes[6] as u64) << 48)
        | ((bytes[7] as u64) << 56);
    Ok(num)
}

pub fn read_name(bytes: &[u8]) -> Result<Name, ReadError> {
    let num = read_u64(bytes)?;
    Ok(Name::new(num))
}

pub fn read_u128(bytes: &[u8]) -> Result<u128, ReadError> {
    if bytes.len() < 16 {
        return Err(ReadError::NotEnoughBytes);
    }
    let num = ((bytes[0] as u128) << 0)
        | ((bytes[1] as u128) << 8)
        | ((bytes[2] as u128) << 16)
        | ((bytes[3] as u128) << 24)
        | ((bytes[4] as u128) << 32)
        | ((bytes[5] as u128) << 40)
        | ((bytes[6] as u128) << 48)
        | ((bytes[7] as u128) << 56)
        | ((bytes[8] as u128) << 64)
        | ((bytes[9] as u128) << 72)
        | ((bytes[10] as u128) << 80)
        | ((bytes[11] as u128) << 88)
        | ((bytes[12] as u128) << 96)
        | ((bytes[13] as u128) << 104)
        | ((bytes[14] as u128) << 112)
        | ((bytes[15] as u128) << 120);
    Ok(num)
}
