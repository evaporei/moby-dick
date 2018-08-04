#![feature(libc)]

use std::env;
use std::process::Command;
use std::process;
use std::fs;

extern crate libc;

fn main() {
    let args: Vec<String> = env::args().collect();
    process_args(&args);
}

fn process_args(args: &Vec<String>) {
    match args.get(1) {
        Some(command) => match command.as_ref() {
            "run" => run(&args),
            "child" => child(&args),
            _ => panic!("Not implemented :("),
        }
        None => panic!("No command provided"),
    };
}

fn run(args: &Vec<String>) {
    println!("Running");

    let proc_self_exe_args = &args[2..];

    Command::new("/proc/self/exe")
        .arg("child")
        .args(proc_self_exe_args)
        .status()
        .expect("Problem running /proc/self/exe");
}

fn child(args: &Vec<String>) {
    println!("Child Running {:?} on PID {}", &args[2..], process::id());

    let program_name = args.get(2);

    if let None = program_name {
        panic!("No program name provided");
    }
    let program_name = program_name.unwrap();

    let program_args = &args[3..];

    // unsafe {
    //     libc::mount(
    //         "rootfs".as_ptr() as *const i8,
    //         "rootfs".as_ptr() as *const i8,
    //         "".as_ptr() as *const i8,
    //         0x1000,
    //         "".as_ptr() as *const libc::c_void
    //     );
    //     libc::mkdir("rootfs/oldrootfs".as_ptr() as *const i8, 700);
    //     libc::syscall(libc::SYS_pivot_root, "rootfs".as_ptr(), "rootfs/oldrootfs".as_ptr());
    //     libc::chdir("/".as_ptr() as *const i8);
    // }

    Command::new("/bin/mkdir")
        .arg("-p")
        .arg("/home/rootfs")
        .status()
        .unwrap();

    Command::new("/bin/chroot")
        .arg("/home/rootfs")
        .status()
        .unwrap();

    Command::new("/bin/chdir")
        .arg("/home/rootfs")
        .status()
        .unwrap();

    unsafe {
        libc::chdir("/".as_ptr() as *const i8);
    }

    Command::new("/bin/mount")
        // .arg("/home/rootfs")
        // .arg("/")
        .arg("--source")
        .arg("proc")
        .arg("--target")
        .arg("proc")
        .arg("--types")
        .arg("proc")
        .status()
        .unwrap();

    //
    // fs::create_dir_all("/home/rootfs").unwrap();
    // unsafe {
    //     libc::chroot("/home/rootfs".as_ptr() as *const i8);
    //     libc::chdir("/".as_ptr() as *const i8);
    //     libc::mount(
    //         "proc".as_ptr() as *const i8,
    //         "proc".as_ptr() as *const i8,
    //         "proc".as_ptr() as *const i8,
    //         0,
    //         "".as_ptr() as *const libc::c_void
    //     );
    // }

    Command::new(program_name)
        .args(program_args)
        .current_dir("/")
        .status()
        .expect("Problem running program");
}
