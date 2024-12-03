#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use std::{
    env,
    io::{BufRead, BufReader, Error, ErrorKind},
    process::{Command, Stdio},
};

use std::io::prelude::*;

use crate::args::Args;
mod args;


fn main() {

    let mut args: Vec<String> = env::args().collect();
    args.remove(0); // 第一个是程序自己，需要去除，不然就会反复启动自己，直接把cpu拉满
    if args.is_empty() {
        println!("until --max 10 --code 0");
        return;
    }

    let mut retry_args = Args::default();

    // 解析参数
    let mut iter = args.iter_mut().peekable();

    let mut use_arg: Vec<String> = Vec::new();

    loop {
        let next = iter.next();

        if next.is_none() {
            break;
        }

        let arg = next.unwrap();

        if arg == "--max" {
            if retry_args.max != -1 {
                // 已经有了max
                panic!("has max");
            }
            // 读取下一个参数作为 max
            let v = iter.next().expect("need number").parse::<i32>();
           retry_args.max = v.expect("--max 2 这样才行");
        } else if arg == "--code" {

            let v = iter.next().expect("need number").parse::<i32>();
            retry_args.code = v.expect("--code 0 这样才行");


        } else {
            use_arg.push(arg.to_string());
        }
    }


    let mut i = 1;

    #[cfg(target_os = "windows")]
    loop {
        let mut r = Command::new("cmd")
            // .creation_flags(0x08000000) // 有这个就没有输出了
            .arg("/C")
            .stderr(Stdio::inherit())
            .stdout(Stdio::inherit())
            .arg(use_arg.join(" "))
            .spawn()
            .unwrap();

        let s = r.wait().unwrap();
        if s.code().unwrap() == retry_args.code {
            break;
        }
        if retry_args.max != -1 && i > retry_args.max {
            break;
        }

        println!("-----第 {} 次重试-------", i);
        i = i + 1;
    }
    // {
    //     while !Command::new("cmd")
    //         .creation_flags(0x08000000)
    //         .arg("/C")
    //         .arg(use_arg.join(" "))
    //         .stdout(Stdio::piped())
    //         .stderr(Stdio::piped())
    //         .status()
    //         .expect("failed to execute process")
    //         .success()
    //     {
    //         println!("-----第 {} 次重试-------", i);
    //         i = i + 1;
    //     }
    // }

    #[cfg(not(target_os = "windows"))]
    loop {

        let mut r = Command::new("sh")
            .arg("-c")
            .arg(use_arg.join(" "))
            .spawn()
            .unwrap();

        let s = r.wait().unwrap();

        if s.code().unwrap() == retry_args.code {
            break;
        }
        if retry_args.max != -1 && i > retry_args.max {
            break;
        }
        println!("-----第 {} 次重试-------", i);
        i = i + 1;
    }

    // {
    //     while !Command::new("sh")
    //         .arg("-c")

    //         .arg(useArg.join(" "))
    //         .status()
    //         .expect("failed to execute process")
    //         .success()
    //     {

    //         println!("-----第 {} 次重试-------", i);
    //         i = i + 1;
    //     }
    // }
}

