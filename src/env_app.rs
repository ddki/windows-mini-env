extern crate native_windows_gui as nwg;
use std::{cell::RefCell, ops::Deref, ptr::eq, rc::Rc};

use nwg::NativeUi;

pub enum OperateEnum {
    NEW,
    MODIFY,
}

impl Default for OperateEnum {
    fn default() -> Self {
        OperateEnum::MODIFY
    }
}

#[derive(Default)]
pub struct EnvNewApp {
    operate: OperateEnum,

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
        print!("save...")
    }
    fn close_window(&self) {
        print!("close window...")
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
            .flags(nwg::WindowFlags::WINDOW | nwg::WindowFlags::VISIBLE)
            .size((300, 140))
            .position((300, 300))
            .title("设置环境变量")
            .build(&mut data.window)?;

        nwg::Label::builder()
            .text("系统环境变量：")
            .h_align(nwg::HTextAlign::Right)
            .parent(&data.window)
            .build(&mut data.is_system_label)?;

        nwg::Label::builder()
            .text("环境变量名称：")
            .h_align(nwg::HTextAlign::Right)
            .parent(&data.window)
            .build(&mut data.key_label)?;

        nwg::Label::builder()
            .text("环境变量值：")
            .h_align(nwg::HTextAlign::Right)
            .parent(&data.window)
            .build(&mut data.value_label)?;

        nwg::CheckBox::builder()
            .text("")
            .check_state(nwg::CheckBoxState::Checked)
            .parent(&data.window)
            .build(&mut data.is_system_input)?;

        if eq(&data.operate, &OperateEnum::MODIFY) {
            let env_keys: Vec<String> = std::env::vars().map(|(key, _)| key).collect();
            nwg::ComboBox::builder()
                .collection(env_keys)
                .selected_index(Some(0))
                .parent(&data.window)
                .build(&mut data.key_select)?;
        }

        nwg::TextInput::builder()
            .parent(&data.window)
            .build(&mut data.key_input)?;

        nwg::TextInput::builder()
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
                            EnvNewApp::save(&evt_ui)
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
        nwg::GridLayout::builder()
            .parent(&ui.window)
            .max_size([400, 200])
            .min_size([80, 60])
            .child(0, 0, &ui.is_system_label)
            .child(0, 1, &ui.key_label)
            .child(0, 2, &ui.value_label)
            .child(1, 0, &ui.is_system_input)
            .child(1, 1, &ui.key_input)
            .child(1, 2, &ui.value_input)
            .child(1, 3, &ui.save_btn)
            .build(&ui.layout)?;

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
