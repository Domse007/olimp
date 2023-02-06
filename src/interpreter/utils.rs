pub fn read_to_u32<T: Iterator<Item = u8>>(data: &mut T) -> u32 {
    let parts = [
        data.next().unwrap(),
        data.next().unwrap(),
        data.next().unwrap(),
        data.next().unwrap(),
    ];
    u32::from_le_bytes(parts)
}

pub fn read_to_u16<T: Iterator<Item = u8>>(data: &mut T) -> u16 {
    let parts = [data.next().unwrap(), data.next().unwrap()];
    u16::from_le_bytes(parts)
}

pub fn read_to_f64<T: Iterator<Item = u8>>(data: &mut T) -> f64 {
    let parts = [
        data.next().unwrap(),
        data.next().unwrap(),
        data.next().unwrap(),
        data.next().unwrap(),
        data.next().unwrap(),
        data.next().unwrap(),
        data.next().unwrap(),
        data.next().unwrap(),
    ];
    f64::from_le_bytes(parts)
}

#[test]
fn test_u32_from_bytes() {
    let num: u32 = 1023;
    let bytes = num.to_le_bytes();
    assert_eq!(read_to_u32(&mut bytes.into_iter()), num);
}

#[test]
fn test_u16_from_bytes() {
    let num: u16 = 3234;
    let bytes = num.to_le_bytes();
    assert_eq!(read_to_u16(&mut bytes.into_iter()), num);
}
