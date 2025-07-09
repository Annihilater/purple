use crate::components::common::*;
use leptos::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeArticle {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub category: String,
    pub tags: Vec<String>,
    pub status: String, // published, draft, archived
    pub author: String,
    pub views: u32,
    pub likes: u32,
    pub language: String,
    pub sort_order: u32,
    pub featured: bool,
    pub created_at: String,
    pub updated_at: String,
    pub published_at: Option<String>,
}

impl KnowledgeArticle {
    pub fn mock_data() -> Vec<Self> {
        vec![
            KnowledgeArticle {
                id: 1,
                title: "如何配置客户端连接".to_string(),
                content: "本文详细介绍了如何在各种设备上配置客户端连接，包括Windows、macOS、iOS、Android等平台的详细步骤。".to_string(),
                category: "使用指南".to_string(),
                tags: vec!["客户端".to_string(), "配置".to_string(), "连接".to_string()],
                status: "published".to_string(),
                author: "技术支持".to_string(),
                views: 1256,
                likes: 89,
                language: "zh-CN".to_string(),
                sort_order: 1,
                featured: true,
                created_at: "2024-01-10 09:00:00".to_string(),
                updated_at: "2024-01-15 14:30:00".to_string(),
                published_at: Some("2024-01-10 10:00:00".to_string()),
            },
            KnowledgeArticle {
                id: 2,
                title: "常见连接问题排查".to_string(),
                content: "当您遇到连接问题时，可以按照以下步骤进行排查：1. 检查网络连接 2. 验证服务器状态 3. 检查客户端配置...".to_string(),
                category: "故障排除".to_string(),
                tags: vec!["故障".to_string(), "排查".to_string(), "连接".to_string()],
                status: "published".to_string(),
                author: "技术支持".to_string(),
                views: 2341,
                likes: 156,
                language: "zh-CN".to_string(),
                sort_order: 2,
                featured: true,
                created_at: "2024-01-08 14:20:00".to_string(),
                updated_at: "2024-01-12 11:45:00".to_string(),
                published_at: Some("2024-01-08 15:00:00".to_string()),
            },
            KnowledgeArticle {
                id: 3,
                title: "账户管理和设置".to_string(),
                content: "了解如何管理您的账户设置，包括修改密码、更新个人信息、查看使用统计等功能。".to_string(),
                category: "账户管理".to_string(),
                tags: vec!["账户".to_string(), "设置".to_string(), "管理".to_string()],
                status: "published".to_string(),
                author: "产品团队".to_string(),
                views: 987,
                likes: 67,
                language: "zh-CN".to_string(),
                sort_order: 3,
                featured: false,
                created_at: "2024-01-05 16:30:00".to_string(),
                updated_at: "2024-01-14 09:15:00".to_string(),
                published_at: Some("2024-01-05 17:00:00".to_string()),
            },
            KnowledgeArticle {
                id: 4,
                title: "新手入门指南".to_string(),
                content: "欢迎使用我们的服务！本文将为您介绍从注册到首次使用的完整流程，帮助您快速上手。".to_string(),
                category: "新手指南".to_string(),
                tags: vec!["新手".to_string(), "入门".to_string(), "指南".to_string()],
                status: "draft".to_string(),
                author: "产品团队".to_string(),
                views: 0,
                likes: 0,
                language: "zh-CN".to_string(),
                sort_order: 4,
                featured: false,
                created_at: "2024-01-16 10:00:00".to_string(),
                updated_at: "2024-01-16 10:00:00".to_string(),
                published_at: None,
            },
            KnowledgeArticle {
                id: 5,
                title: "API 文档和开发者指南".to_string(),
                content: "面向开发者的API文档，包含完整的接口说明、示例代码和最佳实践。".to_string(),
                category: "开发者".to_string(),
                tags: vec!["API".to_string(), "开发".to_string(), "文档".to_string()],
                status: "published".to_string(),
                author: "技术团队".to_string(),
                views: 543,
                likes: 78,
                language: "zh-CN".to_string(),
                sort_order: 5,
                featured: false,
                created_at: "2024-01-12 13:45:00".to_string(),
                updated_at: "2024-01-15 16:20:00".to_string(),
                published_at: Some("2024-01-12 14:00:00".to_string()),
            },
        ]
    }
}

#[component]
pub fn KnowledgeManagementPage() -> impl IntoView {
    let articles = create_rw_signal(KnowledgeArticle::mock_data());

    let stats = create_memo(move |_| {
        let articles_data = articles.get();
        let total_articles = articles_data.len();
        let published_articles = articles_data
            .iter()
            .filter(|a| a.status == "published")
            .count();
        let draft_articles = articles_data.iter().filter(|a| a.status == "draft").count();
        let total_views = articles_data.iter().map(|a| a.views).sum::<u32>();
        let total_likes = articles_data.iter().map(|a| a.likes).sum::<u32>();

        (
            total_articles,
            published_articles,
            draft_articles,
            total_views,
            total_likes,
        )
    });

    let render_article_row = Box::new(|article: &KnowledgeArticle| {
        let status_variant = match article.status.as_str() {
            "published" => "success",
            "draft" => "warning",
            "archived" => "info",
            _ => "info",
        };

        view! {
            <td>
                <div class="article-info">
                    <div class="article-title">
                        {article.title.clone()}
                        {if article.featured {
                            view! {
                                <span class="featured-badge">{"🌟"}</span>
                            }.into_view()
                        } else {
                            view! {}.into_view()
                        }}
                    </div>
                    <div class="article-content-preview">
                        {article.content.chars().take(80).collect::<String>()}
                        {if article.content.len() > 80 { "..." } else { "" }}
                    </div>
                    <div class="article-tags">
                        {article.tags.iter().map(|tag| {
                            view! {
                                <span class="tag">{tag.clone()}</span>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </div>
            </td>
            <td>{article.category.clone()}</td>
            <td>
                <StatusBadge
                    status=match article.status.as_str() {
                        "published" => "已发布",
                        "draft" => "草稿",
                        "archived" => "已归档",
                        _ => "未知"
                    }.to_string()
                    variant=status_variant.to_string()
                />
            </td>
            <td>{article.author.clone()}</td>
            <td>
                <div class="engagement-stats">
                    <div class="views">{"👁️"} {article.views}</div>
                    <div class="likes">{"👍"} {article.likes}</div>
                </div>
            </td>
            <td>{article.language.clone()}</td>
            <td class="sort-order-cell">
                <span class="sort-order">{article.sort_order}</span>
            </td>
            <td>
                <div class="date-info">
                    <div class="created-date">创建: {article.created_at.clone()}</div>
                    <div class="updated-date">更新: {article.updated_at.clone()}</div>
                    {article.published_at.clone().map(|published| {
                        view! {
                            <div class="published-date">发布: {published}</div>
                        }.into_view()
                    }).unwrap_or_else(|| view! {}.into_view())}
                </div>
            </td>
        }
        .into_view()
    });

    let on_add = Some(Rc::new(|| {
        web_sys::console::log_1(&"添加知识库文章".into());
    }) as Rc<dyn Fn()>);

    let on_edit = Some(Rc::new(|index: usize| {
        web_sys::console::log_2(&"编辑知识库文章".into(), &index.to_string().into());
    }) as Rc<dyn Fn(usize)>);

    let on_delete = Some(Rc::new(move |index: usize| {
        articles.update(|articles| {
            articles.remove(index);
        });
    }) as Rc<dyn Fn(usize)>);

    view! {
        <PageTemplate title="知识库管理".to_string() subtitle="管理帮助文档和知识库内容".to_string()>
            // 统计卡片
            <div class="stats-grid">
                <StatsCard
                    title="总文章数".to_string()
                    value=Signal::derive(move || stats.get().0.to_string())
                    icon="📚".to_string()
                    color="blue".to_string()
                    change=None
                    change_type=None
                />
                <StatsCard
                    title="已发布".to_string()
                    value=Signal::derive(move || stats.get().1.to_string())
                    icon="✅".to_string()
                    color="green".to_string()
                    change=None
                    change_type=None
                />
                <StatsCard
                    title="草稿".to_string()
                    value=Signal::derive(move || stats.get().2.to_string())
                    icon="📝".to_string()
                    color="orange".to_string()
                    change=None
                    change_type=None
                />
                <StatsCard
                    title="总浏览量".to_string()
                    value=Signal::derive(move || stats.get().3.to_string())
                    icon="👁️".to_string()
                    color="purple".to_string()
                    change=Some("+15.2%".to_string())
                    change_type=Some("up".to_string())
                />
            </div>

            // 知识库文章列表
            <div class="content-card">
                <DataTable
                    headers=vec![
                        "文章信息".to_string(),
                        "分类".to_string(),
                        "状态".to_string(),
                        "作者".to_string(),
                        "互动数据".to_string(),
                        "语言".to_string(),
                        "排序".to_string(),
                        "时间信息".to_string(),
                    ]
                    data=articles.read_only()
                    render_row=render_article_row
                    on_add=on_add
                    on_edit=on_edit
                    on_delete=on_delete
                />
            </div>
        </PageTemplate>
    }
}
