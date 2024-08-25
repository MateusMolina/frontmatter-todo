use std::vec::Vec;
use crate::models::Task;
use crate::task_collector::TaskCollector;
pub struct FrontmatterTaskCollector<'a> {
    pub blob: Vec<&'a str>
}

impl<'a> TaskCollector for FrontmatterTaskCollector<'a> {
    fn collect_tasks(&self) -> Vec<Task> {
        let tasks: Vec<Task> = Vec::new();

        tasks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_tasks_empty_blob() {
        let collector = FrontmatterTaskCollector { blob: vec![] };
        let tasks = collector.collect_tasks();
        assert!(tasks.is_empty());
    }

    #[test]
    fn test_collect_tasks_valid_blob() {
        let blob = vec![
            "---\nstatus: open\nproject: TestProject\ntags: [\"test\", \"rust\"]\n---\nTask 1",
            "---\nstatus: closed\nproject: AnotherProject\ntags: [\"example\"]\n---\nTask 2",
            ];

        let collector = FrontmatterTaskCollector { blob };
        let tasks = collector.collect_tasks();

        assert_eq!(tasks.len(), 2);

        assert_eq!(tasks[0].status, "open");
        assert_eq!(tasks[0].project, "TestProject");
        assert_eq!(tasks[0].tags, vec!["test", "rust"]);

        assert_eq!(tasks[1].status, "closed");
        assert_eq!(tasks[1].project, "AnotherProject");
        assert_eq!(tasks[1].tags, vec!["example"]);
    }
}