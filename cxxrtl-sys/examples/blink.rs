use cxxrtl_sys::cxxrtl;
use std::path::{Path, PathBuf};
use std::ffi::CString;
use std::process::Command;
use std::env;

pub fn build(sources: &[PathBuf], dest: &Path) {
    let output = Command::new("yosys-config")
        .args(&["--datdir/include"])
        .output()
        .expect("failed to get yosys include dir");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let include = stdout.trim();

    let tmp = env::temp_dir().join("cxxrtl.cpp");

    let capi = Path::new(include).join("backends/cxxrtl/cxxrtl_capi.cc");

    Command::new("yosys")
        .args(&["-p", &format!("write_cxxrtl {}", tmp.to_string_lossy())])
        .args(sources)
        .status()
        .expect("failed generate cxxrtl code");
    
    Command::new("clang++")
        .args(&["-g", "-O3", "-fPIC", "-shared", "-std=c++14"])
        .arg(format!("-I{}", include))
        .arg(capi).arg(tmp).arg("-o").arg(dest)
        .status()
        .expect("failed generate cxxrtl code");
}

fn main() {
    let sources: Vec<PathBuf> = env::args().skip(1).map(PathBuf::from).collect();
    let lib = env::temp_dir().join("blink.so");
    build(sources.as_slice(), &lib);
    unsafe {
        let sim = cxxrtl::new(lib).expect("failed to load");
        let top = sim.cxxrtl_design_create();
        let blink = sim.cxxrtl_create(top);

        let clk_s = CString::new("clk").expect("CString::new failed");
        let clk = sim.cxxrtl_get(blink, clk_s.as_ptr());
        let led_s = CString::new("led").expect("CString::new failed");
        let led = sim.cxxrtl_get(blink, led_s.as_ptr());

        sim.cxxrtl_step(blink);
        let mut prev_led = 0;
        for cycle in 0..1000 {
            *(*clk).next = 0;
            sim.cxxrtl_step(blink);
            *(*clk).next = 1;
            sim.cxxrtl_step(blink);

            let curr_led = *(*led).curr;
            if prev_led != curr_led {
                println!("cycle {}, led {}", cycle, curr_led);
                prev_led = curr_led;
            }
        }
    }
} 