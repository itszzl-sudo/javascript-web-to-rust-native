
//! jruste 完整示例：Snap 加载 + Servo 集成 + 事件循环

use jrust_servo::*;
use std::path::Path;
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    println!("🚀 === jruste 启动 === 🚀\n");
    
    // --- 步骤 1: 初始化 ---
    
    // 1.1 创建事件通道
    println!("--- 1. 创建事件通道 ---");
    let event_channel = EventChannel::new();
    println!("✅ 事件通道创建成功！\n");
    
    // 1.2 初始化 Servo
    println!("--- 2. 初始化 Servo ---");
    let servo_config = ServoConfig::minimal()
        .with_window_size(1280, 720)
        .with_window_title("jrust Demo App".to_string());
    
    let mut servo = ServoInstance::new(servo_config);
    servo.init()?;
    println!();
    
    // 1.3 加载 Snap
    println!("--- 3. 加载 Snap ---");
    let snap_path = Path::new("dist/split_output/app.snap");
    
    if snap_path.exists() {
        let mut injector = SnapInjector::new();
        injector.load_snap_from_file(snap_path)?;
        injector.inject_into_servo()?;
    } else {
        println!("⚠️ Snap 文件不存在，使用默认 DOM...");
        println!("提示：请先运行 split_demo 生成 Snap\n");
    }
    
    // --- 步骤 2: 启动事件循环 ---
    
    println!("\n--- 4. 启动事件循环 ---");
    println!("按 Ctrl+C 退出\n");
    
    // 模拟一些事件用于测试
    let event_channel_clone = event_channel.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(1000));
        println!("[模拟] 发送 Click 事件...");
        let _ = event_channel_clone.send_servo_event(ServoEvent::Click {
            target_id: Some("app".to_string()),
            x: 100,
            y: 100,
        });
        
        thread::sleep(Duration::from_millis(1000));
        println!("[模拟] 发送 KeyDown 事件...");
        let _ = event_channel_clone.send_servo_event(ServoEvent::KeyDown {
            key: "Enter".to_string(),
            key_code: 13,
        });
    });
    
    // 事件循环（简化版）
    let mut counter = 0;
    loop {
        // 尝试接收事件（非阻塞）
        match event_channel.servo_to_jruste_rx.try_recv() {
            Ok(event) => {
                println!("\n📩 收到事件: {:?}", event);
                println!("   处理中...");
                
                // 模拟处理
                thread::sleep(Duration::from_millis(100));
                
                // 模拟生成 DOM 更新
                let update = DomUpdate::set_text_content("status".to_string(), "Event handled!".to_string());
                let _ = event_channel.send_dom_update(update);
                
                println!("✅ 事件处理完成！");
            }
            Err(crossbeam_channel::TryRecvError::Empty) => {
                // 没有事件，继续等待
            }
            Err(crossbeam_channel::TryRecvError::Disconnected) => {
                println!("\n⚠️  事件通道已断开，退出...");
                break;
            }
        }
        
        // 尝试接收 DOM 更新
        match event_channel.jruste_to_servo_rx.try_recv() {
            Ok(update) => {
                println!("\n🔄 收到 DOM 更新: {:?}", update);
                println!("   应用到 Servo...");
            }
            Err(crossbeam_channel::TryRecvError::Empty) => {}
            Err(crossbeam_channel::TryRecvError::Disconnected) => {}
        }
        
        counter += 1;
        if counter % 50 == 0 {
            println!(".");
        }
        
        thread::sleep(Duration::from_millis(50));
        
        // 示例：运行一段时间后退出
        if counter > 100 {
            println!("\n⌛ 示例运行结束");
            break;
        }
    }
    
    println!("\n🎊 === jruste 退出 ===");
    Ok(())
}
