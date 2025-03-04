use crate::infrastructure::network::{HttpMethod, Task};

pub trait TargetType {
    fn base_url(&self) -> String;
    fn path(&self) -> String;
    fn method(&self) -> HttpMethod;
    fn task(&self) -> Task;
    fn headers(&self) -> Option<Vec<(&'static str, String)>> {
        None
    }
}
