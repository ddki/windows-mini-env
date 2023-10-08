extern crate native_windows_gui as nwg;
use std::{cell::RefCell, env, ops::Deref, rc::Rc};

use nwg::NativeUi;
use winreg::{
    enums::{HKEY_CLASSES_ROOT, KEY_ALL_ACCESS},
    RegKey,
};

#[derive(Default)]
struct MyCommand {
    name: String,
    title: String,
    command: String,
    icon: String,
}

impl MyCommand {
    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
    fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
    }
    fn set_command(&mut self, command: &str) {
        self.command = command.to_string();
    }
    fn set_icon(&mut self, icon: &str) {
        self.icon = icon.to_string();
    }

    fn get_name(&self) -> &str {
        return &self.name;
    }
    fn get_title(&self) -> &str {
        return &self.title;
    }
    fn get_command(&self) -> &str {
        return &self.command;
    }
    fn get_icon(&self) -> &str {
        return &self.icon;
    }
}

#[derive(Default)]
pub struct SettingApp {
    window: nwg::Window,
    layout: nwg::GridLayout,

    key_select_input: nwg::TextInput,

    save_reg_btn: nwg::Button,
    clear_reg_btn: nwg::Button,

    spilter_label: nwg::Label,
    env_name_input: nwg::TextInput,
    menu_title_input: nwg::TextInput,
    add_menu_btn: nwg::Button,
}

impl SettingApp {
    fn save_reg_btn(&self) {
        // 添加右键菜单
        const SUB_HKEY: &str = "Directory\\shell";
        // 打开注册表
        let hklm = RegKey::predef(HKEY_CLASSES_ROOT);
        let cur_cer = hklm
            .open_subkey_with_flags(SUB_HKEY, KEY_ALL_ACCESS)
            .unwrap();
        let app_key = "miniEnv";
        if cur_cer.create_subkey(app_key).is_ok() {
            let env_cer = hklm
                .open_subkey_with_flags(format!("{}\\{}", SUB_HKEY, app_key), KEY_ALL_ACCESS)
                .unwrap();

            // 获取程序路径
            let exe_path =
                env::current_exe().expect("Failed to retrieve the current executable path");
            let exe_path_str = exe_path.to_string_lossy();
            // 设置程序变量
            let _ = env_cer.set_value("select_keys", &self.key_select_input.text());
            // 设置名称
            let _ = env_cer.set_value("MUIVerb", &"Mini Env");
            // 设置图标
            let _ = env_cer.set_value("Icon", &exe_path_str.to_string());
            // 开启2级菜单
            let _ = env_cer.set_value("SubCommands", &"");

            // 2级菜单命令
            let mut sub_command1 = MyCommand::default();
            sub_command1.set_name("add_to_path");
            sub_command1.set_title("添加到PATH环境变量");
            sub_command1.set_command(&format!(
                "{} --mode cmd --operate modify --key PATH --value %1",
                exe_path_str
            ));
            sub_command1.set_icon(&exe_path_str.to_string());

            let mut sub_command2 = MyCommand::default();
            sub_command2.set_name("add_to_current_UI");
            sub_command2.set_title("添加到当前环境变量(ui)");
            sub_command2.set_command(&format!(
                "{} --mode ui --operate modify --value %1",
                exe_path_str
            ));
            sub_command2.set_icon(&exe_path_str.to_string());

            let mut sub_command3 = MyCommand::default();
            sub_command3.set_name("add_to_new_UI");
            sub_command3.set_title("添加到新的环境变量(ui)");
            sub_command3.set_command(&format!(
                "{} --mode ui --operate new --value %1",
                exe_path_str
            ));
            sub_command3.set_icon(&exe_path_str.to_string());

            for sub_shell in [sub_command1, sub_command2, sub_command3] {
                if env_cer
                    .create_subkey(format!("shell\\{}", sub_shell.get_name()))
                    .is_ok()
                {
                    let sub_shell_cer = hklm
                        .open_subkey_with_flags(
                            format!("{}\\{}\\shell\\{}", SUB_HKEY, app_key, sub_shell.get_name()),
                            KEY_ALL_ACCESS,
                        )
                        .unwrap();
                    // 设置名称
                    let _ = sub_shell_cer.set_value("MUIVerb", &sub_shell.get_title());
                    // 设置图标
                    let _ = sub_shell_cer.set_value("Icon", &sub_shell.get_icon());
                    if sub_shell_cer.create_subkey("command").is_ok() {
                        let sub_shell_command_cer = hklm
                            .open_subkey_with_flags(
                                format!(
                                    "{}\\{}\\shell\\{}\\command",
                                    SUB_HKEY,
                                    app_key,
                                    sub_shell.get_name()
                                ),
                                KEY_ALL_ACCESS,
                            )
                            .unwrap();
                        let _ = sub_shell_command_cer.set_value("", &sub_shell.get_command());
                    }
                }
            }
        }

        nwg::modal_info_message(&self.window, "提示", "注册完成");
    }
    fn clear_reg(&self) {
        const SUB_HKEY: &str = "Directory\\shell";
        // 打开注册表
        let hklm = RegKey::predef(HKEY_CLASSES_ROOT);
        let cur_cer = hklm
            .open_subkey_with_flags(SUB_HKEY, KEY_ALL_ACCESS)
            .unwrap();
        let app_key = "miniEnv";
        let _ = cur_cer.delete_subkey_all(app_key);
        nwg::modal_info_message(&self.window, "提示", "清理完成");
    }
    fn close_window(&self) {
        nwg::stop_thread_dispatch();
    }
    fn add_menu(&self) {
        if self.env_name_input.text().is_empty() {
            nwg::modal_error_message(&self.window, "错误", "请填写环境变量名称");
            return;
        };
        if self.menu_title_input.text().is_empty() {
            nwg::modal_error_message(&self.window, "错误", "请填写菜单名称");
            return;
        }
        let env_keys: Vec<String> = std::env::vars()
            .map(|(key, _)| key.to_uppercase())
            .collect();
        if !env_keys.contains(&self.env_name_input.text().to_uppercase()) {
            nwg::modal_error_message(
                &self.window,
                "错误",
                &format!("环境变量 {} 不存在", self.env_name_input.text()),
            );
            return;
        }
        // 添加右键菜单
        const SUB_HKEY: &str = "Directory\\shell";
        // 打开注册表
        let hklm = RegKey::predef(HKEY_CLASSES_ROOT);
        let app_key = "miniEnv";

        let env_cer = match hklm
            .open_subkey_with_flags(format!("{}\\{}", SUB_HKEY, app_key), KEY_ALL_ACCESS)
        {
            Ok(cer) => cer,
            Err(_e) => {
                nwg::modal_error_message(&self.window, "错误", "请先注册右键菜单");
                return;
            }
        };

        // 获取程序路径
        let exe_path = env::current_exe().expect("Failed to retrieve the current executable path");
        let exe_path_str = exe_path.to_string_lossy();

        // 2级菜单命令
        let mut sub_command = MyCommand::default();
        sub_command.set_name(&format!(
            "add_to_{}",
            self.env_name_input.text().to_lowercase()
        ));
        sub_command.set_title(&self.menu_title_input.text());
        sub_command.set_command(&format!(
            "{} --mode cmd --operate modify --key {} --value %1",
            exe_path_str,
            self.env_name_input.text()
        ));
        sub_command.set_icon(&exe_path_str.to_string());

        let sub_shell = sub_command;
        if env_cer
            .create_subkey(format!("shell\\{}", sub_shell.get_name()))
            .is_ok()
        {
            let sub_shell_cer = hklm
                .open_subkey_with_flags(
                    format!("{}\\{}\\shell\\{}", SUB_HKEY, app_key, sub_shell.get_name()),
                    KEY_ALL_ACCESS,
                )
                .unwrap();
            // 设置名称
            let _ = sub_shell_cer.set_value("MUIVerb", &sub_shell.get_title());
            // 设置图标
            let _ = sub_shell_cer.set_value("Icon", &sub_shell.get_icon());
            if sub_shell_cer.create_subkey("command").is_ok() {
                let sub_shell_command_cer = hklm
                    .open_subkey_with_flags(
                        format!(
                            "{}\\{}\\shell\\{}\\command",
                            SUB_HKEY,
                            app_key,
                            sub_shell.get_name()
                        ),
                        KEY_ALL_ACCESS,
                    )
                    .unwrap();
                let _ = sub_shell_command_cer.set_value("", &sub_shell.get_command());
            }
        }
        nwg::modal_info_message(
            &self.window,
            "提示",
            &format!("已将 {} 添加到右键菜单", self.menu_title_input.text()),
        );
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
            .flags(
                nwg::WindowFlags::WINDOW | nwg::WindowFlags::VISIBLE | nwg::WindowFlags::RESIZABLE,
            )
            .size((420, 300))
            .position((300, 300))
            .title("MiniEnv设置")
            .build(&mut data.window)?;

        nwg::TextInput::builder()
            .parent(&data.window)
            .placeholder_text(Some("环境变量下拉选择名称，多个使用半角逗号隔开"))
            .build(&mut data.key_select_input)?;

        nwg::Button::builder()
            .text("注册右键菜单")
            .parent(&data.window)
            .build(&mut data.save_reg_btn)?;

        nwg::Button::builder()
            .text("清理注册表")
            .parent(&data.window)
            .build(&mut data.clear_reg_btn)?;

        nwg::Label::builder()
            .text("")
            .h_align(nwg::HTextAlign::Center)
            .background_color(Some([252, 30, 30]))
            .parent(&data.window)
            .build(&mut data.spilter_label)?;

        nwg::TextInput::builder()
            .parent(&data.window)
            .placeholder_text(Some("环境变量名称"))
            .build(&mut data.env_name_input)?;

        nwg::TextInput::builder()
            .parent(&data.window)
            .placeholder_text(Some("右键菜单名称"))
            .build(&mut data.menu_title_input)?;

        nwg::Button::builder()
            .text("添加右键菜单")
            .parent(&data.window)
            .build(&mut data.add_menu_btn)?;

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
                        if &handle == &evt_ui.save_reg_btn {
                            SettingApp::save_reg_btn(&evt_ui);
                        }
                        if &handle == &evt_ui.clear_reg_btn {
                            SettingApp::clear_reg(&evt_ui);
                        }
                        if &handle == &evt_ui.add_menu_btn {
                            SettingApp::add_menu(&evt_ui);
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
            .spacing(2)
            .child(0, 0, &ui.key_select_input)
            .child(0, 1, &ui.save_reg_btn)
            .child(0, 2, &ui.clear_reg_btn)
            .child(0, 3, &ui.spilter_label)
            .child(0, 4, &ui.env_name_input)
            .child(0, 5, &ui.menu_title_input)
            .child(0, 6, &ui.add_menu_btn)
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
