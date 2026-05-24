//! Asset Verification Build Script

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    // Monitor shader directories for changes
    println!("cargo:rerun-if-changed=assets/shaders");
    println!("cargo:rerun-if-changed=assets/models");
    println!("cargo:rerun-if-changed=assets/textures");
    
    // Get output directory
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let out_path = PathBuf::from(out_dir);
    
    // Process shaders
    process_shaders(&out_path);
    
    // Copy asset dependencies to output directory
    copy_assets(&out_path);
    
    // Generate asset manifest
    generate_manifest(&out_path);
}

fn process_shaders(out_path: &Path) {
    let shader_dir = PathBuf::from("assets/shaders");
    
    if !shader_dir.exists() {
        log_info("Shader directory not found, skipping shader processing");
        return;
    }
    
    log_info("Processing shaders...");
    
    // Read all WGSL files
    if let Ok(entries) = fs::read_dir(&shader_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("wgsl") {
                if let Ok(source) = fs::read_to_string(&path) {
                    // Validate basic shader structure
                    if source.contains("fn ") {
                        log_info(&format!("Validated shader: {:?}", path.file_name().unwrap()));
                        
                        // Could add additional validation or compilation here
                        // For now, just verify the file is readable and has function definitions
                    }
                }
            }
        }
    }
}

fn copy_assets(out_path: &Path) {
    let assets_dir = PathBuf::from("assets");
    
    if !assets_dir.exists() {
        log_info("Assets directory not found, skipping asset copy");
        return;
    }
    
    // Create assets directory in output
    let out_assets = out_path.join("assets");
    if let Err(e) = fs::create_dir_all(&out_assets) {
        log_error(&format!("Failed to create output assets directory: {}", e));
        return;
    }
    
    log_info(&format!("Copying assets to {:?}", out_assets));
    
    // Copy configuration files
    let config_dir = assets_dir.join("config");
    if config_dir.exists() {
        let out_config = out_assets.join("config");
        if let Err(e) = fs::create_dir_all(&out_config) {
            log_error(&format!("Failed to create config directory: {}", e));
        } else if let Err(e) = copy_dir_recursive(&config_dir, &out_config) {
            log_error(&format!("Failed to copy config files: {}", e));
        }
    }
}

fn generate_manifest(out_path: &Path) {
    let manifest_path = out_path.join("asset_manifest.txt");
    
    log_info("Generating asset manifest...");
    
    let mut manifest_content = String::new();
    manifest_content.push_str("# Asset Manifest\n");
    manifest_content.push_str(&format!("Generated: {}\n", chrono_lite_timestamp()));
    manifest_content.push_str("\n");
    
    // List tracked asset directories
    manifest_content.push_str("Tracked directories:\n");
    manifest_content.push_str("  - assets/shaders\n");
    manifest_content.push_str("  - assets/models\n");
    manifest_content.push_str("  - assets/textures\n");
    manifest_content.push_str("  - assets/config\n");
    
    if let Err(e) = fs::write(&manifest_path, &manifest_content) {
        log_error(&format!("Failed to write manifest: {}", e));
    } else {
        log_info(&format!("Manifest written to {:?}", manifest_path));
    }
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }
    
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        
        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    
    Ok(())
}

fn log_info(msg: &str) {
    println!("cargo:warning=[BUILD INFO] {}", msg);
}

fn log_error(msg: &str) {
    println!("cargo:warning=[BUILD ERROR] {}", msg);
}

fn chrono_lite_timestamp() -> String {
    // Simple timestamp without external dependency
    "build-time".to_string()
}
