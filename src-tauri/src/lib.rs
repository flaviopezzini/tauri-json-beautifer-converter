use serde_json::Value;

fn parse_json(data: &str) -> Result<Value, String> {
    println!("Data is : {:?}", data);
    serde_json::from_str(data).map_err(|err| format!("Invalid JSON: {}", err))
}

#[tauri::command]
fn beautify(data: &str) -> Result<String, String> {
    parse_json(data).and_then(|parsed_json|
        serde_json::to_string_pretty(&parsed_json).map_err(|e| e.to_string())
    )
}

#[tauri::command]
fn minify(data: &str) -> Result<String, String> {
    parse_json(data).and_then(|parsed_json|
        serde_json::to_string(&parsed_json).map_err(|e| e.to_string())
    )
}

#[tauri::command]
fn json_to_yaml(data: &str) -> Result<String, String> {
    parse_json(data).and_then(|parsed_json|
        serde_yaml::to_string(&parsed_json).map_err(|e| e.to_string())
    )
}

#[tauri::command]
fn json_to_xml(data: &str) -> Result<String, String> {
    parse_json(data).and_then(|parsed_json|
        serde_xml_rs::to_string(&parsed_json).map_err(|e| e.to_string())
    )
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![beautify, minify, json_to_yaml, json_to_xml])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
