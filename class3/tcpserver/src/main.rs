use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

fn handle_client(mut stream: TcpStream) {
    //创建buf流
    let mut buf = [0u8; 1024];
    loop {
        //读取buf
        let r = stream.read(&mut buf);
        if let Ok(size) = r {
            let recv = &buf[0..size];
            //数据类型转换
            let recv_str = String::from_utf8_lossy(recv);
            //打印接收到的字符串
            println!("{}", recv_str);
            //写入数据流
            let res = stream.write_all(recv);
            //模式匹配，错误处理
            match res {
                Ok(_) => {}
                Err(_) => {
                    return;
                }
            }
            //结束处理
            if recv_str.starts_with("end") {
                println!("End tcp");
                return;
            }
        }
    }
}

fn main() {
    //启动监听
    let lis = TcpListener::bind("127.0.0.1:9999");
    match lis {
        Ok(listener) => {
            for sr in listener.incoming() {
                match sr {
                    Ok(stream) => {
                        // 起新线程处理，move表示移交所有权
                        thread::spawn(move || {
                            handle_client(stream);
                        });
                    }
                    Err(_) => {}
                }
            }
        }
        Err(e) => {
            println!("error->{}", e);
        }
    }
} 