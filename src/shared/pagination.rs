use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Page {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

impl Page {
    pub fn limit_offset(&self) -> (u32, u32) {
        let page = self.page.unwrap_or(1).max(1);
        let per = self.per_page.unwrap_or(50).clamp(1, 200);
        let offset = (page - 1) * per;
        (per, offset)
    }
}
