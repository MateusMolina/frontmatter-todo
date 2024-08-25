use crate::models::Task;

pub trait TaskCollector {
    fn collect_tasks(&self) -> Vec<Task>;
}