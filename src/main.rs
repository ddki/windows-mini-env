pub mod env_app;
pub mod env_utils;
pub mod setting_app;

extern crate native_windows_gui as nwg;
use clap::Parser;
use env_app::EnvNewApp;
use nwg::NativeUi;
use setting_app::SettingApp;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    mode: Option<String>,

    #[clap(short, long, value_parser)]
    operate: Option<String>,

    #[clap(short, long, value_parser)]
    key: Option<String>,

    #[clap(short, long, value_parser)]
    value: Option<String>,
}

fn main() {
    let args = Args::parse();
    if cfg!(debug_assertions) {
        println!("args = {:#?}", args);
    }

    match args.mode {
        Some(mode) => {
            // 参数值
            let env_operate = args.operate.unwrap_or("new".to_string());
            let env_key = args.key.unwrap_or_default();
            let env_value = args.value.unwrap();
            // 判断模式
            if mode == "cmd" {
                let _ = env_utils::set_env(&env_operate, true, &env_key, &env_value);
            } else if mode == "ui" {
                nwg::init().expect("Failed to init mini-env GUI");
                nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
                let mut env_app = EnvNewApp::default();
                env_app.operate = env_operate;
                env_app.value = env_value;
                let _ui = EnvNewApp::build_ui(env_app).expect("Failed to build UI");
                nwg::dispatch_thread_events();
            }
        }
        None => {
            nwg::init().expect("Failed to init mini-env GUI");
            nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
            let setting_app = SettingApp::default();
            let _ui = SettingApp::build_ui(setting_app).expect("Failed to build UI");
            nwg::dispatch_thread_events();
        }
    }
}
