use super::manager::ThemeManager;
use super::types::{Theme, ThemeMode};
use leptos::*;
use std::cell::RefCell;
use std::rc::Rc;

pub fn use_theme_system() -> (
    ReadSignal<Theme>,
    impl Fn(&str) + Clone,
    impl Fn(ThemeMode) + Clone,
    impl Fn() + Clone,
    ReadSignal<Vec<String>>,
) {
    let manager = Rc::new(RefCell::new(ThemeManager::new()));

    // 从存储中加载主题设置
    manager.borrow_mut().load_from_storage();

    let (theme, set_theme_signal) = create_signal(manager.borrow().get_current_theme());
    let (available_themes, set_available_themes) = create_signal(
        manager
            .borrow()
            .get_available_themes()
            .iter()
            .map(|t| t.id.clone())
            .collect::<Vec<String>>(),
    );

    // 设置主题
    let set_theme = {
        let manager = manager.clone();
        let set_theme_signal = set_theme_signal;
        move |theme_id: &str| {
            if manager.borrow_mut().set_theme(theme_id) {
                let current_theme = manager.borrow().get_current_theme();
                manager.borrow().apply_theme(&current_theme);
                set_theme_signal.set(current_theme);
            }
        }
    };

    // 设置模式
    let set_mode = {
        let manager = manager.clone();
        let set_theme_signal = set_theme_signal;
        move |mode: ThemeMode| {
            manager.borrow_mut().set_mode(mode);
            let current_theme = manager.borrow().get_current_theme();
            manager.borrow().apply_theme(&current_theme);
            set_theme_signal.set(current_theme);
        }
    };

    // 切换模式
    let toggle_mode = {
        let manager = manager.clone();
        let set_theme_signal = set_theme_signal;
        move || {
            manager.borrow_mut().toggle_mode();
            let current_theme = manager.borrow().get_current_theme();
            manager.borrow().apply_theme(&current_theme);
            set_theme_signal.set(current_theme);
        }
    };

    // 初始化时应用主题
    create_effect({
        let manager = manager.clone();
        move |_| {
            let current_theme = manager.borrow().get_current_theme();
            manager.borrow().apply_theme(&current_theme);
        }
    });

    (theme, set_theme, set_mode, toggle_mode, available_themes)
}

// 简化版本的主题hook，兼容现有代码
pub fn use_theme() -> (impl Fn() -> ThemeMode + Clone, impl Fn() + Clone) {
    let (theme, _set_theme, _set_mode, toggle_mode, _available) = use_theme_system();

    let get_mode = move || theme.get().mode;

    (get_mode, toggle_mode)
}
