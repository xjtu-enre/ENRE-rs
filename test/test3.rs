// 实体总数：41
// 关系总数：82

// [实体] Module: crate::task (通过 use 导入)
// [关系] Use: 使用 crate::task 模块中的 Task
use crate::task::Task;
// [实体] Module: anyhow (通过 use 导入)
// [关系] Use: 使用 anyhow 模块中的 Context, Result
use anyhow::{Context, Result};
// [实体] Module: std::fs (通过 use 导入)
// [关系] Use: 使用 std::fs 模块
use std::fs::{self, File};
// [实体] Module: std::io (通过 use 导入)
// [关系] Use: 使用 std::io 模块中的 BufReader, BufWriter
use std::io::{BufReader, BufWriter};
// [实体] Module: std::path (通过 use 导入)
// [关系] Use: 使用 std::path 模块中的 Path
use std::path::Path;

// [实体] Struct: Storage
// [关系] Define: 定义结构体 Storage
// [关系] Contain: Storage 包含 file_path, tasks, next_id 字段
pub struct Storage {
    // [实体] Variable: file_path (字段)
    file_path: String,
    // [实体] Variable: tasks (字段)
    tasks: Vec<Task>,
    // [实体] Variable: next_id (字段)
    next_id: usize,
}

// [关系] Impl: 为 Storage 实现方法
impl Storage {
    // [实体] Function/Method: new
    // [关系] Define: 定义方法 new
    // [实体] Variable: file_path (参数)
    pub fn new(file_path: &str) -> Result<Self> {
        // [实体] Variable: tasks (局部变量)
        // [实体] Variable: next_id (局部变量)
        // [关系] Call: 调用 Self::load_tasks
        // [关系] UseVar: 使用变量 file_path
        let tasks = Self::load_tasks(file_path)?;
        // [关系] Call: 调用 tasks.iter(), map, task.id, max, unwrap_or
        // [关系] Modify: 修改 next_id
        // [关系] UseVar: 使用变量 tasks
        let next_id = tasks.iter().map(|task| task.id).max().unwrap_or(0) + 1;

        // [关系] Call: 调用 file_path.to_string()
        // [关系] UseVar: 使用变量 file_path, tasks, next_id
        Ok(Self {
            file_path: file_path.to_string(),
            tasks,
            next_id,
        })
    }

    // [实体] Function/Method: load_tasks
    // [关系] Define: 定义方法 load_tasks
    // [实体] Variable: file_path (参数)
    fn load_tasks(file_path: &str) -> Result<Vec<Task>> {
        // [实体] Variable: path (局部变量)
        // [关系] Call: 调用 Path::new
        // [关系] UseVar: 使用变量 file_path
        let path = Path::new(file_path);

        // [关系] Call: 调用 path.exists()
        // [关系] UseVar: 使用变量 path
        if !path.exists() {
            return Ok(Vec::new());
        }

        // [实体] Variable: file (局部变量)
        // [实体] Variable: reader (局部变量)
        // [实体] Variable: tasks (局部变量)
        // [关系] Call: 调用 File::open, BufReader::new, serde_json::from_reader
        // [关系] Call: 调用 with_context
        // [关系] UseVar: 使用变量 path, file_path
        let file = File::open(path).with_context(|| format!("无法打开文件: {}", file_path))?;
        let reader = BufReader::new(file);
        let tasks = serde_json::from_reader(reader)
            .with_context(|| format!("无法解析文件: {}", file_path))?;

        // [关系] UseVar: 使用变量 tasks
        Ok(tasks)
    }

    // [实体] Function/Method: save_tasks
    // [关系] Define: 定义方法 save_tasks
    fn save_tasks(&self) -> Result<()> {
        // 确保目录存在
        // [实体] Variable: parent (if let 变量)
        // [关系] Call: 调用 Path::new, parent, fs::create_dir_all
        // [关系] Call: 调用 with_context, parent.display()
        // [关系] UseVar: 使用变量 self.file_path
        if let Some(parent) = Path::new(&self.file_path).parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("无法创建目录: {}", parent.display()))?;
        }

        // [实体] Variable: file (局部变量)
        // [实体] Variable: writer (局部变量)
        // [关系] Call: 调用 File::create, BufWriter::new, serde_json::to_writer_pretty
        // [关系] Call: 调用 with_context
        // [关系] UseVar: 使用变量 self.file_path, self.tasks
        let file = File::create(&self.file_path)
            .with_context(|| format!("无法创建文件: {}", self.file_path))?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self.tasks)
            .with_context(|| format!("无法写入文件: {}", self.file_path))?;

        Ok(())
    }

    // [实体] Function/Method: add_task
    // [关系] Define: 定义方法 add_task
    // [实体] Variable: description (参数)
    // [实体] Variable: task (局部变量)
    pub fn add_task(&mut self, description: String) -> Result<&Task> {
        // [关系] Call: 调用 Task::new
        // [关系] UseVar: 使用变量 self.next_id, description
        let task = Task::new(self.next_id, description);
        // [关系] Modify: 修改 self.next_id
        // [关系] UseVar: 使用变量 self.next_id
        self.next_id += 1;
        // [关系] Call: 调用 self.tasks.push
        // [关系] Modify: 修改 self.tasks
        // [关系] UseVar: 使用变量 task, self.tasks
        self.tasks.push(task);
        // [关系] Call: 调用 self.save_tasks
        // [关系] UseVar: 使用变量 self
        self.save_tasks()?;

        // [关系] Call: 调用 self.tasks.last, unwrap
        // [关系] UseVar: 使用变量 self.tasks
        Ok(self.tasks.last().unwrap())
    }

    // [实体] Function/Method: list_tasks
    // [关系] Define: 定义方法 list_tasks
    // [实体] Variable: show_completed (参数)
    pub fn list_tasks(&self, show_completed: bool) -> Vec<&Task> {
        // [关系] Call: 调用 self.tasks.iter, filter, collect
        // [关系] UseVar: 使用变量 self.tasks, show_completed
        // [关系] UseVar: 使用闭包中的 task
        self.tasks
            .iter()
            .filter(|task| show_completed || !task.completed)
            .collect()
    }

    // [实体] Function/Method: complete_task
    // [关系] Define: 定义方法 complete_task
    // [实体] Variable: id (参数)
    // [实体] Variable: task (if let 变量)
    pub fn complete_task(&mut self, id: usize) -> Result<()> {
        // [关系] Call: 调用 self.tasks.iter_mut, find
        // [关系] Call: 调用 with_context
        // [关系] UseVar: 使用变量 self.tasks, id
        // [关系] UseVar: 使用闭包中的 task
        let task = self
            .tasks
            .iter_mut()
            .find(|task| task.id == id)
            .with_context(|| format!("未找到 ID 为 {} 的任务", id))?;

        // [关系] Call: 调用 task.toggle_completion
        // [关系] Modify: 修改 task
        // [关系] UseVar: 使用变量 task
        task.toggle_completion();
        // [关系] Call: 调用 self.save_tasks
        // [关系] UseVar: 使用变量 self
        self.save_tasks()?;

        Ok(())
    }

    // [实体] Function/Method: delete_task
    // [关系] Define: 定义方法 delete_task
    // [实体] Variable: id (参数)
    // [实体] Variable: position (局部变量)
    // [实体] Variable: task (局部变量)
    pub fn delete_task(&mut self, id: usize) -> Result<Task> {
        // [关系] Call: 调用 self.tasks.iter, position
        // [关系] Call: 调用 with_context
        // [关系] UseVar: 使用变量 self.tasks, id
        // [关系] UseVar: 使用闭包中的 task
        let position = self
            .tasks
            .iter()
            .position(|task| task.id == id)
            .with_context(|| format!("未找到 ID 为 {} 的任务", id))?;

        // [关系] Call: 调用 self.tasks.remove
        // [关系] Modify: 修改 self.tasks
        // [关系] UseVar: 使用变量 self.tasks, position
        let task = self.tasks.remove(position);
        // [关系] Call: 调用 self.save_tasks
        // [关系] UseVar: 使用变量 self
        self.save_tasks()?;

        // [关系] UseVar: 使用变量 task
        Ok(task)
    }
}
