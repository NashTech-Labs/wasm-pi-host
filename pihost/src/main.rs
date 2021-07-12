#[cfg(any(target_arch = "armv7", target_arch = "arm"))]
extern crate blinkt;

extern crate ctrlc;
extern crate notify;
extern crate wasmi;

use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::{channel, RecvTimeoutError, Sender};
use std::thread;
use std::time::Duration;
use wasm::Runtime;
use wasmi::RuntimeValue;

const MODULE_FILE: &'static str = "/home/knoldus/indicator.wasm";
const MODULE_DIR: &'static str = "/home/knoldus";

enum RunnerCommand {
    Reload,
    Stop,
}

/// watch function watch for the changes in the desired location.
///
/// #Arguments
///
/// tx_wasm - A Sender type object generic over RunnerCommand enum.
///
/// #Return
///
/// A Result type enum
fn watch(tx_wasm: Sender<RunnerCommand>) -> notify::Result<()> {
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(1))?;
    watcher.watch(MODULE_DIR, RecursiveMode::NonRecursive)?;

    loop {
        match rx.recv() {
            Ok(event) => handle_event(event, &tx_wasm),
            Err(recv_err) => println!("watch error: {:?}", recv_err),
        }
    }
}

/// handle_event function handles the event emitted by changes in desired location.
///
/// #Arguments
///
/// event - A DebouncedEvent enum denoting the event emitted.
/// tx_wasm - A reference to Sender type object generic over RunnerCommand enum.

fn handle_event(event: DebouncedEvent, tx_wasm: &Sender<RunnerCommand>) {
    match event {
        DebouncedEvent::NoticeWrite(path) => {
            let path = Path::new(&path);
            let filename = path.file_name().unwrap();
            if filename == "indicator.wasm" {
                tx_wasm.send(RunnerCommand::Reload).unwrap();
            } else {
                println!("write (unexpected file): {:?}", path);
            }
        }
        _ => {}
    }
}

// 20 frames per second == ms delay of 50 per iteration
// 10 fps == 100ms delay

fn main() {
    let (tx_wasm, rx_wasm) = channel();
    let _indicator_runner = thread::spawn(move || {
        let mut runtime = Runtime::new();
        let mut module = wasm::get_module_instance(MODULE_FILE);
        println!("Starting wasm runner thread...");
        loop {
            match rx_wasm.recv_timeout(Duration::from_millis(100)) {
                Ok(RunnerCommand::Reload) => {
                    println!("Received a reload signal, sleeping for 2s");
                    thread::sleep(Duration::from_secs(2));
                    module = wasm::get_module_instance(MODULE_FILE);
                }
                Ok(RunnerCommand::Stop) => {
                    runtime.shutdown();
                    break;
                }
                Err(RecvTimeoutError::Timeout) => {
                    runtime.reduce_battery();
                    runtime.advance_frame();
                    module
                        .invoke_export(
                            "sensor_update",
                            &[
                                RuntimeValue::from(wasm::SENSOR_BATTERY),
                                RuntimeValue::F64(runtime.remaining_battery.into()),
                            ][..],
                            &mut runtime,
                        )
                        .unwrap();

                    module
                        .invoke_export(
                            "apply",
                            &[RuntimeValue::from(runtime.frame)][..],
                            &mut runtime,
                        )
                        .unwrap();
                }
                Err(_) => break,
            }
        }
    });

    let tx_wasm_sig = tx_wasm.clone();

    ctrlc::set_handler(move || {
        // (6)
        tx_wasm_sig.send(RunnerCommand::Stop).unwrap();
    })
    .expect("Error setting Ctrl-C handler");

    if let Err(error) = watch(tx_wasm) {
        // (7)
        println!("error: {:?}", error)
    }
}

mod wasm;
