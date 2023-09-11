use winreg::{
    enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_ALL_ACCESS},
    RegKey,
};

pub fn set_env(
    operate: &str,
    is_system: bool,
    key: &str,
    value: &str,
) -> Result<(), std::io::Error> {
    let is_new = operate == "new";
    if is_system {
        // 系统环境变量
        const SUB_HKEY: &str = "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment";
        // 打开注册表
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let cur_cer = hklm
            .open_subkey_with_flags(SUB_HKEY, KEY_ALL_ACCESS)
            .unwrap();
        if is_new {
            // 新建
            return cur_cer.set_value(key, &value);
        } else {
            // 修改
            let old_value: String = cur_cer.get_value(key).unwrap();
            let new_value = format!("{};{}", old_value, value);
            return cur_cer.set_value(key, &new_value);
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
            return cur_cer.set_value(key, &value);
        } else {
            // 修改
            let old_value: String = cur_cer.get_value(key).unwrap();
            let new_value = format!("{};{}", old_value, value);
            return cur_cer.set_value(key, &new_value);
        }
    }
}
