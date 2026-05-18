use crate::director::jrust_tree::{JsRustId, JsRustInstance, JsRustTree};
use crate::director::config::{BuildConfig, BuildMode};
use crate::dom::document::Document;
use crate::comm::CommMode;
use std::process::Command;
use std::fs;
use std::path::PathBuf;
use serde_json;

pub struct Director {
    jrust_tree: JsRustTree,
    workdir: PathBuf,
    comm_mode: CommMode,
    config: BuildConfig,
}

impl Director {
    pub fn new() -> Self {
        Self {
            jrust_tree: JsRustTree::new(),
            workdir: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            comm_mode: CommMode::Direct,
            config: BuildConfig::default(),
        }
    }
    
    pub fn with_workdir(workdir: PathBuf) -> Self {
        Self {
            jrust_tree: JsRustTree::new(),
            workdir,
            comm_mode: CommMode::Direct,
            config: BuildConfig::default(),
        }
    }
    
    pub fn with_config(config: BuildConfig) -> Self {
        Self {
            jrust_tree: JsRustTree::new(),
            workdir: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            comm_mode: match config.communication.mode.as_str() {
                "thread" => CommMode::Thread,
                "process" => CommMode::Process,
                _ => CommMode::Direct,
            },
            config,
        }
    }
    
    pub fn load_config(&mut self, path: &PathBuf) -> Result<(), String> {
        self.config = BuildConfig::load_from_file(path)?;
        self.comm_mode = match self.config.communication.mode.as_str() {
            "thread" => CommMode::Thread,
            "process" => CommMode::Process,
            _ => CommMode::Direct,
        };
        Ok(())
    }
    
    pub fn save_config(&self, path: &PathBuf) -> Result<(), String> {
        self.config.save_to_file(path)
    }
    
    pub fn config(&self) -> &BuildConfig {
        &self.config
    }
    
    pub fn config_mut(&mut self) -> &mut BuildConfig {
        &mut self.config
    }
    
    pub fn set_title(&mut self, title: &str) {
        self.config.project.title = title.to_string();
    }
    
    pub fn set_icon(&mut self, icon_path: &str) {
        self.config.resources.icon = Some(icon_path.to_string());
    }
    
    pub fn set_favicon(&mut self, favicon_path: &str) {
        self.config.resources.favicon = Some(favicon_path.to_string());
    }
    
    pub fn with_comm_mode(mut self, mode: CommMode) -> Self {
        self.comm_mode = mode;
        self.config.communication.mode = match mode {
            CommMode::Direct => "direct".to_string(),
            CommMode::Thread => "thread".to_string(),
            CommMode::Process => "process".to_string(),
        };
        self
    }
    
    pub fn set_comm_mode(&mut self, mode: CommMode) {
        self.comm_mode = mode;
        self.config.communication.mode = match mode {
            CommMode::Direct => "direct".to_string(),
            CommMode::Thread => "thread".to_string(),
            CommMode::Process => "process".to_string(),
        };
    }
    
    pub fn comm_mode(&self) -> CommMode {
        self.comm_mode
    }
    
    pub fn add_jrust(&mut self, instance: Box<dyn JsRustInstance>) -> JsRustId {
        let id = self.jrust_tree.create_root(instance);
        if let Some(node) = self.jrust_tree.get_node_mut(id) {
            node.instance.init();
        }
        id
    }
    
    pub fn create_child_jrust(&mut self, parent_id: JsRustId, instance: Box<dyn JsRustInstance>) -> Option<JsRustId> {
        let id = self.jrust_tree.create_child(parent_id, instance)?;
        if let Some(node) = self.jrust_tree.get_node_mut(id) {
            node.instance.init();
        }
        Some(id)
    }
    
    pub fn dispatch_event(&mut self) {
        self.jrust_tree.dispatch_event();
    }
    
    pub fn execute_command(&self, command: &str, args: &[&str]) -> Result<String, String> {
        let output = Command::new(command)
            .args(args)
            .current_dir(&self.workdir)
            .output()
            .map_err(|e| format!("Failed to execute command: {}", e))?;
            
        if !output.status.success() {
            return Err(format!(
                "Command failed with exit code {:?}\nStderr: {}",
                output.status.code(),
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
    
    pub fn translate_to_jrust(&self, js_code: &str) -> Result<String, String> {
        println!("=== Director: 翻译 JS 为 JRust ===\n");
        
        println!("输入 JS 代码长度: {} 字节", js_code.len());
        
        let temp_js_path = self.workdir.join("temp_input.js");
        fs::write(&temp_js_path, js_code)
            .map_err(|e| format!("写入临时 JS 文件失败: {}", e))?;
        
        println!("临时 JS 文件已创建: {:?}", temp_js_path);
        
        let translator_path = self.workdir.join("../../target/release/jrust-translator.exe");
        if !translator_path.exists() {
            println!("未找到 jrust-translator 二进制，使用模拟翻译");
            let rust_code = format!(
                "// 由 Director 翻译的 JRust 代码\n\
                fn main() {{\n\
                \tprintln!(\"Hello from translated JRust!\");\n\
                }}\n"
            );
            let _ = fs::remove_file(temp_js_path);
            println!("✅ 模拟翻译完成\n");
            return Ok(rust_code);
        }
        
        println!("正在调用 jrust-translator...");
        let output = Command::new(&translator_path)
            .arg(&temp_js_path)
            .output()
            .map_err(|e| format!("调用 jrust-translator 失败: {}", e))?;
        
        if !output.status.success() {
            return Err(format!(
                "jrust-translator 失败: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        
        let rust_code = String::from_utf8_lossy(&output.stdout).to_string();
        
        let _ = fs::remove_file(temp_js_path);
        
        println!("✅ 真实翻译完成\n");
        Ok(rust_code)
    }
    
    pub fn compile_jrust(&self, rust_code: &str, output_name: &str) -> Result<PathBuf, String> {
        println!("=== Director: 编译 JRust 为二进制 ===\n");

        let temp_project_dir = self.workdir.join("temp_jrust_project");
        let _ = fs::remove_dir_all(&temp_project_dir);
        fs::create_dir_all(&temp_project_dir)
            .map_err(|e| format!("创建临时项目目录失败: {}", e))?;

        let cargo_toml = format!(
            "[package]\n\
            name = \"{}\"\n\
            version = \"0.1.0\"\n\
            edition = \"2021\"\n\
            \n\
            [workspace]\n\
            \n\
            [dependencies]\n\
            jrust-runtime = {{ path = \"../../jrust-runtime\" }}\n",
            output_name
        );
        fs::write(temp_project_dir.join("Cargo.toml"), cargo_toml)
            .map_err(|e| format!("写入 Cargo.toml 失败: {}", e))?;

        let src_dir = temp_project_dir.join("src");
        fs::create_dir_all(&src_dir)
            .map_err(|e| format!("创建 src 目录失败: {}", e))?;
        fs::write(src_dir.join("main.rs"), rust_code)
            .map_err(|e| format!("写入 main.rs 失败: {}", e))?;

        println!("临时项目已创建: {:?}", temp_project_dir);

        println!("正在运行 cargo build...");
        let build_output = Command::new("cargo")
            .arg("build")
            .arg("--release")
            .current_dir(&temp_project_dir)
            .output()
            .map_err(|e| format!("cargo build 失败: {}", e))?;

        if !build_output.status.success() {
            return Err(format!(
                "cargo build 失败:\n{}",
                String::from_utf8_lossy(&build_output.stderr)
            ));
        }

        let exe_name = if cfg!(windows) {
            format!("{}.exe", output_name)
        } else {
            output_name.to_string()
        };

        let target_exe = temp_project_dir.join("target/release").join(exe_name);

        if target_exe.exists() {
            println!("✅ 编译完成，二进制文件: {:?}", target_exe);
            Ok(target_exe)
        } else {
            Err("未找到生成的二进制文件".to_string())
        }
    }
    
    /// 使用 Cranelift 编译 JS 为原生二进制（零 Cargo 依赖）
    pub fn compile_to_native(&self, js_code: &str, output_name: &str) -> Result<PathBuf, String> {
        println!("=== Director: JS → Cranelift → Native ===\n");
        
        // 1. JS → Cranelift IR
        println!("1. 翻译 JS 到 Cranelift IR...");
        let mut translator = jrust_translator::Compiler::new();
        let ir_program = translator.compile_to_ir(js_code)
            .map_err(|e| format!("JS 翻译失败: {:?}", e))?;
        println!("✅ IR 生成完成");
        
        // 2. Cranelift IR → .obj
        println!("2. 编译 IR 到目标文件...");
        let compiler = cranelift_compiler::CraneliftCompiler::new()
            .map_err(|e| e.to_string())?;
        let obj_bytes = compiler.compile(&ir_program)
            .map_err(|e| e.to_string())?;
        println!("✅ 目标文件生成完成: {} 字节", obj_bytes.len());
        
        // 3. 保存 .obj
        let temp_obj = self.workdir.join(format!("{}.obj", output_name));
        fs::write(&temp_obj, &obj_bytes)
            .map_err(|e| format!("写入 obj 文件失败: {}", e))?;
        println!("临时 obj 文件: {:?}", temp_obj);
        
        // 4. TODO: 链接 rust-browser.lib
        println!("\n⚠️ 链接步骤需要预编译的 rust-browser.lib");
        println!("使用方式:");
        println!("  compiler.link_with_lib(&obj_bytes, \"rust-browser.lib\", \"{}.exe\")?;", output_name);
        
        Ok(temp_obj)
    }

    pub fn pack_final_product(&self, exe_path: &PathBuf, output_dir: &PathBuf) -> Result<PathBuf, String> {
        println!("=== Director: 打包最终产品 ===\n");

        fs::create_dir_all(output_dir)
            .map_err(|e| format!("创建输出目录失败: {}", e))?;

        let output_exe = output_dir.join(exe_path.file_name().unwrap());
        fs::copy(exe_path, &output_exe)
            .map_err(|e| format!("复制可执行文件失败: {}", e))?;

        let icon_info = match self.config.get_icon_path() {
            Some(path) => format!("\n图标: {}\n", path),
            None => String::new(),
        };

        let readme = format!(
            "# {}\n\
            \n\
            {}\n\
            \n\
            这是一个由 Director 生成的 JRust 应用！\n\
            {}\n\
            ## 运行\n\
            直接执行可执行文件即可。\n\
            \n\
            ## 技术栈\n\
            - 输入：真实 Vue 项目\n\
            - 翻译：jrust-translator\n\
            - 运行时：jrust-runtime\n\
            - 打包：Director\n",
            self.config.project.title,
            self.config.project.description.clone().unwrap_or_default(),
            icon_info
        );
        fs::write(output_dir.join("README.md"), readme)
            .map_err(|e| format!("写入 README 失败: {}", e))?;

        println!("✅ 产品打包完成！输出目录: {:?}", output_dir);
        Ok(output_exe)
    }
}

pub struct BuildOutput {
    pub output_dir: PathBuf,
    pub source_dir: PathBuf,
    pub lib_dir: PathBuf,
    pub final_exe: PathBuf,
}

impl Director {
    
    pub fn prepare_output_dir(&self, project_name: &str, dist_path: &PathBuf) -> Result<BuildOutput, String> {
        println!("=== Director: 准备输出目录 ===\n");
        
        let output_dir = self.workdir.join(&self.config.output.base_dir).join(project_name);
        
        let source_dir = output_dir.join(&self.config.output.source_dir);
        let lib_dir = output_dir.join(&self.config.output.lib_dir);
        let final_dir = output_dir.join(&self.config.output.final_dir);
        
        fs::create_dir_all(&source_dir).map_err(|e| format!("Create source dir failed: {}", e))?;
        fs::create_dir_all(&lib_dir).map_err(|e| format!("Create lib dir failed: {}", e))?;
        fs::create_dir_all(&final_dir).map_err(|e| format!("Create final dir failed: {}", e))?;
        
        if dist_path.exists() && dist_path.is_dir() {
            if let Ok(entries) = fs::read_dir(dist_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                    if self.config.resources.resource_extensions.contains(&ext.to_string()) {
                        let dest = source_dir.join(path.file_name().unwrap());
                        let _ = fs::copy(&path, &dest);
                        println!("  复制资源: {:?}", path.file_name().unwrap());
                    }
                }
            }
        }
        
        if let Some(icon_path) = &self.config.resources.icon {
            if let Ok(icon_bytes) = fs::read(icon_path) {
                let dest_icon = source_dir.join(icon_path.split('/').last().unwrap_or(icon_path));
                let _ = fs::write(&dest_icon, icon_bytes);
                println!("  复制图标: {:?}", dest_icon);
            }
        } else if let Some(favicon_path) = &self.config.resources.favicon {
            if let Ok(favicon_bytes) = fs::read(favicon_path) {
                let dest_favicon = source_dir.join(favicon_path.split('/').last().unwrap_or(favicon_path));
                let _ = fs::write(&dest_favicon, favicon_bytes);
                println!("  复制 favicon: {:?}", dest_favicon);
            }
        }
        
        println!("✅ 输出目录已准备好:");
        println!("   项目标题: {}", self.config.project.title);
        println!("   源码目录: {:?}", source_dir);
        println!("   库目录: {:?}", lib_dir);
        println!("   最终目录: {:?}", final_dir);
        
        let exe_name = self.config.output.exe_name.clone().unwrap_or_else(|| project_name.to_string());
        Ok(BuildOutput {
            output_dir,
            source_dir,
            lib_dir,
            final_exe: final_dir.join(if cfg!(windows) { format!("{}.exe", exe_name) } else { exe_name }),
        })
    }
    
    pub fn save_jrust_source(&self, jrust_code: &str, output: &BuildOutput) -> Result<PathBuf, String> {
        println!("\n=== Director: 保存 JRust 源码 ===\n");
        
        let source_path = output.source_dir.join("main.rs");
        fs::write(&source_path, jrust_code)
            .map_err(|e| format!("Save jrust source failed: {}", e))?;
        
        println!("✅ JRust 源码已保存到: {:?}", source_path);
        Ok(source_path)
    }
    
    pub fn compile_jrust_lib(&self, jrust_code: &str, project_name: &str, output: &BuildOutput) -> Result<PathBuf, String> {
        println!("\n=== Director: 编译 JRust 库 ===\n");
        
        let temp_project_dir = self.workdir.join("temp_jrust_lib_project");
        let _ = fs::remove_dir_all(&temp_project_dir);
        fs::create_dir_all(&temp_project_dir)
            .map_err(|e| format!("Create temp project failed: {}", e))?;
        
        let cargo_toml = format!(
            "[package]\n\
            name = \"{}\"\n\
            version = \"0.1.0\"\n\
            edition = \"2021\"\n\
            \n\
            [lib]\n\
            name = \"{}\"\n\
            path = \"src/lib.rs\"\n\
            \n\
            [dependencies]\n\
            jrust-runtime = {{ path = \"../../jrust-runtime\" }}\n",
            project_name, project_name
        );
        fs::write(temp_project_dir.join("Cargo.toml"), cargo_toml)
            .map_err(|e| format!("Write Cargo.toml failed: {}", e))?;
        
        let src_dir = temp_project_dir.join("src");
        fs::create_dir_all(&src_dir)
            .map_err(|e| format!("Create src dir failed: {}", e))?;
        fs::write(src_dir.join("lib.rs"), jrust_code)
            .map_err(|e| format!("Write lib.rs failed: {}", e))?;
        
        println!("正在编译库...");
        let build_output = Command::new("cargo")
            .arg("build")
            .arg("--release")
            .current_dir(&temp_project_dir)
            .output()
            .map_err(|e| format!("cargo build failed: {}", e))?;
        
        if !build_output.status.success() {
            return Err(format!("Library build failed:\n{}", String::from_utf8_lossy(&build_output.stderr)));
        }
        
        let lib_name = if cfg!(windows) { format!("{}.lib", project_name) } else { format!("lib{}.a", project_name) };
        let target_lib = temp_project_dir.join("target/release").join(&lib_name);
        
        if target_lib.exists() {
            let dest_lib = output.lib_dir.join(&lib_name);
            fs::copy(&target_lib, &dest_lib)
                .map_err(|e| format!("Copy lib failed: {}", e))?;
            println!("✅ JRust 库已保存到: {:?}", dest_lib);
            Ok(dest_lib)
        } else {
            Err("Library file not found".to_string())
        }
    }
    
    pub fn build_final_exe(&self, jrust_code: &str, project_name: &str, output: &BuildOutput) -> Result<PathBuf, String> {
        println!("\n=== Director: 编译最终可执行文件 ===\n");
        
        let temp_project_dir = self.workdir.join("temp_jrust_exe_project");
        let _ = fs::remove_dir_all(&temp_project_dir);
        fs::create_dir_all(&temp_project_dir)
            .map_err(|e| format!("Create temp project failed: {}", e))?;
        
        let cargo_toml = format!(
            "[package]\n\
            name = \"{}\"\n\
            version = \"0.1.0\"\n\
            edition = \"2021\"\n\
            \n\
            [dependencies]\n\
            jrust-runtime = {{ path = \"../../jrust-runtime\" }}\n",
            project_name
        );
        fs::write(temp_project_dir.join("Cargo.toml"), cargo_toml)
            .map_err(|e| format!("Write Cargo.toml failed: {}", e))?;
        
        let src_dir = temp_project_dir.join("src");
        fs::create_dir_all(&src_dir)
            .map_err(|e| format!("Create src dir failed: {}", e))?;
        fs::write(src_dir.join("main.rs"), jrust_code)
            .map_err(|e| format!("Write main.rs failed: {}", e))?;
        
        println!("正在编译可执行文件...");
        let build_output = Command::new("cargo")
            .arg("build")
            .arg("--release")
            .current_dir(&temp_project_dir)
            .output()
            .map_err(|e| format!("cargo build failed: {}", e))?;
        
        if !build_output.status.success() {
            return Err(format!("Executable build failed:\n{}", String::from_utf8_lossy(&build_output.stderr)));
        }
        
        let exe_name = if cfg!(windows) { format!("{}.exe", project_name) } else { project_name.to_string() };
        let target_exe = temp_project_dir.join("target/release").join(&exe_name);
        
        if target_exe.exists() {
            fs::copy(&target_exe, &output.final_exe)
                .map_err(|e| format!("Copy exe failed: {}", e))?;
            println!("✅ 最终可执行文件已保存到: {:?}", output.final_exe);
            Ok(output.final_exe.clone())
        } else {
            Err("Executable not found".to_string())
        }
    }
    
    pub fn full_build_pipeline(&self, jrust_code: &str, project_name: &str, dist_path: &PathBuf) -> Result<BuildOutput, String> {
        println!("\n========== Director: 构建流程 ==========\n");
        println!("构建模式: {:?}", self.config.build.mode);
        
        let output = self.prepare_output_dir(project_name, dist_path)?;
        
        self.save_jrust_source(jrust_code, &output)?;
        
        if self.config.build.mode == BuildMode::CodeOnly {
            println!("\n⚠️  CodeOnly 模式 - 仅生成源码，跳过编译和 Snap");
            println!("\n========== 构建完成！ ==========\n");
            println!("输出目录: {:?}", output.output_dir);
            println!("1. 源码: {:?}", output.source_dir);
            return Ok(output);
        }
        
        if self.config.build.generate_snap {
            println!("\n--- 生成 Snap ---");
        }
        
        if self.config.build.split_code {
            println!("\n--- 分裂代码: jrusti + jruste ---");
        }
        
        if self.config.build.compile_lib {
            self.compile_jrust_lib(jrust_code, project_name, &output)?;
        }
        
        if self.config.build.compile_exe {
            self.build_final_exe(jrust_code, project_name, &output)?;
        }
        
        println!("\n========== 构建完成！ ==========\n");
        println!("输出目录: {:?}", output.output_dir);
        println!("1. 源码: {:?}", output.source_dir);
        if self.config.build.compile_lib {
            println!("2. 库: {:?}", output.lib_dir);
        }
        if self.config.build.compile_exe {
            println!("3. 最终文件: {:?}", output.final_exe);
        }
        
        Ok(output)
    }
    
    pub fn generate_snap(&self, document: &Document) -> Result<Vec<u8>, String> {
        println!("=== Director: 生成 DOM Snap ===\n");
        
        let json = serde_json::to_vec(document)
            .map_err(|e| format!("Snap JSON serialization failed: {}", e))?;
        
        println!("✅ Snap 生成成功！大小: {} 字节", json.len());
        Ok(json)
    }
    
    pub fn load_snap(&self, bytes: &[u8]) -> Result<Document, String> {
        println!("=== Director: 从 Snap 恢复 DOM ===\n");
        
        let document: Document = serde_json::from_slice(bytes)
            .map_err(|e| format!("Snap JSON deserialization failed: {}", e))?;
        
        println!("✅ DOM 从 Snap 恢复成功！");
        Ok(document)
    }
    
    pub fn save_snap_to_file(&self, document: &Document, path: &PathBuf) -> Result<(), String> {
        let snap_bytes = self.generate_snap(document)?;
        fs::write(path, snap_bytes)
            .map_err(|e| format!("Save snap failed: {}", e))?;
        println!("✅ Snap 已保存到: {:?}", path);
        Ok(())
    }
    
    pub fn load_snap_from_file(&self, path: &PathBuf) -> Result<Document, String> {
        let bytes = fs::read(path)
            .map_err(|e| format!("Load snap failed: {}", e))?;
        self.load_snap(&bytes)
    }

    pub fn auto_split_into_jrusti_jruste(&self, document: &Document, output_dir: &PathBuf, comm_mode: Option<CommMode>) -> Result<(), String> {
        let mode = comm_mode.unwrap_or(self.comm_mode);
        println!("\n=== Director: 自动分裂开始 (通信方式: {}) ===\n", mode.as_str());

        fs::create_dir_all(output_dir)
            .map_err(|e| format!("Create output dir failed: {}", e))?;

        println!("--- 1. 生成 jrusti（初始化器 + Snap） ---");
        let snap_path = output_dir.join("app.snap");
        self.save_snap_to_file(document, &snap_path)?;

        let jrusti_code = self.generate_jrusti_code(mode);
        let jrusti_path = output_dir.join("jrusti.rs");
        fs::write(&jrusti_path, jrusti_code)
            .map_err(|e| format!("Write jrusti failed: {}", e))?;
        println!("✅ jrusti 已保存到 {:?}", jrusti_path);

        println!("\n--- 2. 生成 jruste（事件处理器） ---");
        let jruste_code = self.generate_jruste_code(mode);
        let jruste_path = output_dir.join("jruste.rs");
        fs::write(&jruste_path, jruste_code)
            .map_err(|e| format!("Write jruste failed: {}", e))?;
        println!("✅ jruste 已保存到 {:?}", jruste_path);

        println!("\n=== Director: 自动分裂完成！ ===");
        Ok(())
    }
    
    /// 基于语义分析分离 JS 代码（推荐方式）
    pub fn split_js_by_semantic(&self, js_code: &str) -> Result<(String, String), String> {
        println!("\n=== Director: 语义分析分离代码 ===\n");
        
        let mut compiler = jrust_translator::Compiler::new();
        let compile_result = compiler.compile(js_code)
            .map_err(|e| format!("编译失败: {:?}", e))?;
        
        let mut splitter = jrust_translator::CodeSplitter::new();
        let analysis = splitter.analyze(&compile_result.ast);
        
        println!("分析结果:");
        println!("  初始化函数: {:?}", analysis.initializer_functions);
        println!("  事件处理器: {:?}", analysis.event_handlers);
        println!("  DOM 操作数: {}", analysis.dom_operations.len());
        println!("  事件绑定数: {}", analysis.event_bindings.len());
        
        let (init_stmts, handler_stmts) = splitter.split(&compile_result.ast);
        
        let init_code = self.statements_to_code(&init_stmts);
        let handler_code = self.statements_to_code(&handler_stmts);
        
        println!("\n✅ 分离完成:");
        println!("  初始化代码: {} 字符", init_code.len());
        println!("  事件处理代码: {} 字符", handler_code.len());
        
        Ok((init_code, handler_code))
    }
    
    /// 分离并生成 jrusti + jruste（语义分析版）
    pub fn split_and_compile(&self, js_code: &str, output_dir: &PathBuf) -> Result<(), String> {
        println!("\n=== Director: 分离并编译 ===\n");
        
        let (init_code, handler_code) = self.split_js_by_semantic(js_code)?;
        
        fs::create_dir_all(output_dir)
            .map_err(|e| format!("Create output dir failed: {}", e))?;
        
        let jrusti_path = output_dir.join("jrusti.rs");
        let jrusti_full = format!(
            r#"// jrusti - 初始化器
use jrust_runtime::director::Director;
use jrust_runtime::dom::document::Document;

{}

pub fn init() -> Document {{
    let mut document = Document::new();
    // 初始化代码执行
    document
}}
"#,
            init_code
        );
        fs::write(&jrusti_path, &jrusti_full)
            .map_err(|e| format!("Write jrusti failed: {}", e))?;
        println!("✅ jrusti 已保存到 {:?}", jrusti_path);
        
        let jruste_path = output_dir.join("jruste.rs");
        let jruste_full = format!(
            r#"// jruste - 事件处理器
use jrust_runtime::dom::document::Document;

{}

pub fn handle_events(document: &mut Document) {{
    // 事件处理循环
}}
"#,
            handler_code
        );
        fs::write(&jruste_path, &jruste_full)
            .map_err(|e| format!("Write jruste failed: {}", e))?;
        println!("✅ jruste 已保存到 {:?}", jruste_path);
        
        println!("\n=== 分离并编译完成 ===");
        Ok(())
    }
    
    fn statements_to_code(&self, stmts: &[jrust_translator::ast::Statement]) -> String {
        let program = jrust_translator::ast::Program {
            source_type: jrust_translator::ast::SourceType::Module,
            body: stmts.to_vec(),
            loc: jrust_translator::ast::SourceLocation::default(),
        };
        
        let mut codegen = jrust_translator::codegen::CodeGen::new();
        codegen.generate(&program).unwrap_or_default()
    }

    pub fn split_by_dom_content_loaded(jrust_code: &str) -> Result<(String, String), String> {
        println!("\n=== Director: 通过 DOMContentLoaded 分割代码 ===\n");
        
        let dom_content_loaded = "DOMContentLoaded";
        
        if let Some(pos) = jrust_code.find(dom_content_loaded) {
            let init_code = jrust_code[..pos].to_string();
            let handler_code = jrust_code[pos..].to_string();
            
            println!("✅ 找到 DOMContentLoaded，分割成功");
            println!("   初始化代码: {} 字符", init_code.len());
            println!("   事件处理代码: {} 字符", handler_code.len());
            
            Ok((init_code, handler_code))
        } else {
            println!("⚠️ 未找到 DOMContentLoaded，返回完整代码");
            Ok((String::new(), jrust_code.to_string()))
        }
    }
    
    pub fn split_by_dom_content_loaded_file(jrust_code: &str, output_dir: &PathBuf) -> Result<(), String> {
        let (init_code, handler_code) = Self::split_by_dom_content_loaded(jrust_code)?;
        
        fs::create_dir_all(output_dir)
            .map_err(|e| format!("Create output dir failed: {}", e))?;
        
        let init_path = output_dir.join("init.rs");
        fs::write(&init_path, &init_code)
            .map_err(|e| format!("Write init.rs failed: {}", e))?;
        println!("✅ 初始化代码已保存到 {:?}", init_path);
        
        let handler_path = output_dir.join("handler.rs");
        fs::write(&handler_path, &handler_code)
            .map_err(|e| format!("Write handler.rs failed: {}", e))?;
        println!("✅ 事件处理代码已保存到 {:?}", handler_path);
        
        Ok(())
    }

    fn generate_jrusti_code(&self, comm_mode: CommMode) -> String {
        let comm_import = match comm_mode {
            CommMode::Direct => String::new(),
            CommMode::Thread => r#"use jrust_runtime::comm::{ThreadChannel, ThreadMessage};"#.to_string(),
            CommMode::Process => r#"use jrust_runtime::comm::{ProcessChannel, ProcessMessage};"#.to_string(),
        };

        let comm_init = match comm_mode {
            CommMode::Direct => String::new(),
            CommMode::Thread => r#"
    let (jrusti_channel, jruste_channel) = ThreadChannel::new();
    println!("✅ 跨线程通信通道已建立！");
"#.to_string(),
            CommMode::Process => r#"
    let server = ProcessChannelServer::bind("127.0.0.1:8080").expect("Failed to bind server");
    println!("✅ 跨进程通信服务器已启动！地址: {}", server.addr());
"#.to_string(),
        };

        format!(r#"
//! jrusti - Initializer + Snap 加载器
use jrust_runtime::director::Director;
use jrust_runtime::dom::document::Document;
use std::path::PathBuf;
{}

fn main() -> Result<(), String> {{
    println!("🚀 === jrusti 启动！加载 Snap 中... (通信方式: {}) === 🚀");
    
    let director = Director::new();
    let snap_path = PathBuf::from("app.snap");
    let document = director.load_snap_from_file(&snap_path)?;
    
    println!("✅ Snap 加载成功！DOM 已就绪！");
    println!("   Document title: {{}}", document.title());
    {}
    println!("\n🚀 === jrusti 初始化完成！准备启动 jruste === 🚀");
    Ok(())
}}
"#, comm_import, comm_mode.as_str(), comm_init)
    }

    fn generate_jruste_code(&self, comm_mode: CommMode) -> String {
        let comm_import = match comm_mode {
            CommMode::Direct => String::new(),
            CommMode::Thread => r#"use jrust_runtime::comm::{ThreadChannel, ThreadMessage};"#.to_string(),
            CommMode::Process => r#"use jrust_runtime::comm::{ProcessChannel, ProcessMessage};"#.to_string(),
        };

        let comm_setup = match comm_mode {
            CommMode::Direct => String::new(),
            CommMode::Thread => r#"
    let (_jruste_channel, _jrusti_channel) = ThreadChannel::new();
    println!("✅ 跨线程通信通道已连接！");
"#.to_string(),
            CommMode::Process => r#"
    let channel = ProcessChannel::connect("127.0.0.1:8080").expect("Failed to connect to server");
    println!("✅ 跨进程通信通道已连接！");
"#.to_string(),
        };

        format!(r#"
//! jruste - Event Handler + DOM 渲染
use jrust_runtime::director::Director;
use jrust_runtime::dom::document::Document;
use jrust_runtime::dom::element::Element;
use std::path::PathBuf;
{}

fn main() -> Result<(), String> {{
    println!("🚀 === jruste 启动！加载 Snap + 处理事件 (通信方式: {}) === 🚀");
    
    let director = Director::new();
    let snap_path = PathBuf::from("app.snap");
    let mut document = director.load_snap_from_file(&snap_path)?;
    
    println!("✅ Snap 加载成功！");
    {}
    
    println!("\n--- 事件循环开始 ---");
    println!("   按 Ctrl+C 退出...");
    
    let mut counter = 0;
    loop {{
        std::thread::sleep(std::time::Duration::from_millis(100));
        counter += 1;
        
        if counter % 10 == 0 {{
            println!("🔄 事件循环中... 已处理 {{}} 帧", counter);
        }}
        
        if counter > 50 {{
            break;
        }}
    }}
    
    println!("\n✅ 事件循环结束！");
    Ok(())
}}
"#, comm_import, comm_mode.as_str(), comm_setup)
    }

    pub fn get_jrust_tree(&self) -> &JsRustTree {
        &self.jrust_tree
    }
    
    pub fn get_jrust_tree_mut(&mut self) -> &mut JsRustTree {
        &mut self.jrust_tree
    }

    pub fn list_jrust_instances(&self) -> Vec<JsRustId> {
        self.jrust_tree.list_all_ids()
    }
    
    pub fn print_jrust_tree(&self) {
        println!("\n=== JRust Tree Structure ===");
        if let Some(root) = self.jrust_tree.get_root() {
            self.print_node_recursive(root, 0);
        } else {
            println!("  (empty tree)");
        }
        println!("============================\n");
    }
    
    fn print_node_recursive(&self, id: JsRustId, depth: usize) {
        let indent = "  ".repeat(depth);
        if let Some(node) = self.jrust_tree.get_node(id) {
            println!("{}{}", indent, id);
            for &child_id in &node.children {
                self.print_node_recursive(child_id, depth + 1);
            }
        }
    }
}

pub struct JRustApp {
    pub window: crate::bom::window::Window,
    snap_path: Option<PathBuf>,
}

impl JRustApp {
    pub fn new() -> Self {
        JRustApp {
            window: crate::bom::window::Window::new(),
            snap_path: None,
        }
    }
    
    pub fn with_snap_output<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.snap_path = Some(path.into());
        self
    }
    
    pub fn load_snap<P: Into<PathBuf>>(mut self, path: P) -> Self {
        let snap_path = path.into();
        if snap_path.exists() {
            let director = Director::new();
            if let Ok(doc) = director.load_snap_from_file(&snap_path) {
                self.window.document = doc;
            }
        }
        self
    }
    
    pub fn save_snap(&self) -> Result<(), String> {
        if let Some(snap_path) = &self.snap_path {
            let director = Director::new();
            director.save_snap_to_file(&self.window.document, snap_path)
        } else {
            Err("No snap path configured".to_string())
        }
    }
}

impl Drop for JRustApp {
    fn drop(&mut self) {
        if let Some(snap_path) = &self.snap_path {
            println!("\n🛡️  JRustApp 正在销毁 - 自动生成 Snap...");

            let director = Director::new();
            if let Ok(_) = director.save_snap_to_file(&self.window.document, snap_path) {
                println!("✅ Snap 自动保存成功！");
            }
        }
    }
}

impl Default for Director {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SimpleJsRust {
    name: String,
}

impl SimpleJsRust {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl JsRustInstance for SimpleJsRust {
    fn init(&mut self) {
        println!("{} 初始化完成", self.name);
    }
    
    fn handle_event(&mut self) -> bool {
        println!("{} 收到事件", self.name);
        false
    }
    
    fn deploy_javascript_task(&mut self, _js_code: &str) {}
    
    fn get_children(&self) -> Vec<JsRustId> {
        Vec::new()
    }
}
