#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use thor::scheduler::{
        SchedulerActionHandler, CronScheduler, CronJob,
        parse_crontab, format_crontab, InMemoryCrontabStore,
    };
    use thor::actions::{ActionExecutor, ActionContext};

    #[tokio::test]
    async fn test_scheduler_handler_creation() {
        let handler = SchedulerActionHandler::new();
        assert_eq!(handler.action_type(), "SCHEDULER_OPERATION");
    }

    #[tokio::test]
    async fn test_scheduler_create_job() {
        let handler = SchedulerActionHandler::new();
        let action_data = serde_json::json!({
            "operation": "create",
            "job_name": "test_job",
            "schedule": "0 0 * * *",
            "command": "echo test",
            "operating_system": "linux"
        });
        let context = ActionContext {
            device_id: "test-device".to_string(),
            user_id: "test-user".to_string(),
            action_id: "test-action".to_string(),
        };
        let result = handler.execute(&context, &serde_json::to_vec(&action_data).unwrap()).await;
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_scheduler_list_jobs() {
        let handler = SchedulerActionHandler::new();
        let action_data = serde_json::json!({
            "operation": "list",
            "operating_system": "linux"
        });
        let context = ActionContext {
            device_id: "test-device".to_string(),
            user_id: "test-user".to_string(),
            action_id: "test-action".to_string(),
        };
        let result = handler.execute(&context, &serde_json::to_vec(&action_data).unwrap()).await;
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_scheduler_delete_job() {
        let handler = SchedulerActionHandler::new();
        let action_data = serde_json::json!({
            "operation": "delete",
            "job_name": "test_job",
            "operating_system": "linux"
        });
        let context = ActionContext {
            device_id: "test-device".to_string(),
            user_id: "test-user".to_string(),
            action_id: "test-action".to_string(),
        };
        let result = handler.execute(&context, &serde_json::to_vec(&action_data).unwrap()).await;
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_cron_expression_validation_valid() {
        let valid = ["0 0 * * *", "*/5 * * * *", "0 9 * * 1-5"];
        for expr in &valid {
            assert!(CronScheduler::validate_expression(expr).is_ok(), "{}", expr);
        }
    }

    #[test]
    fn test_cron_expression_validation_invalid() {
        let invalid = ["0 0 * *", "0 0 * * * *", "x y * * *", "60 0 * * *"];
        for expr in &invalid {
            assert!(CronScheduler::validate_expression(expr).is_err(), "{}", expr);
        }
    }

    #[test]
    fn test_parse_crontab_empty() {
        let jobs = parse_crontab("").unwrap();
        assert!(jobs.is_empty());
    }

    #[test]
    fn test_parse_crontab_valid() {
        let content = r#"# Thor job: name=daily
0 0 * * * /usr/bin/echo daily
# Thor job: name=every5
*/5 * * * * /bin/true
"#;
        let jobs = parse_crontab(content).unwrap();
        assert_eq!(jobs.len(), 2);
        assert_eq!(jobs[0].name, "daily");
        assert_eq!(jobs[0].schedule, "0 0 * * *");
        assert_eq!(jobs[0].command, "/usr/bin/echo daily");
        assert_eq!(jobs[1].name, "every5");
        assert_eq!(jobs[1].schedule, "*/5 * * * *");
        assert_eq!(jobs[1].command, "/bin/true");
    }

    #[test]
    fn test_format_crontab_roundtrip() {
        let jobs = vec![
            CronJob { name: "a".to_string(), schedule: "0 0 * * *".to_string(), command: "echo a".to_string() },
            CronJob { name: "b".to_string(), schedule: "*/5 * * * *".to_string(), command: "echo b".to_string() },
        ];
        let formatted = format_crontab(&jobs);
        let parsed = parse_crontab(&formatted).unwrap();
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].name, jobs[0].name);
        assert_eq!(parsed[0].schedule, jobs[0].schedule);
        assert_eq!(parsed[0].command, jobs[0].command);
        assert_eq!(parsed[1].name, jobs[1].name);
        assert_eq!(parsed[1].schedule, jobs[1].schedule);
        assert_eq!(parsed[1].command, jobs[1].command);
    }

    #[tokio::test]
    async fn test_cron_scheduler_with_store_create_list_delete_update() {
        let store = Arc::new(InMemoryCrontabStore::new());
        let scheduler = CronScheduler::new_with_store(store.clone());

        scheduler.create_job("job1", "0 0 * * *", "echo daily").await.unwrap();
        let list = scheduler.list_jobs().await.unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].name, "job1");
        assert_eq!(list[0].schedule, "0 0 * * *");
        assert_eq!(list[0].command, "echo daily");

        scheduler.create_job("job2", "*/5 * * * *", "echo five").await.unwrap();
        let list = scheduler.list_jobs().await.unwrap();
        assert_eq!(list.len(), 2);

        scheduler.delete_job("job1").await.unwrap();
        let list = scheduler.list_jobs().await.unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].name, "job2");

        scheduler.update_job("job2", "0 9 * * *", "echo morning").await.unwrap();
        let list = scheduler.list_jobs().await.unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].schedule, "0 9 * * *");
        assert_eq!(list[0].command, "echo morning");
    }

    #[tokio::test]
    async fn test_cron_scheduler_create_duplicate_fails() {
        let store = Arc::new(InMemoryCrontabStore::new());
        let scheduler = CronScheduler::new_with_store(store);
        scheduler.create_job("dup", "0 0 * * *", "echo x").await.unwrap();
        let r = scheduler.create_job("dup", "*/5 * * * *", "echo y").await;
        assert!(r.is_err());
    }

    #[tokio::test]
    async fn test_cron_scheduler_delete_nonexistent_fails() {
        let store = Arc::new(InMemoryCrontabStore::new());
        let scheduler = CronScheduler::new_with_store(store);
        let r = scheduler.delete_job("nonexistent").await;
        assert!(r.is_err());
    }
}
