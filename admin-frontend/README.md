# Purple Admin Frontend

基于 Leptos 构建的现代化网络服务管理后台系统，提供完整的管理员界面和用户体验。

## 🚀 项目概述

Purple Admin Frontend 是一个使用 Rust 和 Leptos 框架构建的单页面应用程序（SPA），专为网络服务提供商设计的管理后台系统。该项目采用现代化的设计理念，提供直观、高效的管理界面。

## ✨ 核心特性

### 🎯 管理功能
- **系统设置管理** - 站点配置、邮件设置、支付配置、安全设置
- **用户管理** - 用户账户、权限管理、流量统计
- **服务器管理** - 节点管理、权限组管理、路由管理
- **财务管理** - 订阅管理、订单管理、优惠券管理
- **内容管理** - 公告管理、工单管理、知识库管理
- **系统监控** - 队列管理、系统信息、性能监控

### 🎨 用户体验
- **响应式设计** - 支持桌面端、平板端、移动端
- **深色主题** - 完整的深色/浅色主题切换
- **实时搜索** - 智能搜索和多维度筛选
- **数据可视化** - 统计图表和进度条显示
- **模态框操作** - 现代化的交互体验

### 🔧 技术特性
- **类型安全** - 完整的 Rust 类型系统
- **响应式状态管理** - 基于 Leptos 信号系统
- **组件化架构** - 可复用的 UI 组件
- **CSS 变量系统** - 主题化设计支持
- **模块化路由** - 清晰的页面结构

## 🏗️ 技术栈

### 核心框架
- **[Leptos](https://leptos.dev/)** - 现代化的 Rust Web 框架
- **[Rust](https://www.rust-lang.org/)** - 系统编程语言
- **[WebAssembly](https://webassembly.org/)** - 高性能 Web 应用

### 开发工具
- **[Trunk](https://trunkrs.dev/)** - WebAssembly 构建工具
- **[Serde](https://serde.rs/)** - 序列化/反序列化
- **[Leptos Router](https://docs.rs/leptos_router/)** - 客户端路由

### 样式系统
- **CSS 自定义属性** - 主题化设计
- **Flexbox & Grid** - 现代布局系统
- **渐变背景** - 视觉效果增强
- **动画过渡** - 流畅的用户体验

## 📦 项目结构

```
src/
├── components/           # 可复用组件
│   ├── common.rs        # 通用组件（DataTable、StatsCard等）
│   ├── layout.rs        # 布局组件
│   ├── sidebar.rs       # 侧边栏导航
│   └── mod.rs           # 模块导出
├── pages/               # 页面组件
│   ├── dashboard.rs     # 仪表板
│   ├── system_settings.rs # 系统设置
│   ├── coupons.rs       # 优惠券管理
│   ├── users_management.rs # 用户管理
│   ├── nodes_management.rs # 节点管理
│   ├── orders_management.rs # 订单管理
│   └── ...              # 其他管理页面
├── services/            # 服务层
│   ├── auth.rs          # 认证服务
│   └── mod.rs           # 模块导出
├── utils/               # 工具函数
│   ├── theme.rs         # 主题管理
│   └── mod.rs           # 模块导出
├── lib.rs               # 应用入口和路由配置
└── main.rs              # 主函数
```

## 🎯 核心页面

### 📊 仪表板
- 系统概览和关键指标
- 实时数据展示
- 快速操作入口

### 🔧 系统设置
- **站点设置** - 基本信息、Logo、联系方式
- **邮件设置** - SMTP 配置、测试邮件
- **支付设置** - 多种支付方式配置
- **安全设置** - 密码策略、验证设置

### 🎫 优惠券管理
- 完整的优惠券 CRUD 操作
- 支持固定金额和百分比折扣
- 使用次数限制和用户限制
- 有效期管理和状态控制
- 实时搜索和筛选功能

### 👥 用户管理
- 用户账户信息管理
- 流量使用统计
- 权限和角色管理
- 推荐关系追踪

### 🖥️ 服务器管理
- **节点管理** - 服务器节点监控
- **权限组管理** - 角色权限配置
- **路由管理** - API 路由配置

### 💰 财务管理
- **订阅管理** - 用户订阅状态
- **订单管理** - 订单处理和追踪
- **优惠券管理** - 促销活动管理

## 🚀 快速开始

### 环境要求
- Rust 1.70+
- Node.js 18+
- Trunk 0.17+

### 安装依赖
```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 Trunk
cargo install trunk

# 添加 WebAssembly 目标
rustup target add wasm32-unknown-unknown
```

### 运行项目
```bash
# 克隆项目
git clone <repository-url>
cd purple/admin-frontend

# 开发模式运行
trunk serve

# 生产构建
trunk build --release
```

### 访问应用
- 开发服务器: http://localhost:8080
- 默认登录: admin / admin123

## 🎨 主题系统

### 主题切换
```rust
// 主题管理服务
use crate::utils::theme::ThemeService;

// 切换主题
theme_service.toggle_theme();

// 设置特定主题
theme_service.set_theme("dark");
```

### CSS 变量
```css
/* 主题变量 */
:root {
    --color-primary: #667eea;
    --color-secondary: #764ba2;
    --color-text-primary: #1a202c;
    --color-surface: #ffffff;
}

.dark-theme {
    --color-text-primary: #e2e8f0;
    --color-surface: #2d3748;
}
```

## 📱 响应式设计

### 断点系统
- **桌面端**: 1024px+
- **平板端**: 768px - 1023px
- **移动端**: 320px - 767px

### 组件适配
```rust
// 响应式组件示例
view! {
    <div class="stats-grid">
        // 自动适配不同屏幕尺寸
        <StatsCard ... />
    </div>
}
```

## 🛠️ 开发指南

### 代码格式化
```bash
# 格式化代码
cargo fmt --all

# 检查代码
cargo clippy
```

### 组件开发
```rust
// 创建新组件
#[component]
pub fn MyComponent(
    title: String,
    data: ReadSignal<Vec<Item>>,
) -> impl IntoView {
    view! {
        <div class="my-component">
            <h2>{title}</h2>
            // 组件内容
        </div>
    }
}
```

### 状态管理
```rust
// 创建响应式状态
let (data, set_data) = create_signal(initial_value);

// 计算属性
let filtered_data = create_memo(move |_| {
    data.get().into_iter().filter(|item| {
        // 筛选逻辑
    }).collect()
});

// 副作用
create_effect(move |_| {
    // 响应状态变化
});
```

## 🔒 安全特性

### 认证系统
- JWT 令牌认证
- 会话管理
- 权限验证

### 数据保护
- 输入验证
- XSS 防护
- CSRF 保护

## 📈 性能优化

### WebAssembly 优化
- 代码分割
- 懒加载
- 资源压缩

### 渲染优化
- 虚拟滚动
- 组件缓存
- 状态优化

## 🧪 测试

### 单元测试
```bash
# 运行测试
cargo test

# 测试覆盖率
cargo tarpaulin
```

### 端到端测试
```bash
# 安装测试工具
cargo install wasm-pack

# 运行 E2E 测试
wasm-pack test --headless --chrome
```

## 📝 开发规范

### 代码风格
- 遵循 Rust 官方样式指南
- 使用 rustfmt 格式化
- 通过 clippy 检查

### 提交规范
- 使用语义化提交信息
- 每次提交前运行 `cargo fmt --all`
- 确保代码可以编译通过

### 文档编写
- API 文档使用 rustdoc
- 组件文档包含使用示例
- 重要功能提供实现说明

## 🚀 部署

### 构建生产版本
```bash
# 生产构建
trunk build --release

# 输出目录
ls dist/
```

### 部署到静态服务器
```bash
# 部署到 Nginx
cp -r dist/* /var/www/html/

# 部署到 CDN
aws s3 sync dist/ s3://your-bucket/
```

## 🤝 贡献指南

### 开发流程
1. Fork 项目
2. 创建特性分支
3. 提交代码
4. 创建 Pull Request

### 问题报告
- 使用 GitHub Issues
- 提供详细的复现步骤
- 包含环境信息

## 📄 许可证

MIT License - 详见 [LICENSE](LICENSE) 文件

## 🙏 致谢

感谢以下开源项目和贡献者：
- [Leptos](https://leptos.dev/) - 现代化的 Rust Web 框架
- [Trunk](https://trunkrs.dev/) - WebAssembly 构建工具
- [Rust Community](https://www.rust-lang.org/community) - 优秀的 Rust 社区

---

**Purple Admin Frontend** - 现代化的网络服务管理后台系统 🚀