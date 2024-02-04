// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn save(steam_id: &str, save_path: &str) -> Result<String, String> {
    if steam_id == "" {
        return Err("steam id输入错误".into());
    };
    if save_path == "" {
        return Err("存档未上传".into());
    }

    println!("steam_id :{}", steam_id);
    // 尝试将字符串解析为 i64
    let parsed_number = match steam_id.parse::<i64>() {
        Ok(parsed_number) => parsed_number,
        Err(_) => return Err("请输入有效的数字格式的 Steam ID".into()),
    };
    let byte_data = parsed_number.to_le_bytes();
    // 使用 hex::encode 将字节数组转换为十六进制字符串
    // 打印字节数组的十六进制形式
    for &byte in &byte_data {
        print!("{:02X} ", byte);
    }
    // 打印转换后的字节数组
    println!("转换后的字节数组：{:?}", byte_data);
    let mut data = std::fs::read(save_path).unwrap();
    println!("{:?}", &data[4..12]);
    // 检查 data 的长度是否足够
    if data.len() < 12 {
        return Err("文件长度不足".into());
    }

    // 将 byte_data 的内容复制到 data 的指定位置
    data[4..12].copy_from_slice(&byte_data);

    // 打印修改后的数据片段
    println!("{:?}", &data[4..12]);

    // 将修改后的数据写回文件
    std::fs::write("new_save.dat", &data).map_err(|e| format!("写入文件失败：{}", e))?;
    Ok(format!("存档修改成功!"))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![save])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
