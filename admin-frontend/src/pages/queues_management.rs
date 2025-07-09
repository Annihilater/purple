use crate::components::common::*;
use leptos::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueJob {
    pub id: u32,
    pub job_type: String,
    pub status: String,   // pending, processing, completed, failed
    pub priority: String, // low, medium, high, urgent
    pub payload: String,
    pub attempts: u32,
    pub max_attempts: u32,
    pub created_at: String,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub failed_at: Option<String>,
    pub error_message: Option<String>,
    pub processing_time: Option<String>,
}

impl QueueJob {
    pub fn mock_data() -> Vec<Self> {
        vec![
            QueueJob {
                id: 1,
                job_type: "用户注册邮件".to_string(),
                status: "completed".to_string(),
                priority: "medium".to_string(),
                payload: r#"{"user_id": 1001, "email": "user@example.com"}"#.to_string(),
                attempts: 1,
                max_attempts: 3,
                created_at: "2024-01-15 10:30:00".to_string(),
                started_at: Some("2024-01-15 10:30:05".to_string()),
                completed_at: Some("2024-01-15 10:30:12".to_string()),
                failed_at: None,
                error_message: None,
                processing_time: Some("7.2s".to_string()),
            },
            QueueJob {
                id: 2,
                job_type: "支付通知".to_string(),
                status: "processing".to_string(),
                priority: "high".to_string(),
                payload: r#"{"order_id": 12345, "amount": 99.0}"#.to_string(),
                attempts: 1,
                max_attempts: 5,
                created_at: "2024-01-15 11:00:00".to_string(),
                started_at: Some("2024-01-15 11:00:10".to_string()),
                completed_at: None,
                failed_at: None,
                error_message: None,
                processing_time: None,
            },
            QueueJob {
                id: 3,
                job_type: "流量统计".to_string(),
                status: "failed".to_string(),
                priority: "low".to_string(),
                payload: r#"{"user_id": 1002, "traffic_data": "..."}"#.to_string(),
                attempts: 3,
                max_attempts: 3,
                created_at: "2024-01-15 09:45:00".to_string(),
                started_at: Some("2024-01-15 09:45:15".to_string()),
                completed_at: None,
                failed_at: Some("2024-01-15 09:48:30".to_string()),
                error_message: Some("数据库连接超时".to_string()),
                processing_time: Some("3m15s".to_string()),
            },
            QueueJob {
                id: 4,
                job_type: "系统维护通知".to_string(),
                status: "pending".to_string(),
                priority: "urgent".to_string(),
                payload: r#"{"message": "系统维护通知", "target": "all_users"}"#.to_string(),
                attempts: 0,
                max_attempts: 1,
                created_at: "2024-01-15 12:00:00".to_string(),
                started_at: None,
                completed_at: None,
                failed_at: None,
                error_message: None,
                processing_time: None,
            },
            QueueJob {
                id: 5,
                job_type: "订阅到期提醒".to_string(),
                status: "completed".to_string(),
                priority: "medium".to_string(),
                payload: r#"{"user_id": 1003, "subscription_id": 456}"#.to_string(),
                attempts: 1,
                max_attempts: 3,
                created_at: "2024-01-15 08:00:00".to_string(),
                started_at: Some("2024-01-15 08:00:05".to_string()),
                completed_at: Some("2024-01-15 08:00:18".to_string()),
                failed_at: None,
                error_message: None,
                processing_time: Some("13.4s".to_string()),
            },
        ]
    }
}

#[component]
pub fn QueuesManagementPage() -> impl IntoView {
    let jobs = create_rw_signal(QueueJob::mock_data());

    let stats = create_memo(move |_| {
        let jobs_data = jobs.get();
        let total_jobs = jobs_data.len();
        let pending_jobs = jobs_data.iter().filter(|j| j.status == "pending").count();
        let processing_jobs = jobs_data
            .iter()
            .filter(|j| j.status == "processing")
            .count();
        let completed_jobs = jobs_data.iter().filter(|j| j.status == "completed").count();
        let failed_jobs = jobs_data.iter().filter(|j| j.status == "failed").count();

        (
            total_jobs,
            pending_jobs,
            processing_jobs,
            completed_jobs,
            failed_jobs,
        )
    });

    let render_job_row = Box::new(|job: &QueueJob| {
        let status_variant = match job.status.as_str() {
            "pending" => "warning",
            "processing" => "info",
            "completed" => "success",
            "failed" => "error",
            _ => "info",
        };

        let priority_variant = match job.priority.as_str() {
            "urgent" => "error",
            "high" => "warning",
            "medium" => "info",
            "low" => "success",
            _ => "info",
        };

        let progress_info = if job.status == "processing" {
            format!("进行中 ({}/{})", job.attempts, job.max_attempts)
        } else if job.status == "failed" {
            format!("失败 ({}/{})", job.attempts, job.max_attempts)
        } else {
            format!("({}/{})", job.attempts, job.max_attempts)
        };

        view! {
            <td>
                <div class="job-info">
                    <div class="job-type">{job.job_type.clone()}</div>
                    <div class="job-id">#{job.id}</div>
                </div>
            </td>
            <td>
                <StatusBadge
                    status=match job.status.as_str() {
                        "pending" => "等待中",
                        "processing" => "处理中",
                        "completed" => "已完成",
                        "failed" => "失败",
                        _ => "未知"
                    }.to_string()
                    variant=status_variant.to_string()
                />
            </td>
            <td>
                <StatusBadge
                    status=match job.priority.as_str() {
                        "urgent" => "紧急",
                        "high" => "高",
                        "medium" => "中",
                        "low" => "低",
                        _ => "未知"
                    }.to_string()
                    variant=priority_variant.to_string()
                />
            </td>
            <td>
                <div class="payload-preview">
                    {job.payload.chars().take(50).collect::<String>()}
                    {if job.payload.len() > 50 { "..." } else { "" }}
                </div>
            </td>
            <td>
                <div class="attempts-info">
                    <div class="attempts-text">{progress_info}</div>
                    <div class="attempts-bar">
                        <div
                            class="attempts-fill"
                            style=format!("width: {}%", (job.attempts as f64 / job.max_attempts as f64 * 100.0) as i32)
                        ></div>
                    </div>
                </div>
            </td>
            <td>
                <div class="time-info">
                    <div class="created-time">创建: {job.created_at.clone()}</div>
                    {job.started_at.clone().map(|started| {
                        view! {
                            <div class="started-time">开始: {started}</div>
                        }.into_view()
                    }).unwrap_or_else(|| view! {}.into_view())}
                    {job.completed_at.clone().map(|completed| {
                        view! {
                            <div class="completed-time">完成: {completed}</div>
                        }.into_view()
                    }).unwrap_or_else(|| view! {}.into_view())}
                </div>
            </td>
            <td>
                <div class="processing-info">
                    {job.processing_time.clone().unwrap_or_else(|| "-".to_string())}
                </div>
            </td>
            <td>
                {job.error_message.clone().map(|error| {
                    view! {
                        <div class="error-message" title={error.clone()}>
                            {error.chars().take(30).collect::<String>()}
                            {if error.len() > 30 { "..." } else { "" }}
                        </div>
                    }.into_view()
                }).unwrap_or_else(|| view! {<span>"-"</span>}.into_view())}
            </td>
        }.into_view()
    });

    let on_add = Some(Rc::new(|| {
        web_sys::console::log_1(&"添加队列任务".into());
    }) as Rc<dyn Fn()>);

    let on_edit = Some(Rc::new(|index: usize| {
        web_sys::console::log_2(&"编辑队列任务".into(), &index.to_string().into());
    }) as Rc<dyn Fn(usize)>);

    let on_delete = Some(Rc::new(move |index: usize| {
        jobs.update(|jobs| {
            jobs.remove(index);
        });
    }) as Rc<dyn Fn(usize)>);

    view! {
        <PageTemplate title="队列管理".to_string() subtitle="监控和管理系统队列任务".to_string()>
            // 统计卡片
            <div class="stats-grid">
                <StatsCard
                    title="总任务数".to_string()
                    value=Signal::derive(move || stats.get().0.to_string())
                    icon="📋".to_string()
                    color="blue".to_string()
                    change=None
                    change_type=None
                />
                <StatsCard
                    title="等待中".to_string()
                    value=Signal::derive(move || stats.get().1.to_string())
                    icon="⏳".to_string()
                    color="orange".to_string()
                    change=None
                    change_type=None
                />
                <StatsCard
                    title="处理中".to_string()
                    value=Signal::derive(move || stats.get().2.to_string())
                    icon="🔄".to_string()
                    color="info".to_string()
                    change=None
                    change_type=None
                />
                <StatsCard
                    title="已完成".to_string()
                    value=Signal::derive(move || stats.get().3.to_string())
                    icon="✅".to_string()
                    color="green".to_string()
                    change=None
                    change_type=None
                />
            </div>

            // 队列任务列表
            <div class="content-card">
                <DataTable
                    headers=vec![
                        "任务信息".to_string(),
                        "状态".to_string(),
                        "优先级".to_string(),
                        "载荷".to_string(),
                        "尝试次数".to_string(),
                        "时间信息".to_string(),
                        "处理时间".to_string(),
                        "错误信息".to_string(),
                    ]
                    data=jobs.read_only()
                    render_row=render_job_row
                    on_add=on_add
                    on_edit=on_edit
                    on_delete=on_delete
                />
            </div>
        </PageTemplate>
    }
}
