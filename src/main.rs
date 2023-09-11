pub mod env_app;
pub mod env_utils;
pub mod setting_app;

extern crate native_windows_gui as nwg;
use clap::Parser;
use env_app::EnvNewApp;
use nwg::NativeUi;
use setting_app::{SettingApp, SettingAppUi};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    mode: String,

    #[arg(short, long)]
    operate: String,

    #[arg(short, long)]
    key: String,

    #[arg(short, long)]
    value: String,
}

fn main() {
    let args = Args::parse();
    if args.mode == "cmd" {
        let _ = env_utils::set_env(&args.operate, true, &args.key, &args.value);
    } else if args.mode == "setting" {
        nwg::init().expect("Failed to init Native Windows GUI");
        nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
        let mut setting_app = SettingApp::default();
        let _ui = SettingAppUi::build_ui(setting_app).expect("Failed to build UI");
        nwg::dispatch_thread_events();
    } else {
        nwg::init().expect("Failed to init Native Windows GUI");
        nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
        let mut env_app = EnvNewApp::default();
        env_app.operate = "new".to_string();
        let _ui = EnvNewApp::build_ui(env_app).expect("Failed to build UI");
        nwg::dispatch_thread_events();
    }
}
