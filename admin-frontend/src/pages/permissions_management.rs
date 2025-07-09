use crate::components::common::*;
use leptos::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionGroup {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub user_count: u32,
    pub permissions: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl PermissionGroup {
    pub fn mock_data() -> Vec<Self> {
        vec![
            PermissionGroup {
                id: 1,
                name: "管理员".to_string(),
                description: "拥有所有权限的管理员组".to_string(),
                user_count: 2,
                permissions: vec![
                    "用户管理".to_string(),
                    "节点管理".to_string(),
                    "订单管理".to_string(),
                    "系统设置".to_string(),
                ],
                created_at: "2024-01-01".to_string(),
                updated_at: "2024-01-15".to_string(),
            },
            PermissionGroup {
                id: 2,
                name: "客服".to_string(),
                description: "处理用户问题和工单的客服组".to_string(),
                user_count: 5,
                permissions: vec![
                    "用户管理".to_string(),
                    "工单管理".to_string(),
                    "公告管理".to_string(),
                ],
                created_at: "2024-01-02".to_string(),
                updated_at: "2024-01-14".to_string(),
            },
            PermissionGroup {
                id: 3,
                name: "运维".to_string(),
                description: "管理服务器和监控的运维组".to_string(),
                user_count: 3,
                permissions: vec![
                    "节点管理".to_string(),
                    "监控查看".to_string(),
                    "日志查看".to_string(),
                ],
                created_at: "2024-01-03".to_string(),
                updated_at: "2024-01-16".to_string(),
            },
        ]
    }
}

#[component]
pub fn PermissionsManagementPage() -> impl IntoView {
    let groups = create_rw_signal(PermissionGroup::mock_data());

    let stats = create_memo(move |_| {
        let groups_data = groups.get();
        let total_groups = groups_data.len();
        let total_users = groups_data.iter().map(|g| g.user_count).sum::<u32>();
        let avg_permissions = if total_groups > 0 {
            groups_data
                .iter()
                .map(|g| g.permissions.len())
                .sum::<usize>()
                / total_groups
        } else {
            0
        };

        (total_groups, total_users, avg_permissions)
    });

    let render_group_row = Box::new(|group: &PermissionGroup| {
        view! {
            <td>{group.name.clone()}</td>
            <td>{group.description.clone()}</td>
            <td>{group.user_count}</td>
            <td>
                <div class="permissions-list">
                    {group.permissions.iter().take(3).map(|perm| {
                        view! {
                            <span class="permission-tag">{perm}</span>
                        }
                    }).collect::<Vec<_>>()}
                    {if group.permissions.len() > 3 {
                        view! {
                            <span class="permission-more">
                                {format!("+{}", group.permissions.len() - 3)}
                            </span>
                        }.into_view()
                    } else {
                        view! {}.into_view()
                    }}
                </div>
            </td>
            <td>{group.created_at.clone()}</td>
        }
        .into_view()
    });

    let on_add = Some(Rc::new(|| {
        web_sys::console::log_1(&"添加权限组".into());
    }) as Rc<dyn Fn()>);

    let on_edit = Some(Rc::new(|index: usize| {
        web_sys::console::log_2(&"编辑权限组".into(), &index.to_string().into());
    }) as Rc<dyn Fn(usize)>);

    let on_delete = Some(Rc::new(move |index: usize| {
        groups.update(|groups| {
            groups.remove(index);
        });
    }) as Rc<dyn Fn(usize)>);

    view! {
        <PageTemplate title="权限组管理".to_string() subtitle="管理用户权限组和访问控制".to_string()>
            // 统计卡片
            <div class="stats-grid">
                <StatsCard
                    title="权限组数".to_string()
                    value=Signal::derive(move || stats.get().0.to_string())
                    icon="🔐".to_string()
                    color="blue".to_string()
                    change=None
                    change_type=None
                />
                <StatsCard
                    title="总用户数".to_string()
                    value=Signal::derive(move || stats.get().1.to_string())
                    icon="👥".to_string()
                    color="green".to_string()
                    change=None
                    change_type=None
                />
                <StatsCard
                    title="平均权限数".to_string()
                    value=Signal::derive(move || stats.get().2.to_string())
                    icon="⚙️".to_string()
                    color="purple".to_string()
                    change=None
                    change_type=None
                />
            </div>

            // 权限组列表
            <div class="content-card">
                <DataTable
                    headers=vec![
                        "组名称".to_string(),
                        "描述".to_string(),
                        "用户数".to_string(),
                        "权限".to_string(),
                        "创建时间".to_string(),
                    ]
                    data=groups.read_only()
                    render_row=render_group_row
                    on_add=on_add
                    on_edit=on_edit
                    on_delete=on_delete
                />
            </div>
        </PageTemplate>
    }
}
