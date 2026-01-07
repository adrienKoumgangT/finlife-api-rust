use uuid::Uuid;

pub fn ub(u: Uuid) -> Vec<u8> {
    u.as_bytes().to_vec()
}

pub fn oub(u: Option<Uuid>) -> Option<Vec<u8>> {
    match u {
        Some(u) => Some(u.as_bytes().to_vec()),
        None => None
    }
}

pub fn bu(b: &[u8]) -> Uuid {
    Uuid::from_slice(b).expect("invalid uuid bytes")
}

pub fn obu(b: Option<&[u8]>) -> Option<Uuid> {
    match b {
        Some(b) => Some(Uuid::from_slice(b).expect("invalid uuid bytes")),
        None => None
    }
}
