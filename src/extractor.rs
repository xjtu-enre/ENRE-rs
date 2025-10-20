//// 业务核心：Extractor 结构体 + 除 visit_* 外的所有辅助方法
use proc_macro2::Span; //用于表示源代码中的位置信息
use std::collections::{HashMap, HashSet};
use syn::{spanned::Spanned, *};

use crate::entity::*;
use crate::hierarchy::TraitHierarchy;

pub struct Extractor {
    pub next_id: i32,             //用于生成实体ID的计数器
    pub entities: Vec<Entity>,    //存储所有实体的向量
    pub relations: Vec<Relation>, //：存储实体间关系的向量
    pub owner2name2id: HashMap<(i32, &'static str), HashMap<String, i32>>, //用于根据所有者和类别查找实体ID的哈希映射
    pub scope_stack: Vec<i32>,              //作用域栈，用于跟踪当前的作用域
    pub path2id: HashMap<String, i32>,      //将完整路径映射到实体ID的哈希映射
    pub glob_name2id: HashMap<String, i32>, //将名称映射到实体ID的全局哈希映射
    pub alias_map: HashMap<String, i32>,    //存储别名映射的哈希映射
    pub current_file_id: i32,               //当前处理的文件ID
    pub unresolved_calls: Vec<(i32, String, Span)>, //存储未解析的函数调用的向量
    pub trait_hierarchy: TraitHierarchy,    //TraitHierarchy实例，用于处理trait继承关系
    pub param_type_map: HashMap<i32, i32>,  //将参数ID映射到trait ID的哈希映射
    pub block_scope_counter: i32,           //块作用域计数器
    pub current_impl_type_id: Option<i32>,  //当前impl块的类型ID
}

impl Extractor {
    pub fn new(file_id: i32) -> Self {
        //创建一个新的Extractor实例，初始化所有字段
        let mut ext = Self {
            next_id: 1,
            entities: Vec::new(),
            relations: Vec::new(),
            owner2name2id: HashMap::new(),
            scope_stack: vec![0],
            path2id: HashMap::new(),
            glob_name2id: HashMap::new(),
            alias_map: HashMap::new(),
            current_file_id: file_id,
            unresolved_calls: Vec::new(),
            trait_hierarchy: TraitHierarchy::new(),
            param_type_map: HashMap::new(),
            block_scope_counter: 0,
            current_impl_type_id: None,
        };

        let crate_entity = Entity {
            id: 0,
            name: "crate".to_string(),
            qualified_name: "crate".to_string(),
            parent: None,
            category: "Crate",
            location: Some(Location {
                file: file_id,
                identifier: Range {
                    start: Position { line: 1, column: 1 },
                    end: Some(Position { line: 1, column: 6 }),
                },
                block: None,
            }),
        };
        ext.entities.push(crate_entity);
        ext
    }

    pub fn with_block_scope<F>(&mut self, node_span: Span, f: F)
    //用于在块作用域内执行操作
    where
        F: FnOnce(&mut Self),
    {
        let block_id = self.add(
            format!("block_{}", self.block_scope_counter),
            "Block", //为块创建一个新的实体，名称为"block_
            node_span,
        );
        self.block_scope_counter += 1; //增加块作用域计数器
        self.scope_stack.push(block_id); //将块实体ID推入作用域栈
        f(self); //执行传入的闭包函数f
        self.scope_stack.pop(); //从作用域栈中弹出块实体ID
    }

    pub fn span_to_range(&self, span: Span) -> Range {
        //将Span转换为Range
        let start = span.start(); //获取span的起始和结束位置
        let end = span.end();
        if start.line == 0 || end.line == 0 {
            return Range {
                start: Position { line: 0, column: 0 },
                end: None,
            };
        }
        Range {
            start: Position {
                line: start.line,
                column: start.column + 1,
            },
            end: Some(Position {
                line: end.line,
                column: end.column + 1,
            }),
        }
    }

    pub fn add(&mut self, name: String, cat: &'static str, span: Span) -> i32 {
        //用于添加新的实体
        let parent = *self.scope_stack.last().unwrap(); //获取当前作用域的父实体ID
        let parent_opt = if parent == -1 { None } else { Some(parent) };
        let key = (parent, cat);

        // 标准去重检查
        if let Some(map) = self.owner2name2id.get(&key) {
            //进行标准去重检查，如果已存在同名实体则直接返回其ID
            if let Some(&id) = map.get(&name) {
                return id;
            }
        }

        // 对于结构体，使用基于名字和位置的去重检查
        if cat == "Struct" {
            // 获取结构体的位置信息
            let range = self.span_to_range(span);

            // 遍历已有的结构体实体，检查是否有相同名字和位置的
            for entity in &self.entities {
                if entity.category == "Struct" && entity.name == name {
                    // 检查位置是否相同（基于行号）
                    if let Some(ref location) = entity.location {
                        if location.identifier.start.line == range.start.line {
                            // 同名且同行，认为是同一个结构体
                            return entity.id;
                        }
                    }
                }
            }
        }

        // 对于变量，添加基于行号的去重检查，防止重复创建
        if cat == "Variable" {
            let range = self.span_to_range(span);
            // 遍历已有的变量实体，检查是否有相同名字和位置的
            for entity in &self.entities {
                if entity.category == "Variable" && entity.name == name {
                    // 检查位置是否相同（基于行号）
                    if let Some(ref location) = entity.location {
                        if location.identifier.start.line == range.start.line {
                            // 同名且同行，认为是同一个变量
                            return entity.id;
                        }
                    }
                }
            }
        }

        // 原有的创建实体逻辑
        let path: Vec<_> = self
            .scope_stack
            .iter()
            .filter(|&&id| id > 0)
            .map(|&id| self.entities[id as usize].name.clone())
            .collect();

        let qualified = if path.is_empty() {
            if cat == "Block" {
                format!("crate::{}", name)
            } else {
                format!("crate::{}", name)
            }
        } else {
            if cat == "Block" {
                format!("crate::{}::{}", path.join("::"), name)
            } else {
                format!("crate::{}::{}", path.join("::"), name)
            }
        };

        let id = self.next_id;
        self.next_id += 1;

        let range = self.span_to_range(span); // 重新计算range

        let location = Location {
            file: self.current_file_id,
            identifier: range,
            block: None,
        };

        self.entities.push(Entity {
            id,
            name: name.clone(),
            qualified_name: qualified.clone(),
            parent: parent_opt,
            category: cat,
            location: Some(location),
        });

        self.owner2name2id
            .entry(key)
            .or_default()
            .insert(name.clone(), id);

        self.path2id.insert(qualified.clone(), id);
        self.glob_name2id.insert(name.clone(), id);
        self.add_rel(parent, id, "Define");

        id
    }

    pub fn add_rel(&mut self, from: i32, to: i32, rel: &'static str) {
        //用于添加实体间的关系
        if !self
            .relations
            .iter()
            .any(|r| r.from == from && r.to == to && r.category == rel)
        {
            self.relations.push(Relation {
                from,
                to,
                category: rel,
            });
        }
    }

    pub fn resolve_path(&self, path: &Path) -> Option<i32> {
        //用于解析路径并返回对应的实体ID
        let p = path //将路径的各个段连接成字符串
            .segments
            .iter()
            .map(|s| s.ident.to_string())
            .collect::<Vec<_>>()
            .join("::");
        let possibilities = vec![
            format!("crate::{}", p),
            p.clone(),
            path.segments
                .last()
                .map(|s| s.ident.to_string())
                .unwrap_or_default(),
        ];
        for path_str in possibilities {
            //在path2id和glob_name2id中查找匹配的实体ID
            if let Some(&id) = self.path2id.get(&path_str) {
                return Some(id);
            }
            if let Some(&id) = self.glob_name2id.get(&path_str) {
                return Some(id);
            }
        }
        None
    }

    pub fn record_var_use(&mut self, user: i32, name: &str) {
        let proper_user = if user >= 0 && (user as usize) < self.entities.len() {
            let category = self.entities[user as usize].category;
            if category == "Block" {
                self.get_proper_caller()
            } else {
                user
            }
        } else {
            user
        };
        for &scope in self.scope_stack.iter().rev() {
            let key = (scope, "Variable");
            if let Some(map) = self.owner2name2id.get(&key) {
                if let Some(&var_id) = map.get(name) {
                    self.add_rel(proper_user, var_id, "UseVar");
                    return;
                }
            }
        }
    }

    pub fn record_var_modify(&mut self, user: i32, name: &str) {
        //用于记录变量的修改
        let proper_user = if user >= 0 && (user as usize) < self.entities.len() {
            let category = self.entities[user as usize].category;
            if category == "Block" {
                self.get_proper_caller()
            } else {
                user
            }
        } else {
            user
        };
        for &scope in self.scope_stack.iter().rev() {
            let key = (scope, "Variable");
            if let Some(map) = self.owner2name2id.get(&key) {
                if let Some(&var_id) = map.get(name) {
                    self.add_rel(proper_user, var_id, "Modify");
                    return;
                }
            }
        }
    }

    pub fn record_var_relation(&mut self, user: i32, name: &str, rel_type: &'static str) {
        let proper_user = if user >= 0 && (user as usize) < self.entities.len() {
            let category = self.entities[user as usize].category;
            if category == "Block" {
                self.get_proper_caller()
            } else {
                user
            }
        } else {
            user
        };
        for &scope in self.scope_stack.iter().rev() {
            let key = (scope, "Variable");
            if let Some(map) = self.owner2name2id.get(&key) {
                if let Some(&var_id) = map.get(name) {
                    self.add_rel(proper_user, var_id, rel_type);
                    return;
                }
            }
        }
    }

    pub fn is_method(sig: &Signature) -> bool {
        //用于判断函数签名是否为方法（即是否有self参数）
        sig.inputs
            .iter()
            .any(|arg| matches!(arg, FnArg::Receiver(_)))
    }

    pub fn extract_lifetime(&mut self, lifetime: &Lifetime) -> i32 {
        //用于提取生命周期参数
        let name = lifetime.ident.to_string();
        let parent = *self.scope_stack.last().unwrap();
        let key = (parent, "Lifetime");
        if let Some(map) = self.owner2name2id.get(&key) {
            if let Some(&id) = map.get(&name) {
                return id;
            }
        }
        let id = self.next_id;
        self.next_id += 1;
        self.entities.push(Entity {
            id,
            name: name.clone(),
            qualified_name: format!("{}@{}", name, id),
            parent: Some(parent),
            category: "Lifetime",
            location: Some(Location {
                file: self.current_file_id,
                identifier: self.span_to_range(lifetime.span()),
                block: None,
            }),
        });
        self.owner2name2id.entry(key).or_default().insert(name, id);
        self.add_rel(parent, id, "Define");
        id
    }

    pub fn extract_params(&mut self, sig: &Signature) {
        //“把除 self 以外的每个 name: Type 参数抽成 "Parameter" 实体，挂到当前函数下面，并把 trait object 参数的类型信息记下来，为后续分析做准备。”
        //提取参数
        let parent = *self.scope_stack.last().unwrap(); //调用 extract_params 前，栈顶一定是刚刚压进去的那个 函数/方法实体 id。所有形参都会把它当父节点
        for input in &sig.inputs {
            match input {
                FnArg::Receiver(_) => {}
                FnArg::Typed(pat_type) => match &*pat_type.pat {
                    Pat::Ident(pat_ident) => {
                        //最常见的写法 x: T
                        let name = pat_ident.ident.to_string();
                        let param_id = self.add(name.clone(), "Parameter", pat_ident.ident.span()); //把参数名注册成实体，类别 "Parameter"，父节点是函数/方法
                        self.add_rel(parent, param_id, "Define");
                        if let Some(trait_id) = self.extract_trait_object_type(&pat_type.ty) {
                            self.param_type_map.insert(param_id, trait_id); //把“参数实体 id → trait 实体 id” 存进 param_type_map，供后续 CHA（类层次分析） 或 类型推断 使用。
                        }
                    }
                    Pat::Type(pat_type_inner) => {
                        //Pat::Type：带括号 (x): T 或更复杂写法时，再剥一层拿到内部 Pat::Ident
                        if let Pat::Ident(pat_ident) = &*pat_type_inner.pat {
                            let name = pat_ident.ident.to_string();
                            let param_id =
                                self.add(name.clone(), "Parameter", pat_ident.ident.span()); //把参数名注册成实体，类别 "Parameter"，父节点是函数/方法
                            self.add_rel(parent, param_id, "Define");
                            if let Some(trait_id) = self.extract_trait_object_type(&pat_type.ty) {
                                self.param_type_map.insert(param_id, trait_id); //把“参数实体 id → trait 实体 id” 存进 param_type_map，供后续 CHA（类层次分析） 或 类型推断 使用。
                            }
                        }
                    }
                    _ => {}
                },
            }
        }
    }

    pub fn extract_use_tree(
        //把一条 use 语句拆成“被引入的每一个符号实体”，并在“当前模块”与这些符号之间建立关系，同时去重。
        // 六种典型形态
        //use std::io::Write;           路径
        //use Vec;                      裸名
        //use std::fs as f;             重命名
        //use std::{io, fs::File};      花括号组
        //use utils::*;                 Glob形式
        &mut self,
        tree: &UseTree,           // 当前要解析的 use 子树
        user_mod: i32,            // 当前“使用”这条 use 的模块实体 id
        prefix: &mut Vec<String>, // 已累积的路径前缀（如 ["std", "io"]）
        seen: &mut HashSet<i32>,  // 已处理过的实体 id，防止重复建边
    ) {
        match tree {
            UseTree::Path(use_path) => {
                //递归拼前缀
                prefix.push(use_path.ident.to_string()); //看到 std::collections::HashMap 先拆成三段 ["std", "collections", "HashMap"]。
                self.extract_use_tree(&use_path.tree, user_mod, prefix, seen); //每次把当前段 push 进 prefix，再递归处理下一段
                prefix.pop(); //处理完后 弹出，保证 prefix 干净，不影响别的分支。
            }
            UseTree::Name(use_name) => {
                //裸名
                let mut full_path = prefix.join("::"); //// 1. 拼出完整路径
                if !full_path.is_empty() {
                    full_path.push_str("::");
                }
                full_path.push_str(&use_name.ident.to_string());
                let name_str = use_name.ident.to_string();
                let target_id = if let Some(&id) = self.path2id.get(&full_path) {
                    //按完整路径找实体
                    id
                } else if let Some(&id) = self.glob_name2id.get(&name_str) {
                    // 路径没找到，再按裸名兜底
                    id
                } else {
                    // 如果都找不到，创建一个占位符实体
                    let id = self.next_id;
                    self.next_id += 1;

                    // 创建占位符实体
                    let entity = Entity {
                        id,
                        name: name_str.clone(),
                        qualified_name: format!("crate::{}", name_str),
                        parent: Some(0),    // 以crate为父节点
                        category: "Import", // 使用"Import"类别表示这是一个导入的项
                        location: Some(Location {
                            file: self.current_file_id,
                            identifier: self.span_to_range(use_name.span()),
                            block: None,
                        }),
                    };

                    // 将实体添加到entities向量和glob_name2id映射中
                    self.entities.push(entity);
                    self.glob_name2id.insert(name_str.clone(), id);
                    self.path2id.insert(format!("crate::{}", name_str), id);
                    id
                };

                if seen.insert(target_id) {
                    self.add_rel(user_mod, target_id, "Use"); //建立一条 当前模块 → 目标实体 的关系，标签 "Use"
                }
            }
            UseTree::Rename(use_rename) => {
                let ident_str = use_rename.ident.to_string(); //// 原名
                let mut orig_path = prefix.join("::");
                if !orig_path.is_empty() {
                    orig_path.push_str("::"); // // 拼成完整路径
                }
                orig_path.push_str(&ident_str);
                let possible_paths = vec![
                    // 候选路径列表
                    format!("crate::{}", orig_path), // crate 绝对路径
                    orig_path.clone(),               // 相对路径
                    ident_str.clone(),               // 裸名
                ];
                let target_id = possible_paths // 找目标实体,只要任意一条命中，就拿到 target_id
                    .iter()
                    .find_map(|path| self.path2id.get(path))
                    .copied()
                    .or_else(|| self.glob_name2id.get(&ident_str).copied());
                if let Some(target_id) = target_id {
                    //给 Map 创建实体（类别 "ImportAlias"），记录位置。
                    let alias_id = self.add(
                        use_rename.rename.to_string(),
                        "ImportAlias",
                        use_rename.rename.span(),
                    );
                    self.alias_map
                        .insert(use_rename.rename.to_string(), target_id);
                    self.glob_name2id
                        .insert(use_rename.rename.to_string(), target_id);
                    self.add_rel(alias_id, target_id, "Aliases");
                    self.add_rel(user_mod, alias_id, "Define");
                    if seen.insert(target_id) {
                        self.add_rel(user_mod, target_id, "Use");
                    }
                }
            }
            UseTree::Group(group) => {
                for t in &group.items {
                    self.extract_use_tree(t, user_mod, prefix, seen);
                }
            }
            UseTree::Glob(_) => {
                // 处理 glob 形式 (use utils::*;)
                // 对于 glob 形式，我们需要引入模块中的所有公共项
                if !prefix.is_empty() {
                    let module_path = prefix.join("::");
                    // 收集需要处理的实体ID到临时向量中，避免借用冲突
                    let mut glob_items = Vec::new();

                    if let Some(&module_id) = self.path2id.get(&module_path) {
                        // 找到模块中的所有公共项并建立Use关系
                        // 遍历所有实体，找到父实体ID等于module_id的实体
                        for entity in &self.entities {
                            if let Some(parent_id) = entity.parent {
                                if parent_id == module_id {
                                    glob_items.push(entity.id);
                                }
                            }
                        }
                    } else if let Some(&module_id) = self.glob_name2id.get(prefix.last().unwrap()) {
                        // 如果按完整路径找不到，尝试按裸名查找模块
                        // 找到模块中的所有公共项并建立Use关系
                        // 遍历所有实体，找到父实体ID等于module_id的实体
                        for entity in &self.entities {
                            if let Some(parent_id) = entity.parent {
                                if parent_id == module_id {
                                    glob_items.push(entity.id);
                                }
                            }
                        }
                    }

                    // 处理收集到的实体ID
                    for item_id in glob_items {
                        if seen.insert(item_id) {
                            self.add_rel(user_mod, item_id, "Use");
                        }
                    }
                }
            }
            _ => {}
        }
    }

    pub fn get_proper_caller(&self) -> i32 {
        //获取合适的调用者实体ID
        for &scope_id in self.scope_stack.iter().rev() {
            if scope_id >= 0 && (scope_id as usize) < self.entities.len() {
                let category = self.entities[scope_id as usize].category;
                match category {
                    "Function" | "Method" | "Closure" | "AsyncBlock" => {
                        return scope_id;
                    }
                    _ => continue,
                }
            }
        }
        0
    }

    pub fn resolve_receiver_type(&self, expr: &Expr) -> Option<String> {
        //把方法调用左边的表达式 → 翻译成类型字符串 → 供后续方法查找分析
        match expr {
            Expr::Path(path) => {
                //表达式就是一个标识符，如 x 或 foo。
                if let Some(ident) = path.path.get_ident() {
                    //确保它确实是单个标识符（不是 a::b 这种路径）。
                    let name = ident.to_string(); //把标识符变成字符串，如 "x"。
                    for &scope in self.scope_stack.iter().rev() {
                        //从当前作用域向外逐层查找符号表。
                        if let Some(map) = self.owner2name2id.get(&(scope, "Variable")) {
                            //先看“变量”表
                            if let Some(&var_id) = map.get(&name) {
                                if let Some(&trait_id) = self.param_type_map.get(&var_id) {
                                    let trait_name = &self.entities[trait_id as usize].name;
                                    return Some(format!("dyn {}", trait_name)); //如果这个变量之前被标注或推断为 dyn Trait（存在 param_type_map），返回 "dyn TraitName"。
                                }
                                return Some(name); //否则直接返回变量名字字符串
                            }
                        }
                        if let Some(map) = self.owner2name2id.get(&(scope, "Parameter")) {
                            //再看“形参”表
                            if let Some(&param_id) = map.get(&name) {
                                if let Some(&trait_id) = self.param_type_map.get(&param_id) {
                                    let trait_name = &self.entities[trait_id as usize].name;
                                    return Some(format!("dyn {}", trait_name));
                                }
                                return Some(name);
                            }
                        }
                    }
                }
                None
            }
            Expr::Reference(ExprReference { expr, .. }) => self.resolve_receiver_type(expr), //遇到 &x、&mut x 等，剥掉引用继续推断内部表达式
            Expr::Field(field) => {
                //syn 里用来表示 “点号访问成员” 的表达式节点
                if let Some(base_type) = self.resolve_receiver_type(&*field.base) {
                    let member_str = match &field.member {
                        syn::Member::Named(ident) => ident.to_string(),
                        syn::Member::Unnamed(index) => index.index.to_string(),
                    };
                    return Some(format!("{}::{}", base_type, member_str));
                }
                None
            }
            Expr::Call(call) => {
                if let Expr::Path(ref path) = *call.func {
                    if path.path.segments.last().map(|s| s.ident.to_string()) == Some("Box".into())
                    {
                        if let Some(inner) = call.args.first() {
                            return self.resolve_receiver_type(inner);
                        }
                    }
                }
                None
            }
            Expr::MethodCall(call) => self.resolve_receiver_type(&call.receiver), //遇到 a.b().c()，只关心最左边的 a（call.receiver），继续递归。
            Expr::Group(group) => self.resolve_receiver_type(&group.expr), //遇到 (x)，剥掉括号继续递归。
            _ => None,
        }
    }

    pub fn find_method_target(&self, receiver_type: &str, method_name: &str) -> Vec<i32> {
        let mut targets = Vec::new();
        let trait_name = if receiver_type.starts_with("&dyn ") {
            &receiver_type[5..]
        } else if receiver_type.starts_with("dyn ") {
            &receiver_type[4..]
        } else {
            receiver_type
        };
        if let Some(&trait_id) = self.glob_name2id.get(trait_name) {
            let mut cha_targets = self
                .trait_hierarchy
                .cha_find_method_targets(trait_id, method_name);
            targets.append(&mut cha_targets);
        }
        targets
    }

    pub fn extract_trait_object_type(&self, ty: &Type) -> Option<i32> {
        //“给定一个 syn::Type，如果它代表一个 trait object（包括被 Box/Rc/Arc/& 包装的情况），就把对应的 trait 实体 id 找出来返回；否则返回 None。”
        match ty {
            Type::TraitObject(obj) => {
                for bound in &obj.bounds {
                    if let TypeParamBound::Trait(trait_bound) = bound {
                        eprintln!("Found trait bound: {:?}", trait_bound.path);
                        return self.resolve_path(&trait_bound.path);
                    }
                }
                None
            }
            Type::Path(path) => {
                if let Some(last_seg) = path.path.segments.last() {
                    let name = last_seg.ident.to_string();
                    if name == "Box" || name == "Arc" || name == "Rc" {
                        if let PathArguments::AngleBracketed(args) = &last_seg.arguments {
                            for arg in &args.args {
                                if let GenericArgument::Type(inner_ty) = arg {
                                    eprintln!("Found inner type in Box/Arc/Rc: {:?}", inner_ty);
                                    return self.extract_trait_object_type(inner_ty);
                                }
                            }
                        }
                    }
                }
                None
            }
            Type::Reference(TypeReference { elem, .. }) => {
                eprintln!("Found reference type, checking inner type: {:?}", elem);
                self.extract_trait_object_type(elem)
            }
            Type::Paren(TypeParen { elem, .. }) | Type::Group(TypeGroup { elem, .. }) => {
                eprintln!("Found paren/group type, checking inner type: {:?}", elem);
                self.extract_trait_object_type(elem)
            }
            _ => {
                eprintln!("Unsupported type in extract_trait_object_type: {:?}", ty);
                None
            }
        } //extract_trait_object_type 像剥洋葱一样，一层层剥掉 &、括号、Box/Rc/Arc，直到露出最里层的 dyn Trait，然后把 trait 名字映射成实体 id；只要中途任何一步不匹配，就返回 None。
    }

    pub fn resolve_unresolved_calls(&mut self) {
        //解析未解决的调用
        let mut to_resolve = Vec::new();
        let mut unresolved = Vec::new();
        for (caller, name, span) in self.unresolved_calls.drain(..) {
            //遍历未解决的调用列表
            if let Some(&callee_id) = self.glob_name2id.get(&name) {
                //尝试在全局名称映射中查找目标实体
                to_resolve.push((caller, callee_id));
            } else {
                unresolved.push((caller, name, span));
            }
        }
        for (caller, callee_id) in to_resolve {
            self.add_rel(caller, callee_id, "Call"); //如果找到，则添加"Call"关系
        }
        self.unresolved_calls = unresolved;
        for (_caller, name, span) in &self.unresolved_calls {
            eprintln!(
                "Warning: unresolved call to '{}' at line {}",
                name,
                span.start().line
            );
        }
    }

    pub fn infer_variable_type(&mut self, var_id: i32, expr: &Expr) {
        //推断变量类型
        match expr {
            Expr::Call(call_expr) => {
                //“函数调用”节点
                if let Expr::Path(func_path) = &*call_expr.func {
                    if let Some(last) = func_path.path.segments.last() {
                        let name = last.ident.to_string();
                        if name == "Box" || name == "Arc" || name == "Rc" {
                            if let PathArguments::AngleBracketed(args) = &last.arguments {
                                for arg in &args.args {
                                    if let GenericArgument::Type(ty) = arg {
                                        if let Some(trait_id) = self.extract_trait_object_type(ty) {
                                            self.param_type_map.insert(var_id, trait_id);
                                            return;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                if let Expr::Path(func_path) = &*call_expr.func {
                    if let Some(last_segment) = func_path.path.segments.last() {
                        if last_segment.ident == "new" && func_path.path.segments.len() >= 2 {
                            let type_segment =
                                &func_path.path.segments[func_path.path.segments.len() - 2];
                            let type_name = type_segment.ident.to_string();
                            if type_name == "Box" || type_name == "Arc" || type_name == "Rc" {
                                if let PathArguments::AngleBracketed(args) = &last_segment.arguments
                                {
                                    for arg in &args.args {
                                        if let GenericArgument::Type(inner_ty) = arg {
                                            if let Some(trait_id) =
                                                self.extract_trait_object_type(inner_ty)
                                            {
                                                self.param_type_map.insert(var_id, trait_id);
                                                return;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                if let Expr::Path(func_path) = &*call_expr.func {
                    if let Some(ident) = func_path.path.get_ident() {
                        let func_name = ident.to_string();
                        if let Some(_func_id) = self.glob_name2id.get(&func_name) {}
                    }
                }
            }
            Expr::Path(path_expr) => {
                if let Some(ident) = path_expr.path.get_ident() {
                    let source_var_name = ident.to_string();
                    self.copy_variable_type_info(&source_var_name, var_id);
                }
            }
            _ => {}
        }
    }

    pub fn copy_variable_type_info(&mut self, source_var_name: &str, target_var_id: i32) {
        //复制变量类型信息
        for &scope in self.scope_stack.iter().rev() {
            if let Some(map) = self.owner2name2id.get(&(scope, "Variable")) {
                if let Some(&source_var_id) = map.get(source_var_name) {
                    if let Some(&trait_id) = self.param_type_map.get(&source_var_id) {
                        self.param_type_map.insert(target_var_id, trait_id);
                    }
                    return;
                }
            }
        }
    }
    // 处理引用类型的初始化
    // 处理引用类型的初始化
    // 处理引用类型的初始化
    pub fn process_reference_initialization(&mut self, var_id: i32, init_expr: &Expr) {
        match init_expr {
            Expr::Reference(ref_expr) => {
                // 处理 &x 或 &mut x 形式的初始化
                if let Expr::Path(path_expr) = &*ref_expr.expr {
                    if let Some(ident) = path_expr.path.get_ident() {
                        let referenced_var_name = ident.to_string();
                        // 查找被引用的变量
                        if let Some(referenced_var_id) =
                            self.get_var_id_by_name(&referenced_var_name)
                        {
                            let rel_type = if ref_expr.mutability.is_some() {
                                "MutReference"
                            } else {
                                "ImmReference"
                            };
                            self.add_rel(var_id, referenced_var_id, rel_type);
                        }
                    }
                }
                // 处理 &arr[0] 形式的初始化
                else if let Expr::Index(index_expr) = &*ref_expr.expr {
                    if let Expr::Path(path_expr) = &*index_expr.expr {
                        if let Some(ident) = path_expr.path.get_ident() {
                            let referenced_var_name = ident.to_string();
                            // 查找被引用的数组变量
                            if let Some(referenced_var_id) =
                                self.get_var_id_by_name(&referenced_var_name)
                            {
                                let rel_type = if ref_expr.mutability.is_some() {
                                    "MutReference"
                                } else {
                                    "ImmReference"
                                };
                                self.add_rel(var_id, referenced_var_id, rel_type);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    // 辅助函数：根据名称查找变量ID
    // 辅助函数：根据名称查找变量ID
    pub fn get_var_id_by_name(&self, name: &str) -> Option<i32> {
        for &scope in self.scope_stack.iter().rev() {
            if let Some(map) = self.owner2name2id.get(&(scope, "Variable")) {
                if let Some(&id) = map.get(name) {
                    return Some(id);
                }
            }
        }
        None
    }
    pub fn process_assignment_left_operand(&mut self, left_expr: &Expr) {
        match left_expr {
            Expr::Path(path) => {
                // 简单变量赋值 x = value
                if let Some(ident) = path.path.get_ident() {
                    let mutator = self.get_proper_caller();
                    self.record_var_modify(mutator, &ident.to_string());
                }
            }
            Expr::Unary(unary_expr) => {
                // 解引用赋值 *ref = value
                if let UnOp::Deref(_) = unary_expr.op {
                    if let Expr::Path(path_expr) = &*unary_expr.expr {
                        if let Some(ident) = path_expr.path.get_ident() {
                            // 通过引用变量修改了它指向的变量
                            let mutator = self.get_proper_caller();
                            if let Some(ref_var_id) = self.get_var_id_by_name(&ident.to_string()) {
                                // 查找引用变量指向的变量
                                if let Some(target_var_id) = self.get_referenced_var_id(ref_var_id)
                                {
                                    self.add_rel(mutator, target_var_id, "Modify");
                                }
                            }
                        }
                    }
                }
            }
            Expr::Index(index_expr) => {
                // 数组/切片索引赋值 arr[i] = value
                // 这里可以添加对索引赋值的处理
                if let Expr::Path(path_expr) = &*index_expr.expr {
                    if let Some(ident) = path_expr.path.get_ident() {
                        let mutator = self.get_proper_caller();
                        self.record_var_modify(mutator, &ident.to_string());
                    }
                }
            }
            Expr::Field(field_expr) => {
                // 结构体字段赋值 obj.field = value
                if let Expr::Path(path_expr) = &*field_expr.base {
                    if let Some(ident) = path_expr.path.get_ident() {
                        let mutator = self.get_proper_caller();
                        self.record_var_modify(mutator, &ident.to_string());
                    }
                }
            }
            Expr::Tuple(tuple_expr) => {
                // 解构赋值 (x, y) = (1, 2)
                for elem in &tuple_expr.elems {
                    if let Expr::Path(path_expr) = elem {
                        if let Some(ident) = path_expr.path.get_ident() {
                            let mutator = self.get_proper_caller();
                            self.record_var_modify(mutator, &ident.to_string());
                        }
                    }
                }
            }
            _ => {
                // 其他类型的左操作数
            }
        }
    }
    // 在extractor.rs中添加
    pub fn infer_loop_variable_type(&mut self, var_id: i32, iterable_expr: &Expr) {
        // 根据迭代表达式的类型推断循环变量的类型
        match iterable_expr {
            Expr::Range(_) => {
                // 对于范围表达式，循环变量通常是i32
                // 这里可以添加更复杂的类型推断逻辑
            }
            Expr::Path(path_expr) => {
                // 对于路径表达式，需要解析变量并获取其类型
                if let Some(ident) = path_expr.path.get_ident() {
                    let var_name = ident.to_string();
                    // 查找变量并获取其类型
                    for &scope in self.scope_stack.iter().rev() {
                        if let Some(map) = self.owner2name2id.get(&(scope, "Variable")) {
                            if let Some(&source_var_id) = map.get(&var_name) {
                                if let Some(&trait_id) = self.param_type_map.get(&source_var_id) {
                                    self.param_type_map.insert(var_id, trait_id);
                                }
                                break;
                            }
                        }
                    }
                }
            }
            _ => {
                // 对于其他类型的表达式，暂时不做类型推断
            }
        }
    }

    // 辅助函数：查找引用变量指向的变量ID

    pub fn get_referenced_var_id(&self, ref_var_id: i32) -> Option<i32> {
        // 查找从ref_var_id出发的引用关系
        for relation in &self.relations {
            if relation.from == ref_var_id
                && (relation.category == "MutReference" || relation.category == "ImmReference")
            {
                return Some(relation.to);
            }
        }
        None
    }
}
