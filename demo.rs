use std::{time::{Duration, self, SystemTime, UNIX_EPOCH}, process::exit};

fn main() {

    /* 开发测试用的程序 */

    use random::Source;
    let mut source = random::default(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64);
    let r = source.read::<i32>();
    println!("{}",r);
    if r % 2 == 0 {exit(1);}

    // std::thread::sleep(Duration::from_secs(1));
    // exit(1);

    
}

