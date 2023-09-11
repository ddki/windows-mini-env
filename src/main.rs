pub mod env_app;

extern crate native_windows_gui as nwg;
use env_app::{EnvNewApp, OperateEnum};
use nwg::NativeUi;

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let mut env_app = EnvNewApp::default();
    env_app.operate = OperateEnum::MODIFY;
    let _ui = EnvNewApp::build_ui(env_app).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
