use crate::components::common::*;
use leptos::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerNode {
    pub id: u32,
    pub name: String,
    pub location: String,
    pub address: String,
    pub port: u16,
    pub protocol: String,
    pub status: String, // online, offline, maintenance
    pub load: f32,
    pub traffic_today: String,
    pub traffic_total: String,
    pub online_users: u32,
    pub created_at: String,
    pub updated_at: String,
}

impl ServerNode {
    pub fn mock_data() -> Vec<Self> {
        vec![
            ServerNode {
                id: 1,
                name: "香港节点1".to_string(),
                location: "香港".to_string(),
                address: "hk1.example.com".to_string(),
                port: 443,
                protocol: "VMess".to_string(),
                status: "online".to_string(),
                load: 0.25,
                traffic_today: "12.3 GB".to_string(),
                traffic_total: "1.2 TB".to_string(),
                online_users: 156,
                created_at: "2024-01-01".to_string(),
                updated_at: "2024-01-15".to_string(),
            },
            ServerNode {
                id: 2,
                name: "新加坡节点1".to_string(),
                location: "新加坡".to_string(),
                address: "sg1.example.com".to_string(),
                port: 443,
                protocol: "Trojan".to_string(),
                status: "online".to_string(),
                load: 0.45,
                traffic_today: "8.7 GB".to_string(),
                traffic_total: "890 GB".to_string(),
                online_users: 89,
                created_at: "2024-01-02".to_string(),
                updated_at: "2024-01-14".to_string(),
            },
            ServerNode {
                id: 3,
                name: "美国节点1".to_string(),
                location: "美国".to_string(),
                address: "us1.example.com".to_string(),
                port: 443,
                protocol: "Shadowsocks".to_string(),
                status: "maintenance".to_string(),
                load: 0.0,
                traffic_today: "0 GB".to_string(),
                traffic_total: "2.1 TB".to_string(),
                online_users: 0,
                created_at: "2024-01-03".to_string(),
                updated_at: "2024-01-16".to_string(),
            },
        ]
    }
}

#[component]
pub fn NodesManagementPage() -> impl IntoView {
    let nodes = create_rw_signal(ServerNode::mock_data());

    let stats = create_memo(move |_| {
        let nodes_data = nodes.get();
        let total_nodes = nodes_data.len();
        let online_nodes = nodes_data.iter().filter(|n| n.status == "online").count();
        let total_users = nodes_data.iter().map(|n| n.online_users).sum::<u32>();
        let avg_load = if total_nodes > 0 {
            nodes_data.iter().map(|n| n.load).sum::<f32>() / total_nodes as f32
        } else {
            0.0
        };

        (total_nodes, online_nodes, total_users, avg_load)
    });

    let render_node_row = Box::new(|node: &ServerNode| {
        let status_variant = match node.status.as_str() {
            "online" => "success",
            "offline" => "error",
            "maintenance" => "warning",
            _ => "info",
        };

        let load_color = if node.load < 0.5 {
            "success"
        } else if node.load < 0.8 {
            "warning"
        } else {
            "error"
        };

        view! {
            <td>{node.name.clone()}</td>
            <td>{node.location.clone()}</td>
            <td>{format!("{}:{}", node.address, node.port)}</td>
            <td>{node.protocol.clone()}</td>
            <td>
                <StatusBadge
                    status=match node.status.as_str() {
                        "online" => "在线",
                        "offline" => "离线",
                        "maintenance" => "维护中",
                        _ => "未知"
                    }.to_string()
                    variant=status_variant.to_string()
                />
            </td>
            <td>
                <div class="load-indicator">
                    <div class="load-bar">
                        <div
                            class=format!("load-fill load-{}", load_color)
                            style=format!("width: {}%", (node.load * 100.0) as i32)
                        ></div>
                    </div>
                    <span class="load-text">{format!("{:.1}%", node.load * 100.0)}</span>
                </div>
            </td>
            <td>{node.online_users}</td>
            <td>{node.traffic_today.clone()}</td>
        }
        .into_view()
    });

    let on_add = Some(Rc::new(|| {
        web_sys::console::log_1(&"添加节点".into());
    }) as Rc<dyn Fn()>);

    let on_edit = Some(Rc::new(|index: usize| {
        web_sys::console::log_2(&"编辑节点".into(), &index.to_string().into());
    }) as Rc<dyn Fn(usize)>);

    let on_delete = Some(Rc::new(move |index: usize| {
        nodes.update(|nodes| {
            nodes.remove(index);
        });
    }) as Rc<dyn Fn(usize)>);

    view! {
        <PageTemplate title="节点管理".to_string() subtitle="管理和监控服务器节点状态".to_string()>
            // 统计卡片
            <div class="stats-grid">
                <StatsCard
                    title="总节点数".to_string()
                    value=Signal::derive(move || stats.get().0.to_string())
                    icon="🖥️".to_string()
                    color="blue".to_string()
                    change=None
                    change_type=None
                />
                <StatsCard
                    title="在线节点".to_string()
                    value=Signal::derive(move || stats.get().1.to_string())
                    icon="🟢".to_string()
                    color="green".to_string()
                    change=None
                    change_type=None
                />
                <StatsCard
                    title="在线用户".to_string()
                    value=Signal::derive(move || stats.get().2.to_string())
                    icon="👥".to_string()
                    color="purple".to_string()
                    change=None
                    change_type=None
                />
                <StatsCard
                    title="平均负载".to_string()
                    value=Signal::derive(move || format!("{:.1}%", stats.get().3 * 100.0))
                    icon="📊".to_string()
                    color="orange".to_string()
                    change=None
                    change_type=None
                />
            </div>

            // 节点列表
            <div class="content-card">
                <DataTable
                    headers=vec![
                        "节点名称".to_string(),
                        "位置".to_string(),
                        "地址".to_string(),
                        "协议".to_string(),
                        "状态".to_string(),
                        "负载".to_string(),
                        "在线用户".to_string(),
                        "今日流量".to_string(),
                    ]
                    data=nodes.read_only()
                    render_row=render_node_row
                    on_add=on_add
                    on_edit=on_edit
                    on_delete=on_delete
                />
            </div>
        </PageTemplate>
    }
}
