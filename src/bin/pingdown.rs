//#![allow(dead_code)]
//#![allow(unused)]

use std::process::Command;
use std::process::Output;
use std::time::Duration;
use pingdown::*;
use std::thread;
use std::io;

///负责接收并输出参数，在Windows下调整终端输出以及作为循环的入口点(初始化)
fn main() {
    let cli = Cli::parse();
    #[cfg(windows)]
    cmd_to_utf8();
    
    if cli.vec_ip.is_empty()  {
        println!("Please input at least one ip or website.\nYou can also use -h or --help to get help.");
        sleep(7);
        std::process::exit(0);
    }
    println!("{:#?}", cli);
    
    println!("Started running...");
    normal_loop(&cli.vec_ip, &cli);
}

///普通循环，一般情况下永不返回
fn normal_loop(vec_ip: &Vec<String>, cli: &Cli) {
    let secs: u64 = match cli.secs_for_normal_loop.parse() {
        Ok(secs) => secs,//判断用户输入是否有误
        Err(_) => {
            println!("Please check your input.");
            error("turning input to a number[in function normal_loop]")
        }
    };//普通循环初始化到此为止
    println!("Started {}sec loop...", secs);
    for i in 1.. {//无限循环，同时计数
        let status = check_status(vec_ip, cli);//传入向量，进入数据处理阶段
        if status == false {//判断链接状态
            emergency_loop(vec_ip, cli);//紧急循环
            continue;//若重新连接则跳过本次等待
        }
        println!("Normal looped for {} times...", i);
        println!("{} secs left for the next normal loop...", secs);
        sleep(secs);
    }
}

///利用操作系统中的ping指令来判断单个地址的连接
fn get_status(ip: &str) -> bool {
    let command = format!("ping -c 1 {}", ip);
    let message = format!("Started clienting {}...", ip);
    let output = match run_command(&command, Some(&message)) {
        Ok(output) => output,//获取输出
        Err(_) => error("running command[in function get_status]"),
    };
    let status = String::from_utf8_lossy(&output.stdout).to_string();//从UTF-8转为普通字符串，lossy方法将不可用的字符统一处理为？
    println!("Started checking {}...", ip);
    if status.contains("TTL") || status.contains("ttl") {//通过检测命令输出中是否有TTL字样判断是否连接成功
         println!("fine.");
         true
    } else {
         println!("Request timed out.");
         false//整个程序中false表异常
    }
}

///将以向量存储的地址们解包处理，同时将多个返回结果统计为一个bool值
fn check_status(vec_ip: &Vec<String>, cli: &Cli) -> bool {
    let mut status_vec: Vec<bool> = vec![];
    for ip in vec_ip {
        let status = get_status(ip);
        status_vec.push(status);
    }
    let status = match cli.strict {
        false => {
            match status_vec.contains(&true) {//默认模式中，若任何地址通，则视为通过
                true => true,
                false => false,
            }
        },
        true => {
            match status_vec.contains(&false) {//严格模式中，若任意地址不通，则视为不通
                true => false,
                false => true,
            }
        },
    };
    status
}

///紧急循环，check_status函数判定为false时进入，判定为true时退出
fn emergency_loop(vec_ip: &Vec<String>, cli: &Cli) {
    let secs: u64 = match cli.secs_for_emergency_loop.parse() {
        Ok(secs) => secs,//判断参数是否正确
        Err(_) => {
            println!("Please check your input.");
            error("turning input to a usable number[in function emergency_loop]");
        }
    };
    let mut time_left: usize = match cli.times_for_emergency_loop.parse() {
        Ok(time_left) => time_left,
        Err(_) => {
            println!("Please check your input.");
            error("turning input to a usable number[in function emergency_loop]");
        }
    };
    println!("Warning!!! Connection lost!!!!");
    println!("Checking web connection per {} seconds!!", secs);
    loop {
        println!("{} tries remaining...", time_left);
        let status = check_status(vec_ip, cli);
        if status == true {
            break;
        } else if time_left <= 0 {
            shutdown();
            error("shutting down[permission denied]");//经过shutdown函数的指令轰炸(Unix)后，还没有关机的只能是没权限了
        }

        println!("{} secs left for the next loop...", secs);
        sleep(secs);
        time_left -= 1;
    }
    println!("Reconnected!!!");
    println!("Exiting {}sec emergency loop...", secs);
}

///适用于Windows的关机指令
#[cfg(windows)]
fn shutdown() {
    run_command("shutdown /s /t 0", Some("Started shutting down..."));
}

///适用于Unix的关机指令
#[cfg(unix)]
fn shutdown() {
    let _ = run_command("shutdown -h now", Some("Started shutting down..."));
    sleep(7);//等待关机，如过了7秒还没有关机的视为系统不支持该指令
    let _ = run_command("poweroff", None);
    sleep(7);
    let _ = run_command("poweroff -f", None);
    sleep(7);
    let _ = run_command("halt", None);
    sleep(7);
    let _ = run_command("init 0", None);
    sleep(7);
    let _ = run_command("systemctl poweroff", None);
}

///适用于Windows的命令行调用(cmd)
#[cfg(windows)]
fn run_command(command: &str, message: Option<&str>) -> io::Result<Output> {
    match message {
        Some(message) => println!("{}", message),
        None => {},
    }
    let output = Command::new("cmd").arg("/C").arg(command).output()?;
    Ok(output)
}

///适用于unix的命令行调用(sh)
#[cfg(unix)]
fn run_command(command: &str, message: Option<&str>) -> io::Result<Output> {
    match message {
        Some(message) => println!("{}", message),
        None => {},
    }
    let output = Command::new("sh").arg("-c").arg(command).output()?;
    Ok(output)
}

///在Windows中需要将终端输出格式设置为UTF-8以读取
#[cfg(windows)]
fn cmd_to_utf8() {
    ///65001是Windows中代表UTF-8输出的魔数
    let _ = match run_command("chcp 65001", None) {
        Ok(output) => output,
        Err(_) => error("turning cmd to UTF-8,[in function cmd_to_utf8]"),
    };
}

///错误处理函数
fn error(message: &str) -> ! {
    eprintln!("Sorry, an error occurred when {},please send an email to h-chris233@qq.com or open a issue to help me improve, thanks!", message);
    sleep(7);
    std::process::exit(1);
}

///暂停运行函数
fn sleep(time: u64) {
    thread::sleep(Duration::from_secs(time));
}
