# ENRE-rs

ENRE-rs (Entity and Relation Extractor) 是一个用于分析Rust源代码的工具，能够提取代码中的实体（如模块、结构体、函数等）以及它们之间的关系（如定义、调用、使用等），并将结果以JSON格式输出。

## 功能介绍

ENRE-rs可以识别和提取以下类型的实体：

- 模块 (Module)
- 结构体 (Struct)
- 枚举 (Enum)
- 函数 (Function)
- 方法 (Method)
- 变量 (Variable)
- 常量 (Constant)
- Trait
- 实现 (Impl)
- 以及更多...

同时，它还能分析实体之间的多种关系：

- 定义关系 (Define)
- 调用关系 (Call)
- 使用关系 (Use/UseVar)
- 实现关系 (Impl/Implement)
- 继承关系 (Inherit)
- 修改关系 (Modify)
- 引用关系 (Reference)
- 以及更多...

## 用法

### 编译

```bash
cargo build 
```

### 运行

```bash
cargo run  <rust-source-file>
```

运行后，程序会生成一个`output.json`文件，其中包含分析结果。

### 示例

```bash
cargo run  test/test_mod.rs
```

这将分析`test/test_mod.rs`文件，并将结果保存在`output.json`中。

## 项目结构

```
.
├── src/                    # 源代码目录
│   ├── main.rs            # 程序入口点
│   ├── entity.rs          # 数据结构定义
│   ├── extractor.rs       # 核心提取逻辑
│   ├── visitor.rs         # AST遍历实现
│   └── hierarchy.rs       # Trait层次结构处理
├── docs/                   # 文档目录
│   ├── entity/            # 实体相关文档
│   └── relation/          # 关系相关文档
├── test/                  # 测试文件目录
├── Cargo.toml             # 项目配置文件
└── README.md              # 本文件
```

## 各部分功能介绍

### main.rs

程序的入口点，负责：
1. 解析命令行参数
2. 读取指定的Rust源文件
3. 驱动Extractor进行代码分析
4. 将分析结果序列化为JSON并保存到`output.json`

### entity.rs

定义了程序中使用的核心数据结构：
- `Entity`: 表示代码中的各种实体
- `Relation`: 表示实体之间的关系
- `Location`: 表示实体在源代码中的位置
- `Output`: 表示最终的输出结构

### extractor.rs

核心业务逻辑模块，包含`Extractor`结构体及其实现：
- 管理实体和关系的存储
- 提供添加实体和关系的方法
- 处理路径解析、作用域管理等核心功能
- 实现CHA (Class Hierarchy Analysis) 算法支持

### visitor.rs

基于`syn`库的Visitor模式实现，负责遍历Rust AST：
- 实现各种`visit_*`方法来处理不同类型的AST节点
- 在遍历过程中调用`extractor.rs`中的方法来提取实体和关系
- 处理各种Rust语言特性，如模块、函数、结构体、trait等

### hierarchy.rs

专门处理trait继承链与CHA (Class Hierarchy Analysis)：
- 管理trait之间的继承关系
- 管理trait与具体类型的实现关系
- 支持查找方法的目标实现

## 输出格式

程序会生成一个JSON文件，包含以下主要部分：

```json
{
  "entities": [...],    // 提取到的实体列表
  "relations": [...],   // 实体间的关系列表
  "meta": null          // 元数据（目前为空）
}
```

每个实体包含以下信息：
- `id`: 实体唯一标识符
- `name`: 实体名称
- `qualifiedName`: 实体的完全限定名
- `parent`: 父实体ID
- `category`: 实体类别（如"Function"、"Struct"等）
- `location`: 实体在源代码中的位置信息

每条关系包含以下信息：
- `from`: 起始实体ID
- `to`: 目标实体ID
- `category`: 关系类别（如"Define"、"Call"等）
### 测试

项目包含多个测试文件，用于验证不同功能的正确性：

```bash
# 运行特定测试
cargo run --release test/test_mod.rs
cargo run --release test/test_struct.rs
```

## 许可证

本项目采用[MIT许可证](LICENSE)。