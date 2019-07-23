use android_glue::ffi;
use android_glue::looper::{ForeignLooper, Poll, ThreadLooper};
use android_glue::queue::AInputQueue;

use std::ffi::CString;
use std::panic::PanicInfo;

#[link(name = "android")]
extern "C" {}

fn panic_hook(info: &PanicInfo) {
    let c_str: *const u8 = CString::new(format!("panic: {}", info))
        .map(|s| s.as_ptr())
        .unwrap_or(b"Panic!!!\0" as *const u8);
    let tag: *const u8 = b"RustStdoutStderr\0" as *const u8;
    unsafe {
        ffi::__android_log_write(ffi::ANDROID_LOG_FATAL, tag, c_str);
    }
}

fn main() {
    println!("Main started");

    let android_app = android_glue::glue::get_android_app();
    unsafe {
        let looper = ThreadLooper::for_thread().unwrap();

        loop {
            match looper.poll_all().unwrap() {
                Poll::Event(ffi::LOOPER_ID_INPUT, fd, e, ptr) => {
                    let mut input_queue = AInputQueue::from_ptr((*android_app).inputQueue);
                    println!(
                        "Input event: {}, {}, {}, {:p}",
                        ffi::LOOPER_ID_INPUT,
                        fd,
                        e,
                        ptr
                    );
                    let event = input_queue.get_event().unwrap();
                    if let Some(event) = input_queue.pre_dispatch(event) {
                        println!("Event not pre dispatched.");
                        input_queue.finish_event(event, false);
                    }
                }
                Poll::Event(ffi::LOOPER_ID_MAIN, fd, e, ptr) => {
                    let cmd = ffi::android_app_read_cmd(android_app);
                    ffi::android_app_pre_exec_cmd(android_app, cmd);
                    println!(
                        "Main event: {}, {}, {}, {:p}",
                        ffi::LOOPER_ID_INPUT,
                        fd,
                        e,
                        ptr
                    );
                    println!("Command is {}", cmd);
                    ffi::android_app_post_exec_cmd(android_app, cmd);
                    if cmd as i32 == ffi::APP_CMD_DESTROY {
                        return;
                    }
                }
                Poll::Event(_, _, _, _) => unreachable!(),
                _ => continue,
            }
        }
    }
}
