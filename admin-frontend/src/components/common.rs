use leptos::*;
use std::rc::Rc;

#[component]
pub fn PageTemplate(title: String, subtitle: String, children: Children) -> impl IntoView {
    view! {
        <div class="page-template">
            <div class="page-header">
                <h1 class="page-title">{title}</h1>
                <p class="page-subtitle">{subtitle}</p>
            </div>
            <div class="page-content">
                {children()}
            </div>
        </div>
    }
}

#[component]
pub fn DataTable<T>(
    headers: Vec<String>,
    data: ReadSignal<Vec<T>>,
    render_row: Box<dyn Fn(&T) -> View>,
    on_add: Option<Rc<dyn Fn()>>,
    on_edit: Option<Rc<dyn Fn(usize)>>,
    on_delete: Option<Rc<dyn Fn(usize)>>,
) -> impl IntoView
where
    T: Clone + 'static,
{
    let show_add_modal = create_rw_signal(false);
    let show_edit_modal = create_rw_signal(false);
    let selected_index = create_rw_signal(None::<usize>);

    view! {
        <div class="data-table-container">
            <div class="table-header">
                <div class="table-title">
                    <span class="table-count">{move || format!("共 {} 条记录", data.get().len())}</span>
                </div>
                <div class="table-actions">
                    {on_add.map(|add_fn| {
                        let add_fn = add_fn.clone();
                        view! {
                            <button
                                class="btn btn-primary"
                                on:click=move |_| {
                                    add_fn();
                                    show_add_modal.set(true);
                                }
                            >
                                <span class="btn-icon">"+"</span>
                                "添加"
                            </button>
                        }
                    })}
                    <button class="btn btn-secondary">
                        <span class="btn-icon">"↻"</span>
                        "刷新"
                    </button>
                </div>
            </div>

            <div class="table-wrapper">
                <table class="data-table">
                    <thead>
                        <tr>
                            {headers.iter().map(|header| {
                                view! {
                                    <th>{header}</th>
                                }
                            }).collect::<Vec<_>>()}
                            <th class="actions-column">"操作"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {move || {
                            data.get().iter().enumerate().map(|(index, item)| {
                                let item_clone = item.clone();
                                view! {
                                    <tr>
                                        {render_row(&item_clone)}
                                        <td class="actions-cell">
                                            <div class="action-buttons">
                                                {on_edit.as_ref().map(|edit_fn| {
                                                    let edit_fn = edit_fn.clone();
                                                    view! {
                                                        <button
                                                            class="btn btn-sm btn-secondary"
                                                            on:click=move |_| {
                                                                selected_index.set(Some(index));
                                                                edit_fn(index);
                                                                show_edit_modal.set(true);
                                                            }
                                                        >
                                                            "编辑"
                                                        </button>
                                                    }
                                                })}
                                                {on_delete.as_ref().map(|delete_fn| {
                                                    let delete_fn = delete_fn.clone();
                                                    view! {
                                                        <button
                                                            class="btn btn-sm btn-danger"
                                                            on:click=move |_| {
                                                                if web_sys::window().unwrap().confirm_with_message("确定要删除这条记录吗？").unwrap() {
                                                                    delete_fn(index);
                                                                }
                                                            }
                                                        >
                                                            "删除"
                                                        </button>
                                                    }
                                                })}
                                            </div>
                                        </td>
                                    </tr>
                                }
                            }).collect::<Vec<_>>()
                        }}
                    </tbody>
                </table>
            </div>
        </div>
    }
}

#[component]
pub fn StatusBadge(
    status: String,
    variant: String, // success, warning, error, info
) -> impl IntoView {
    view! {
        <span class=format!("status-badge {}", variant)>
            {status}
        </span>
    }
}

#[component]
pub fn ActionButton(
    label: String,
    icon: String,
    variant: String, // primary, secondary, success, warning, danger
    on_click: Rc<dyn Fn()>,
) -> impl IntoView {
    view! {
        <button class=format!("btn btn-{}", variant) on:click=move |_| on_click()>
            <span class="btn-icon">{icon}</span>
            {label}
        </button>
    }
}

#[component]
pub fn EmptyState(
    title: String,
    description: String,
    action_label: Option<String>,
    on_action: Option<Rc<dyn Fn()>>,
) -> impl IntoView {
    view! {
        <div class="empty-state">
            <div class="empty-state-icon">"📋"</div>
            <h3 class="empty-state-title">{title}</h3>
            <p class="empty-state-description">{description}</p>
            {match (action_label, on_action) {
                (Some(label), Some(action)) => {
                    view! {
                        <button class="btn btn-primary" on:click=move |_| action()>
                            {label}
                        </button>
                    }.into_view()
                }
                _ => view! {}.into_view()
            }}
        </div>
    }
}

#[component]
pub fn StatsCard(
    title: String,
    value: Signal<String>,
    icon: String,
    color: String,
    change: Option<String>,
    change_type: Option<String>, // up, down, neutral
) -> impl IntoView {
    view! {
        <div class="stats-card">
            <div class="stats-card-header">
                <div class="stats-card-title">{title}</div>
                <div class=format!("stats-card-icon stats-card-icon-{}", color)>
                    {icon}
                </div>
            </div>
            <div class="stats-card-content">
                <div class="stats-card-value">{value}</div>
                {match (change, change_type) {
                    (Some(change_val), Some(change_type_val)) => {
                        let change_class = match change_type_val.as_str() {
                            "up" => "positive",
                            "down" => "negative",
                            _ => "neutral"
                        };
                        view! {
                            <div class=format!("stats-card-change {}", change_class)>
                                <span class="change-icon">{match change_type_val.as_str() {
                                    "up" => "↗",
                                    "down" => "↘",
                                    _ => "→"
                                }}</span>
                                {change_val}
                            </div>
                        }.into_view()
                    }
                    _ => view! {}.into_view()
                }}
            </div>
        </div>
    }
}
