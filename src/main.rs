use std::env;
use std::thread;
use std::time::Duration;
use std::u64;
use windows::Win32::System::Power::{
    SetThreadExecutionState, ES_CONTINUOUS, ES_DISPLAY_REQUIRED, ES_SYSTEM_REQUIRED,
};

const OS: &str = "-os";
const ONLY_SLEEP: &str = "--only_sleep";
const WS: &str = "-ws";
const WITH_SCREEN: &str = "--with_screen";
const H: &str = "-h";
const HELP: &str = "--help";

fn print_help() {
    println!(
        "选项: 
{} 或 {}\t禁用睡眠
{} 或 {}\t同时禁用息屏与睡眠
{} 或 {}\t获取帮助信息",
        OS, ONLY_SLEEP, WS, WITH_SCREEN, H, HELP
    );
}

fn keep_thread_alive() {
    loop {
        thread::sleep(Duration::from_secs(u64::MAX));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        print_help();
        return;
    }
    let option = &args[1].to_lowercase();
    match option.as_str() {
        OS | ONLY_SLEEP => {
            unsafe {
                SetThreadExecutionState(ES_CONTINUOUS | ES_SYSTEM_REQUIRED);
            }
            println!("已禁用睡眠, 请保持该程序运行");
            keep_thread_alive();
        }
        WS | WITH_SCREEN => {
            unsafe {
                SetThreadExecutionState(ES_CONTINUOUS | ES_DISPLAY_REQUIRED | ES_SYSTEM_REQUIRED);
            }
            println!("已同时禁用息屏与睡眠, 请保持该程序运行");
            keep_thread_alive();
        }
        H | HELP => {
            print_help();
        }
        _ => {
            eprintln!("无效的选项: {}, 使用 --help 获取帮助信息", option);
        }
    }
}
