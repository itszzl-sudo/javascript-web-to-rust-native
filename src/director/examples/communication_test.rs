
//! 通信方式测试示例
use jrust_runtime::comm::{CommMode, ThreadChannel, ProcessChannel, ProcessChannelServer, ThreadMessage, ProcessMessage};
use std::thread;
use std::time::Duration;

fn main() {
    println!("🚀 === 通信方式测试 === 🚀\n");

    // 1. 测试跨线程通信
    println!("--- 1. 跨线程通信测试 ---");
    test_thread_communication();
    println!();

    // 2. 测试跨进程通信（简化版，只启动服务器）
    println!("--- 2. 跨进程通信测试 ---");
    test_process_communication_simple();
    println!();

    println!("✅ 所有测试完成！");
}

fn test_thread_communication() {
    let (channel1, channel2) = ThreadChannel::new();

    // 线程1：发送消息
    let handle1 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));

        let msg = ThreadMessage::CallFunction {
            name: "sayHello".to_string(),
            args: vec!["JRust".to_string()],
        };

        channel1.send(msg).unwrap();
        println!("✅ Thread1: 发送消息成功");

        let response = channel1.recv().unwrap();
        println!("✅ Thread1: 收到响应: {:?}", response);
    });

    // 线程2：接收和回复
    let handle2 = thread::spawn(move || {
        let received = channel2.recv().unwrap();
        println!("✅ Thread2: 收到消息: {:?}", received);

        let response = ThreadMessage::Response(Ok("Hello from Thread2!".to_string()));
        channel2.send(response).unwrap();
        println!("✅ Thread2: 发送响应成功");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
    println!("✅ 跨线程通信测试成功！");
}

fn test_process_communication_simple() {
    println!("注意：完整跨进程测试需要启动两个进程");
    println!("正在尝试启动服务器...");

    let server = ProcessChannelServer::bind("127.0.0.1:8080");
    match server {
        Ok(srv) => {
            println!("✅ 服务器启动成功！地址: {}", srv.addr());
            println!("   （为了不阻塞，我们不等待连接）");
        }
        Err(e) => {
            println!("⚠️ 服务器启动失败: {}", e);
            println!("   （可能是端口被占用，这是正常的）");
        }
    }
}

