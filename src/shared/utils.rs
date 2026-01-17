use uuid::Uuid;
use crate::shared::response::PaginationRequest;

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

pub fn extract_pagination_data(pagination: Option<PaginationRequest>) -> (Option<u32>, Option<u32>, Option<String>) {
    let mut limit: Option<u32> = None;
    let mut offset: Option<u32> = None;
    let mut search: Option<String> = None;

    if let Some(pagination) = pagination {
        limit = pagination.page_size;

        if let (Some(page_size), Some(page)) = (pagination.page_size, pagination.page) {
            offset = Some(page * page_size);
        }

        if let Some(location_name) = pagination.search {
            search = Some(location_name);
        }
    }
    
    (limit, offset, search)
}
