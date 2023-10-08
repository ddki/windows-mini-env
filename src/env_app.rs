extern crate native_windows_gui as nwg;
use std::{cell::RefCell, ops::Deref, rc::Rc};

use nwg::{CheckBoxState, NativeUi};
use winreg::{
    enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_ALL_ACCESS},
    RegKey,
};

#[derive(Default)]
pub struct EnvNewApp {
    pub operate: String,
    pub value: String,

    window: nwg::Window,
    layout: nwg::GridLayout,

    is_system_label: nwg::Label,
    key_label: nwg::Label,
    value_label: nwg::Label,

    is_system_input: nwg::CheckBox,
    key_input: nwg::TextInput,
    key_select: nwg::ComboBox<String>,
    value_input: nwg::TextInput,

    save_btn: nwg::Button,
}

impl EnvNewApp {
    fn save(&self) {
        let value = self.value_input.text();

        let is_system = self.is_system_input.check_state() == CheckBoxState::Checked;
        let is_new = self.operate == "new";
        if is_system {
            // 系统环境变量
            const SUB_HKEY: &str =
                "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment";
            // 打开注册表
            let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
            let cur_cer = hklm
                .open_subkey_with_flags(SUB_HKEY, KEY_ALL_ACCESS)
                .unwrap();
            if is_new {
                // 新建
                let key = self.key_input.text();
                let _ = cur_cer.set_value(key, &value);
            } else {
                // 修改
                let key_select = self.key_select.selection_string().unwrap();
                let update_key = key_select.clone();
                let old_value: String = cur_cer.get_value(key_select).unwrap();
                let new_value = format!("{};{}", old_value, value);
                let _ = cur_cer.set_value(update_key, &new_value);
            }
        } else {
            // 用户环境变量
            const SUB_HKEY: &str = "Environment";
            // 打开注册表
            let hklm = RegKey::predef(HKEY_CURRENT_USER);
            let cur_cer = hklm
                .open_subkey_with_flags(SUB_HKEY, KEY_ALL_ACCESS)
                .unwrap();
            if is_new {
                // 新建
                let key = self.key_input.text();
                let _ = cur_cer.set_value(key, &value);
            } else {
                // 修改
                let key_select = self.key_select.selection_string().unwrap();
                let update_key = key_select.clone();
                let old_value: String = cur_cer.get_value(key_select).unwrap();
                let new_value = format!("{};{}", old_value, value);
                let _ = cur_cer.set_value(update_key, &new_value);
            }
        }
        nwg::modal_info_message(&self.window, "提示", "保存成功");
    }
    fn close_window(&self) {
        nwg::stop_thread_dispatch();
    }
}

pub struct EnvNewAppUi {
    inner: Rc<EnvNewApp>,
    default_handler: RefCell<Option<nwg::EventHandler>>,
}

impl NativeUi<EnvNewAppUi> for EnvNewApp {
    fn build_ui(mut data: EnvNewApp) -> Result<EnvNewAppUi, nwg::NwgError> {
        use nwg::Event as E;

        // ui
        nwg::Window::builder()
            .flags(
                nwg::WindowFlags::WINDOW | nwg::WindowFlags::VISIBLE | nwg::WindowFlags::RESIZABLE,
            )
            .size((400, 200))
            .position((300, 300))
            .title("设置环境变量")
            .build(&mut data.window)?;

        nwg::Label::builder()
            .text("系统级：")
            .h_align(nwg::HTextAlign::Left)
            .parent(&data.window)
            .build(&mut data.is_system_label)?;

        nwg::Label::builder()
            .text("名称：")
            .h_align(nwg::HTextAlign::Left)
            .parent(&data.window)
            .build(&mut data.key_label)?;

        nwg::Label::builder()
            .text("值：")
            .h_align(nwg::HTextAlign::Left)
            .parent(&data.window)
            .build(&mut data.value_label)?;

        nwg::CheckBox::builder()
            .text("")
            .check_state(nwg::CheckBoxState::Checked)
            .parent(&data.window)
            .build(&mut data.is_system_input)?;

        let show_key_select = "modify" == data.operate;
        if show_key_select {
            let mut env_keys: Vec<String> = std::env::vars().map(|(key, _)| key).collect();
            env_keys.sort();
            // let mut env_keys: Vec<String> = Vec::new();
            // env_keys.push("PATH".to_string());
            nwg::ComboBox::builder()
                .collection(env_keys)
                .selected_index(Some(0))
                .parent(&data.window)
                .build(&mut data.key_select)?;
        } else {
            nwg::TextInput::builder()
                .parent(&data.window)
                .build(&mut data.key_input)?;
        }

        nwg::TextInput::builder()
            .text(&data.value)
            .parent(&data.window)
            .build(&mut data.value_input)?;

        nwg::Button::builder()
            .text("保存")
            .parent(&data.window)
            .build(&mut data.save_btn)?;

        // Wrap-up
        let ui = EnvNewAppUi {
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
                            EnvNewApp::save(&evt_ui);
                        }
                    }
                    E::OnWindowClose => {
                        if &handle == &evt_ui.window {
                            EnvNewApp::close_window(&evt_ui);
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
        if show_key_select {
            nwg::GridLayout::builder()
                .parent(&ui.window)
                .spacing(2)
                // label
                .child_item(nwg::GridLayoutItem::new(&ui.is_system_label, 0, 0, 1, 1))
                .child_item(nwg::GridLayoutItem::new(&ui.key_label, 0, 1, 1, 1))
                .child_item(nwg::GridLayoutItem::new(&ui.value_label, 0, 2, 1, 1))
                // input
                .child_item(nwg::GridLayoutItem::new(&ui.is_system_input, 1, 0, 3, 1))
                .child_item(nwg::GridLayoutItem::new(&ui.key_select, 1, 1, 3, 1))
                .child_item(nwg::GridLayoutItem::new(&ui.value_input, 1, 2, 3, 1))
                // button
                .child_item(nwg::GridLayoutItem::new(&ui.save_btn, 0, 3, 4, 1))
                .build(&ui.layout)?;
        } else {
            nwg::GridLayout::builder()
                .parent(&ui.window)
                // .spacing(2)
                // label
                .child(0, 0, &ui.is_system_label)
                .child(0, 1, &ui.key_label)
                .child(0, 2, &ui.value_label)
                // input
                .child_item(nwg::GridLayoutItem::new(&ui.is_system_input, 1, 0, 3, 1))
                .child_item(nwg::GridLayoutItem::new(&ui.key_input, 1, 1, 3, 1))
                .child_item(nwg::GridLayoutItem::new(&ui.value_input, 1, 2, 3, 1))
                // button
                .child_item(nwg::GridLayoutItem::new(&ui.save_btn, 0, 3, 4, 1))
                .build(&ui.layout)?;
        }

        return Ok(ui);
    }
}

impl Drop for EnvNewAppUi {
    /// To make sure that everything is freed without issues, the default handler must be unbound.
    fn drop(&mut self) {
        let handler = self.default_handler.borrow();
        if handler.is_some() {
            nwg::unbind_event_handler(handler.as_ref().unwrap());
        }
    }
}

impl Deref for EnvNewAppUi {
    type Target = EnvNewApp;

    fn deref(&self) -> &EnvNewApp {
        &self.inner
    }
}
