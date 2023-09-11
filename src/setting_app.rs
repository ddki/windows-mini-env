extern crate native_windows_gui as nwg;
use std::{cell::RefCell, env, ops::Deref, rc::Rc};

use nwg::{CheckBoxState, NativeUi};
use winreg::{
    enums::{HKEY_CLASSES_ROOT, KEY_ALL_ACCESS},
    RegKey,
};

#[derive(Default)]
pub struct SettingApp {
    window: nwg::Window,
    layout: nwg::GridLayout,

    add_right_click_menu_label: nwg::Label,
    key_select_label: nwg::Label,

    add_right_click_menu_input: nwg::CheckBox,
    key_select_input: nwg::TextInput,

    save_btn: nwg::Button,
}

impl SettingApp {
    fn save(&self) {
        // 环境变量选择列表
        let key_select = self.key_select_input.text();
        // 添加右键菜单
        let add_right_click_menu =
            self.add_right_click_menu_input.check_state() == CheckBoxState::Checked;
        if add_right_click_menu {
            const SUB_HKEY: &str = "Directory\\shell";
            // 打开注册表
            let hklm = RegKey::predef(HKEY_CLASSES_ROOT);
            let cur_cer = hklm
                .open_subkey_with_flags(SUB_HKEY, KEY_ALL_ACCESS)
                .unwrap();
            if cur_cer.create_subkey("miniEnv").is_ok() {
                let env_cer = hklm
                    .open_subkey_with_flags(format!("{}\\{}", SUB_HKEY, "miniEnv"), KEY_ALL_ACCESS)
                    .unwrap();
                // 获取程序路径
                let exe_path =
                    env::current_exe().expect("Failed to retrieve the current executable path");
                let exe_path_str = exe_path.to_string_lossy();
                let _ = env_cer.set_value(
                    "添加到PATH环境变量",
                    &format!("{} --o modify --k PATH --v %1", exe_path_str),
                );
                let _ = env_cer.set_value(
                    "添加到当前环境变量(ui)",
                    &format!("{} --o MODIFY --v %1", exe_path_str),
                );
                let _ = env_cer.set_value(
                    "添加到新的环境变量(ui)",
                    &format!("{} --o NEW --v %1", exe_path_str),
                );
            }
        }
        nwg::modal_info_message(&self.window, "info", "保存成功");
    }
    fn close_window(&self) {
        // nwg::modal_info_message(&self.window, "close", "关闭窗口");
        nwg::stop_thread_dispatch();
    }
}

pub struct SettingAppUi {
    inner: Rc<SettingApp>,
    default_handler: RefCell<Option<nwg::EventHandler>>,
}

impl NativeUi<SettingAppUi> for SettingApp {
    fn build_ui(mut data: SettingApp) -> Result<SettingAppUi, nwg::NwgError> {
        use nwg::Event as E;

        // ui
        nwg::Window::builder()
            .flags(nwg::WindowFlags::WINDOW | nwg::WindowFlags::VISIBLE)
            .size((400, 200))
            .position((300, 300))
            .title("MiniEnv设置")
            .build(&mut data.window)?;

        nwg::Label::builder()
            .text("添加到右键菜单：")
            .h_align(nwg::HTextAlign::Right)
            .parent(&data.window)
            .build(&mut data.add_right_click_menu_label)?;

        nwg::Label::builder()
            .text("环境变量选择名称：")
            .h_align(nwg::HTextAlign::Right)
            .parent(&data.window)
            .build(&mut data.key_select_label)?;

        nwg::CheckBox::builder()
            .text("")
            .check_state(nwg::CheckBoxState::Checked)
            .parent(&data.window)
            .build(&mut data.add_right_click_menu_input)?;

        nwg::TextInput::builder()
            .parent(&data.window)
            .build(&mut data.key_select_input)?;

        nwg::Button::builder()
            .text("保存")
            .parent(&data.window)
            .build(&mut data.save_btn)?;

        // Wrap-up
        let ui = SettingAppUi {
            inner: Rc::new(data),
            default_handler: Default::default(),
        };

        // 事件
        let evt_ui = Rc::downgrade(&ui.inner);
        let handle_events = move |evt, _evt_data, handle| {
            if let Some(evt_ui) = evt_ui.upgrade() {
                match evt {
                    E::OnButtonClick => {
                        if &handle == &evt_ui.save_btn {
                            SettingApp::save(&evt_ui);
                        }
                    }
                    E::OnWindowClose => {
                        if &handle == &evt_ui.window {
                            SettingApp::close_window(&evt_ui);
                        }
                    }
                    _ => {}
                }
            }
        };

        *ui.default_handler.borrow_mut() = Some(nwg::full_bind_event_handler(
            &ui.window.handle,
            handle_events,
        ));

        // Layouts
        nwg::GridLayout::builder()
            .parent(&ui.window)
            .max_size([600, 300])
            .min_size([200, 100])
            .max_column(Some(2))
            .child(0, 0, &ui.add_right_click_menu_label)
            .child(0, 1, &ui.key_select_label)
            .child(1, 0, &ui.add_right_click_menu_input)
            .child(1, 1, &ui.key_select_input)
            .child(1, 2, &ui.save_btn)
            .build(&ui.layout)?;

        return Ok(ui);
    }
}

impl Drop for SettingAppUi {
    /// To make sure that everything is freed without issues, the default handler must be unbound.
    fn drop(&mut self) {
        let handler = self.default_handler.borrow();
        if handler.is_some() {
            nwg::unbind_event_handler(handler.as_ref().unwrap());
        }
    }
}

impl Deref for SettingAppUi {
    type Target = SettingApp;

    fn deref(&self) -> &SettingApp {
        &self.inner
    }
}
