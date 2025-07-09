use crate::components::common::*;
use leptos::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub id: u32,
    pub name: String,
    pub path: String,
    pub target: String,
    pub method: String,
    pub status: String, // active, inactive
    pub rate_limit: Option<u32>,
    pub auth_required: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl Route {
    pub fn mock_data() -> Vec<Self> {
        vec![
            Route {
                id: 1,
                name: "用户登录".to_string(),
                path: "/api/auth/login".to_string(),
                target: "auth_service".to_string(),
                method: "POST".to_string(),
                status: "active".to_string(),
                rate_limit: Some(10),
                auth_required: false,
                created_at: "2024-01-01".to_string(),
                updated_at: "2024-01-15".to_string(),
            },
            Route {
                id: 2,
                name: "获取用户信息".to_string(),
                path: "/api/user/profile".to_string(),
                target: "user_service".to_string(),
                method: "GET".to_string(),
                status: "active".to_string(),
                rate_limit: Some(100),
                auth_required: true,
                created_at: "2024-01-02".to_string(),
                updated_at: "2024-01-14".to_string(),
            },
            Route {
                id: 3,
                name: "创建订单".to_string(),
                path: "/api/orders".to_string(),
                target: "order_service".to_string(),
                method: "POST".to_string(),
                status: "active".to_string(),
                rate_limit: Some(20),
                auth_required: true,
                created_at: "2024-01-03".to_string(),
                updated_at: "2024-01-16".to_string(),
            },
            Route {
                id: 4,
                name: "文件上传".to_string(),
                path: "/api/upload".to_string(),
                target: "file_service".to_string(),
                method: "POST".to_string(),
                status: "inactive".to_string(),
                rate_limit: Some(5),
                auth_required: true,
                created_at: "2024-01-04".to_string(),
                updated_at: "2024-01-17".to_string(),
            },
        ]
    }
}

#[component]
pub fn RoutesManagementPage() -> impl IntoView {
    let routes = create_rw_signal(Route::mock_data());

    let stats = create_memo(move |_| {
        let routes_data = routes.get();
        let total_routes = routes_data.len();
        let active_routes = routes_data.iter().filter(|r| r.status == "active").count();
        let auth_required_routes = routes_data.iter().filter(|r| r.auth_required).count();
        let avg_rate_limit = if total_routes > 0 {
            routes_data.iter().filter_map(|r| r.rate_limit).sum::<u32>() / total_routes as u32
        } else {
            0
        };

        (
            total_routes,
            active_routes,
            auth_required_routes,
            avg_rate_limit,
        )
    });

    let render_route_row = Box::new(|route: &Route| {
        let status_variant = match route.status.as_str() {
            "active" => "success",
            "inactive" => "error",
            _ => "info",
        };

        let method_color = match route.method.as_str() {
            "GET" => "info",
            "POST" => "success",
            "PUT" => "warning",
            "DELETE" => "error",
            _ => "info",
        };

        view! {
            <td>{route.name.clone()}</td>
            <td class="path-cell">
                <code>{route.path.clone()}</code>
            </td>
            <td>
                <span class=format!("method-badge method-{}", method_color)>
                    {route.method.clone()}
                </span>
            </td>
            <td>{route.target.clone()}</td>
            <td>
                <StatusBadge
                    status=match route.status.as_str() {
                        "active" => "启用",
                        "inactive" => "禁用",
                        _ => "未知"
                    }.to_string()
                    variant=status_variant.to_string()
                />
            </td>
            <td>
                {route.rate_limit.map(|limit| limit.to_string()).unwrap_or_else(|| "无限制".to_string())}
            </td>
            <td>
                <span class=format!("auth-badge {}", if route.auth_required { "auth-required" } else { "auth-optional" })>
                    {if route.auth_required { "需要" } else { "不需要" }}
                </span>
            </td>
        }.into_view()
    });

    let on_add = Some(Rc::new(|| {
        web_sys::console::log_1(&"添加路由".into());
    }) as Rc<dyn Fn()>);

    let on_edit = Some(Rc::new(|index: usize| {
        web_sys::console::log_2(&"编辑路由".into(), &index.to_string().into());
    }) as Rc<dyn Fn(usize)>);

    let on_delete = Some(Rc::new(move |index: usize| {
        routes.update(|routes| {
            routes.remove(index);
        });
    }) as Rc<dyn Fn(usize)>);

    view! {
        <PageTemplate title="路由管理".to_string() subtitle="管理API路由和访问控制".to_string()>
            // 统计卡片
            <div class="stats-grid">
                <StatsCard
                    title="总路由数".to_string()
                    value=Signal::derive(move || stats.get().0.to_string())
                    icon="🛣️".to_string()
                    color="blue".to_string()
                    change=None
                    change_type=None
                />
                <StatsCard
                    title="启用路由".to_string()
                    value=Signal::derive(move || stats.get().1.to_string())
                    icon="✅".to_string()
                    color="green".to_string()
                    change=None
                    change_type=None
                />
                <StatsCard
                    title="需要认证".to_string()
                    value=Signal::derive(move || stats.get().2.to_string())
                    icon="🔐".to_string()
                    color="purple".to_string()
                    change=None
                    change_type=None
                />
                <StatsCard
                    title="平均限流".to_string()
                    value=Signal::derive(move || format!("{}/min", stats.get().3))
                    icon="⚡".to_string()
                    color="orange".to_string()
                    change=None
                    change_type=None
                />
            </div>

            // 路由列表
            <div class="content-card">
                <DataTable
                    headers=vec![
                        "路由名称".to_string(),
                        "路径".to_string(),
                        "方法".to_string(),
                        "目标服务".to_string(),
                        "状态".to_string(),
                        "限流".to_string(),
                        "认证".to_string(),
                    ]
                    data=routes.read_only()
                    render_row=render_route_row
                    on_add=on_add
                    on_edit=on_edit
                    on_delete=on_delete
                />
            </div>
        </PageTemplate>
    }
}
