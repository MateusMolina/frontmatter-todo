use std::vec::Vec;
use crate::models::Task;
use crate::task_collector::TaskCollector;
use fronma::parser::parse;
pub struct FrontmatterTaskCollector<'a> {
    pub blob: Vec<&'a str>
}

impl<'a> TaskCollector for FrontmatterTaskCollector<'a> {
    fn collect_tasks(&self) -> Vec<Task> {
        let mut tasks = vec![];
        for file_blob in &self.blob {
            tasks.push(parse::<Task>(&file_blob).unwrap().headers);
        }

        return tasks;
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, NaiveDate, TimeZone, Utc};

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
   
    #[test]
    fn test_collect_tasks_with_efforts() {
        let blob = vec![
            "---\nstatus: open\nproject: TestProject\ntags: [\"test\", \"rust\"]\nefforts:\n  - project: TestProject\n    amount: 5\n    when: 2024-08-01\n---\nTask 1",
            "---\nstatus: closed\nproject: AnotherProject\ntags: [\"example\"]\nefforts:\n  - project: ProjectA\n    amount: 3\n    when: 2024-09-04\n  - project: ProjectB\n    amount: 0.5\n    when: 2023-04-04\n---\n",
        ];

        let collector = FrontmatterTaskCollector { blob };
        let tasks = collector.collect_tasks();

        assert_eq!(tasks.len(), 2);

        assert_eq!(tasks[0].status, "open");
        assert_eq!(tasks[0].project, "TestProject");
        assert_eq!(tasks[0].tags, vec!["test", "rust"]);
        assert_eq!(tasks[0].efforts.len(), 1);
        assert_eq!(tasks[0].efforts[0].project, "TestProject");
        assert_eq!(tasks[0].efforts[0].amount, 5.0);
        assert_eq!(tasks[0].efforts[0].when, NaiveDate::parse_from_str("2024-08-01", "%Y-%m-%d").unwrap());
        
        assert_eq!(tasks[1].status, "closed");
        assert_eq!(tasks[1].project, "AnotherProject");
        assert_eq!(tasks[1].tags, vec!["example"]);
        assert_eq!(tasks[1].efforts.len(), 2);
        assert_eq!(tasks[1].efforts[0].project, "ProjectA");
        assert_eq!(tasks[1].efforts[0].amount, 3.0);
        assert_eq!(tasks[1].efforts[0].when, NaiveDate::parse_from_str("2024-09-04", "%Y-%m-%d").unwrap());
        assert_eq!(tasks[1].efforts[1].project, "ProjectB");
        assert_eq!(tasks[1].efforts[1].amount, 0.5);
        assert_eq!(tasks[1].efforts[1].when, NaiveDate::parse_from_str("2023-04-04", "%Y-%m-%d").unwrap());
    } 
}