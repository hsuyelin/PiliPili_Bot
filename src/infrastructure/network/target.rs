use crate::infrastructure::network::{HttpMethod, Task};

pub trait TargetType {
    fn base_url(&self) -> &str;
    fn path(&self) -> &str;
    fn method(&self) -> HttpMethod;
    fn task(&self) -> Task;
    fn headers(&self) -> Option<Vec<(&'static str, &'static str)>> {
        None
    }
}
