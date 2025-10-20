//// syn 的 Visitor 实现：所有 visit_* 函数集中地，负责 AST 遍历
use crate::entity::*;
use crate::extractor::Extractor;
use std::collections::HashSet;
use syn::spanned::Spanned;
use syn::{visit::Visit, *};
impl<'ast> Visit<'ast> for Extractor {
    fn visit_expr_method_call(&mut self, node: &ExprMethodCall) {
        //遇到一个方法调用表达式(x.foo(1, 2))
        let caller = self.get_proper_caller(); //找到“真正的调用者”实体 id
        let method = node.method.to_string(); //提取方法名（x.foo()就是foo)
        let receiver_type = self.resolve_receiver_type(&node.receiver); //尝试推断 接收者 的 类型字符串(x.foo()就是x的类型)
        if let Some(ty) = receiver_type {
            eprintln!(" receiver type: {}", ty);
            let trait_name = ty //把 dyn Trait 的各种写法（含 Box<dyn Trait>、&dyn Trait 等）统一抽成裸 trait 名。用裸名在 全局名字索引 里查找对应 trait 实体 id。
                .strip_prefix("Box<dyn ")
                .or_else(|| ty.strip_prefix("&dyn "))
                .or_else(|| ty.strip_prefix("dyn "))
                .unwrap_or(&ty)
                .trim_end_matches('>');
            eprintln!(" trait name: {}", trait_name);
            if let Some(&trait_id) = self.glob_name2id.get(trait_name) {
                eprintln!(" trait id: {}", trait_id);
                let targets = self
                    .trait_hierarchy
                    .cha_find_method_targets(trait_id, &method); //根据 trait 继承链和所有实现，找出 所有可能真正被调用的方法实体 id（包括默认实现、各 impl 的实现）。
                eprintln!(" CHA targets: {:?}", targets);
                for id in targets {
                    self.add_rel(caller, id, "Call"); //为每一个潜在目标建立 调用关系：当前函数/方法 → 被调用的方法，标签 "Call"。
                }
            } else {
                eprintln!(" trait NOT FOUND");
            }
        } else {
            eprintln!(" receiver type NOT RESOLVED");
        }
        syn::visit::visit_expr_method_call(self, node); //让 syn 的默认实现去“兜底”，确保 任何深层的节点 都不会被跳过。
    }

    fn visit_expr(&mut self, node: &'ast Expr) {
        //对任意表达式的总入口
        match node {
            Expr::MethodCall(call) => self.visit_expr_method_call(call), // 方法调用
            Expr::Call(call) => self.visit_expr_call(call),              //函数调用
            Expr::Struct(struct_expr) => self.visit_expr_struct(struct_expr), //结构体构造
            Expr::Path(path_expr) => self.visit_expr_path(path_expr),    //路径表达式
            Expr::Assign(assign_expr) => self.visit_expr_assign(assign_expr), //简单赋值
            Expr::Binary(binary_expr) => self.visit_expr_binary(binary_expr), //二元操作
            Expr::Unary(unary_expr) => self.visit_expr_unary(unary_expr), //一元操作
            Expr::ForLoop(for_loop) => self.visit_expr_for_loop(for_loop), //for循环
            _ => syn::visit::visit_expr(self, node),
        }
    }

    fn visit_expr_for_loop(&mut self, node: &ExprForLoop) {
        // 处理for循环中的迭代变量模式
        let mut loop_var_ids = Vec::new();

        // 定义一个递归函数来处理模式
        fn process_pattern(pat: &Pat, visitor: &mut Extractor, var_ids: &mut Vec<i32>) {
            match pat {
                Pat::Ident(pat_ident) => {
                    // 简单标识符模式，如 `i` in 0..10
                    let name = pat_ident.ident.to_string();
                    let var_id = visitor.add(name.clone(), "Variable", pat_ident.ident.span());
                    visitor.add_rel(*visitor.scope_stack.last().unwrap(), var_id, "Contain");
                    var_ids.push(var_id);
                }
                Pat::Tuple(tuple_pat) => {
                    // 元组模式，如 `(a, b)` in vec![(1, 2), (3, 4)]
                    for elem_pat in &tuple_pat.elems {
                        process_pattern(elem_pat, visitor, var_ids);
                    }
                }
                Pat::Slice(slice_pat) => {
                    // 切片模式，如 `[a, b]` in vec![vec![1, 2], vec![3, 4]]
                    for elem_pat in &slice_pat.elems {
                        process_pattern(elem_pat, visitor, var_ids);
                    }
                }
                Pat::Reference(ref_pat) => {
                    // 引用模式，如 `&x` in &vec
                    process_pattern(&ref_pat.pat, visitor, var_ids);
                }
                Pat::Paren(paren_pat) => {
                    // 括号模式
                    process_pattern(&paren_pat.pat, visitor, var_ids);
                }
                Pat::Type(pat_type) => {
                    // 类型模式，如 `x: i32` in 0..10
                    process_pattern(&pat_type.pat, visitor, var_ids);
                }
                Pat::Wild(_) => {
                    // 通配符模式 `_`，不创建变量实体
                }
                _ => {
                    // 对于其他未处理的模式类型，不创建变量实体
                }
            }
        }

        // 处理for循环的模式
        process_pattern(&node.pat, self, &mut loop_var_ids);

        // 为所有创建的循环变量推断类型
        // 这里可以添加更复杂的类型推断逻辑
        // 暂时为所有循环变量标记为i32类型（简化处理）
        for &var_id in &loop_var_ids {
            // 可以在这里添加更精确的类型推断
        }

        // 遍历迭代表达式
        self.visit_expr(&node.expr);

        // 遍历循环体
        self.visit_block(&node.body);

        syn::visit::visit_expr_for_loop(self, node);
    }

    fn visit_item_mod(&mut self, node: &ItemMod) {
        //遇到 模块定义（mod foo { … }）时触发。
        let id = self.add(node.ident.to_string(), "Module", node.ident.span());
        self.scope_stack.push(id); //把新模块实体压入作用域栈，后续所有子项（函数、类型、子模块等）都将以这个模块为父实体。
        syn::visit::visit_item_mod(self, node); //让 syn 的默认遍历器 继续递归处理模块内部的所有内容（子模块、函数、类型、use 等），确保不遗漏任何节点。
        self.scope_stack.pop(); //模块处理完毕，弹出作用域栈，回到上一层作用域
    }

    fn visit_item_use(&mut self, node: &ItemUse) {
        //遇到任何 use 语句（如 use std::collections::HashMap;）时被调用
        let user_mod = *self.scope_stack.last().unwrap(); //当前作用域栈顶的实体 id 就是 正在声明 use 的模块（“使用者”）。
        let mut prefix = Vec::new(); //准备一个空向量，用来在递归过程中累积 路径前缀
        let mut seen = HashSet::new(); //用于去重：同一模块里多次 use 同一个目标时，避免重复建立关系
        self.extract_use_tree(&node.tree, user_mod, &mut prefix, &mut seen); //把 use 语句的 use-tree 递归拆开，逐层处理
        syn::visit::visit_item_use(self, node); //兜底：让 syn 的默认实现继续递归处理 use 语句内部可能存在的其他子节点（属性、宏等），确保不遗漏。
    }

    fn visit_item_struct(&mut self, node: &ItemStruct) {
        let id = self.add(node.ident.to_string(), "Struct", node.ident.span()); //把 struct 的名字（比如 Person）注册成实体，类别 "Struct"，返回全局 id。

        self.scope_stack.push(id); // 把结构体压栈
        for param in &node.generics.params {
            //如果结构体带生命周期（struct Foo<'a>），就把 'a 注册成 "Lifetime" 实体，并和结构体建立 "Define" 关系。
            if let GenericParam::Lifetime(lt) = param {
                let lt_id = self.extract_lifetime(&lt.lifetime);
                self.add_rel(id, lt_id, "Define");
            }
        }
        match &node.fields {
            Fields::Named(fields) => {
                //每个字段名（如 x）注册成 "Variable" 实体。
                for f in &fields.named {
                    if let Some(ident) = &f.ident {
                        let f_id = self.add(ident.to_string(), "Variable", ident.span());
                        self.add_rel(id, f_id, "Define");
                        if let Some(trait_id) = self.extract_trait_object_type(&f.ty) {
                            self.param_type_map.insert(f_id, trait_id); //如果字段类型是 dyn Trait 或 Box<dyn Trait>，把字段 id 与 trait id 记录到 param_type_map，供后续动态分派用。
                        }
                    }
                }
            }
            Fields::Unnamed(fields) => {
                for (i, _) in fields.unnamed.iter().enumerate() {
                    let field_name = format!("_{}", i); //没有名字，就按索引起名字 "_0"、"_1"，同样注册成 "Variable"。
                    let f_id = self.add(field_name.clone(), "Variable", node.ident.span());
                    self.add_rel(id, f_id, "Define");
                }
            }
            Fields::Unit => {}
        }
        syn::visit::visit_item_struct(self, node); //让 syn 继续遍历结构体内部还没手动处理的子节点
        self.scope_stack.pop(); //结构体处理完毕，把它的 id 从栈里弹出，回到上一层作用域
    }

    fn visit_item_enum(&mut self, node: &ItemEnum) {
        let id = self.add(node.ident.to_string(), "Enum", node.ident.span()); //把枚举名字（如 Color）注册成实体，类别 "Enum"，拿到全局 id
        self.scope_stack.push(id); //以后所有变体、字段都会以这个枚举为父实体。
        for param in &node.generics.params {
            //如果枚举带生命周期（enum Foo<'a> { … }），把 'a 注册成 "Lifetime" 实体，并和枚举建立 "Define" 关系。
            if let GenericParam::Lifetime(lt) = param {
                let lt_id = self.extract_lifetime(&lt.lifetime);
                self.add_rel(id, lt_id, "Define");
            }
        }
        for var in &node.variants {
            //枚举里的每个变体
            let v_id = self.add(var.ident.to_string(), "Variant", var.ident.span()); //把变体名字注册为 "Variant" 实体。建立 枚举 → 变体 的关系 "Define"。
            self.add_rel(id, v_id, "Define");
            self.scope_stack.push(v_id); //让变体成为子作用域，后续字段的父实体就是该变体。
            match &var.fields {
                //Rust 语法允许枚举变体携带三种字段形态，对应 syn::Fields 的三个变体
                Fields::Named(fields) => {
                    //无字段:enum E { A }
                    for f in &fields.named {
                        if let Some(ident) = &f.ident {
                            let f_id = self.add(ident.to_string(), "Variable", ident.span());
                            self.add_rel(v_id, f_id, "Define");
                        }
                    }
                }
                Fields::Unnamed(fields) => {
                    for (i, _) in fields.unnamed.iter().enumerate() {
                        let field_name = format!("_{}", i);
                        let f_id = self.add(field_name.clone(), "Variable", var.ident.span());
                        self.add_rel(v_id, f_id, "Define");
                    }
                }
                Fields::Unit => {}
            }
            self.scope_stack.pop(); //变体处理完毕，回到枚举级作用域。
        }
        syn::visit::visit_item_enum(self, node); //让 syn 继续遍历枚举里 我们没手动处理的子节点
        self.scope_stack.pop(); //枚举整体处理结束，把枚举实体从栈里弹出。
    }

    fn visit_item_union(&mut self, node: &ItemUnion) {
        //把 union 的名字（如 Packet）注册成实体，类别 "Union"，返回全局 id。
        let id = self.add(node.ident.to_string(), "Union", node.ident.span());
        self.scope_stack.push(id); //把 union 压栈
        for f in &node.fields.named {
            //逐个取出字段名。
            if let Some(ident) = &f.ident {
                let f_id = self.add(ident.to_string(), "Variable", ident.span()); //把字段名（如 a、b）注册成 "Variable" 实体。
                self.add_rel(id, f_id, "Define");
            }
        }
        syn::visit::visit_item_union(self, node); //让 syn 继续遍历 union 内可能存在的属性、宏等。
        self.scope_stack.pop(); //union 处理完毕，回到上一层作用域。
    }

    fn visit_item_trait(&mut self, node: &ItemTrait) {
        let trait_id = self.add(node.ident.to_string(), "Trait", node.ident.span()); //创建 trait 实体
        self.scope_stack.push(trait_id); //把 trait 推进作用域栈
        for bound in &node.supertraits {
            // 处理父 trait（继承关系）
            if let TypeParamBound::Trait(trait_bound) = bound {
                if let Some(parent_trait_id) = self.resolve_path(&trait_bound.path) {
                    //如果能解析出 Drawable 的实体 id，就在关系表里加一条继承关系
                    self.add_rel(trait_id, parent_trait_id, "Inherit");
                    self.trait_hierarchy.add_subtrait(parent_trait_id, trait_id); //在 trait 层次管理器里记录谁是谁的子trait
                }
            }
        }
        for gp in &node.generics.params {
            //处理泛型生命周期参数
            if let GenericParam::Lifetime(lt) = gp {
                let lt_id = self.extract_lifetime(&lt.lifetime);
                self.add_rel(trait_id, lt_id, "Define");
            }
        }
        for item in &node.items {
            //遍历 trait 内部条目
            match item {
                TraitItem::Fn(m) => {
                    //判断是“方法”还是“普通函数”：有无 self 参数
                    let cat = if Self::is_method(&m.sig) {
                        "Method"
                    } else {
                        "Function"
                    };
                    let id = self.add(m.sig.ident.to_string(), cat, m.sig.ident.span()); //为方法建实体 → 挂到 trait 下；
                    self.trait_hierarchy.register_trait_method(
                        //往 “trait 方法表” 里插一条记录：(trait_id, "draw") -> id以后做 类层次分析（CHA） 时，通过方法名就能快速找到实现或默认实现。
                        trait_id,
                        m.sig.ident.to_string(),
                        id,
                    );
                    self.scope_stack.push(id);
                    self.extract_params(&m.sig); //把每个形参（包括 self）注册成 "Parameter" 实体，并挂到方法下同时把参数类型写到 param_type_map，供后续类型推断或 CHA 用
                    self.scope_stack.pop();
                }
                TraitItem::Const(c) => {
                    self.add(c.ident.to_string(), "AssociatedConst", c.ident.span()); //把 const X: T = ... 里的 X 注册成 "AssociatedConst" 实体，父节点依旧是 trait
                }
                TraitItem::Type(t) => {
                    self.add(t.ident.to_string(), "AssociatedType", t.ident.span()); //把 type Output; 里的 Output 注册成 "AssociatedType" 实体，父节点也是 trait
                }
                _ => {}
            }
        }
        syn::visit::visit_item_trait(self, node); //保底
        self.scope_stack.pop(); //离开 trait 作用域，保证后续节点不会再以这个 trait 为父
    }

    fn visit_item_impl(&mut self, node: &ItemImpl) {
        // syn 的 visitor 在遍历 AST 时，每碰到一个 impl 块就调用此函数
        // 辅助函数：从Type中提取类型名称
        fn extract_type_name(ty: &Type) -> Option<String> {
            match ty {
                Type::Path(type_path) => {
                    // 处理路径类型，如 std::vec::Vec<i32>
                    Some(
                        type_path
                            .path
                            .segments
                            .iter()
                            .map(|s| s.ident.to_string())
                            .collect::<Vec<_>>()
                            .join("::"),
                    )
                }
                Type::Tuple(tuple_type) => {
                    // 处理元组类型，如 (u8, u8)
                    if tuple_type.elems.is_empty() {
                        Some("()".to_string())
                    } else {
                        let elem_names: Vec<String> = tuple_type
                            .elems
                            .iter()
                            .filter_map(extract_type_name)
                            .collect();
                        Some(format!("({})", elem_names.join(", ")))
                    }
                }
                Type::Reference(ref_type) => {
                    // 处理引用类型，如 &str
                    if let Some(inner_name) = extract_type_name(&ref_type.elem) {
                        if ref_type.mutability.is_some() {
                            Some(format!("&mut {}", inner_name))
                        } else {
                            Some(format!("&{}", inner_name))
                        }
                    } else {
                        None
                    }
                }
                Type::Array(array_type) => {
                    // 处理数组类型，如 [u8; 32]
                    if let Some(inner_name) = extract_type_name(&array_type.elem) {
                        // 注意：这里简化处理，实际可能需要考虑数组长度
                        Some(format!("[{}]", inner_name))
                    } else {
                        None
                    }
                }
                Type::Slice(slice_type) => {
                    // 处理切片类型，如 [u8]
                    if let Some(inner_name) = extract_type_name(&slice_type.elem) {
                        Some(format!("[{}]", inner_name))
                    } else {
                        None
                    }
                }
                Type::Paren(TypeParen { elem, .. }) | Type::Group(TypeGroup { elem, .. }) => {
                    // 处理括号类型，如 (i32)
                    extract_type_name(elem)
                }
                Type::Ptr(ptr_type) => {
                    // 处理原始指针类型，如 *const T, *mut T
                    if let Some(inner_name) = extract_type_name(&ptr_type.elem) {
                        if ptr_type.mutability.is_some() {
                            Some(format!("*mut {}", inner_name))
                        } else {
                            Some(format!("*const {}", inner_name))
                        }
                    } else {
                        None
                    }
                }
                Type::BareFn(_) => {
                    // 处理函数指针类型
                    Some("fn".to_string())
                }
                Type::Never(_) => {
                    // 处理never类型 !
                    Some("!".to_string())
                }
                _ => {
                    // 对于其他未处理的类型，返回None
                    None
                }
            }
        }

        // 使用改进的类型名称提取逻辑
        let type_name = match extract_type_name(&*node.self_ty) {
            Some(name) => name,
            None => {
                eprintln!(
                    "Warning: Unsupported self type in impl block: {:?}",
                    node.self_ty
                );
                return;
            }
        };

        eprintln!("DEBUG: impl block for type: {}", type_name);

        // 改进的类型ID查找逻辑
        let type_id = match self.glob_name2id.get(&type_name) {
            Some(&id) => id,
            None => {
                // 如果找不到现有实体，为这个类型创建一个新的实体
                // 这对于元组类型、引用类型等特别有用
                let new_id = self.next_id;
                self.next_id += 1;

                // 创建类型实体
                let type_entity = Entity {
                    id: new_id,
                    name: type_name.clone(),
                    qualified_name: format!("crate::{}", type_name),
                    parent: Some(0), // 以crate为父节点
                    category: "Type",
                    location: None, // impl块本身不提供具体位置信息
                };

                self.entities.push(type_entity);
                self.glob_name2id.insert(type_name.clone(), new_id);
                self.path2id.insert(format!("crate::{}", type_name), new_id);

                new_id
            }
        };

        eprintln!("DEBUG: type_id: {}, type_name: {}", type_id, type_name);
        self.current_impl_type_id = Some(type_id); // 把"当前实现类型"暂存并压入作用域
        let parent = *self.scope_stack.last().unwrap();
        eprintln!("DEBUG: parent before push: {}", parent);
        self.scope_stack.push(type_id);
        eprintln!("DEBUG: scope_stack after push: {:?}", self.scope_stack);

        for gp in &node.generics.params {
            //处理泛型生命期参数
            if let GenericParam::Lifetime(lt) = gp {
                let lt_id = self.extract_lifetime(&lt.lifetime);
                self.add_rel(type_id, lt_id, "Define");
            }
        }

        if let Some((_, trait_path, _)) = &node.trait_ {
            //如果这是 impl Trait for Type 形式（而不是普通 impl Type），则进入此分支。
            if let Some(trait_id) = self.resolve_path(trait_path) {
                //把 trait 路径解析成 trait_id；解析不到就跳过
                self.trait_hierarchy.add_implementation(trait_id, type_id); //在 trait 继承体系里记录：该类型实现了该 trait。
                // 建立从类型到trait的Impl关系
                self.add_rel(type_id, trait_id, "Impl");

                for item in &node.items {
                    //遍历 impl 块中的每个成员（方法、常量等）。
                    if let ImplItem::Fn(m) = item {
                        //对每个方法，先确定它在 trait 中有没有同名原型：遍历 Method / Function 两种类别，在 owner2name2id 里找
                        let method_name = m.sig.ident.to_string();
                        let mut trait_method_id = None;
                        for &cat in &["Method", "Function"] {
                            if let Some(method_map) = self.owner2name2id.get(&(trait_id, cat)) {
                                if let Some(&id) = method_map.get(&method_name) {
                                    trait_method_id = Some(id);
                                    break;
                                }
                            }
                        }
                        let cat = if Self::is_method(&m.sig) {
                            "Method"
                        } else {
                            "AssociatedFunction"
                        };
                        let impl_method_id =
                            self.add(m.sig.ident.to_string(), cat, m.sig.ident.span()); //然后给这个方法分配一个新的实体 id
                        self.trait_hierarchy.register_impl_method(
                            type_id,
                            method_name.clone(),
                            impl_method_id,
                        );
                        if let Some(trait_method_id) = trait_method_id {
                            self.add_rel(impl_method_id, trait_method_id, "Implement");
                        }
                        self.scope_stack.push(impl_method_id);
                        self.extract_params(&m.sig);
                        self.visit_block(&m.block);
                        self.scope_stack.pop();
                    }
                }
            }
        } else {
            // 对于普通 impl Type 形式，收集所有方法ID到临时向量中
            let mut method_ids = Vec::new();
            // 添加一个向量来收集常量ID
            let mut const_ids = Vec::new();

            for item in &node.items {
                match item {
                    ImplItem::Fn(m) => {
                        eprintln!("DEBUG: Processing function in impl block: {}", m.sig.ident);
                        eprintln!(
                            "DEBUG: scope_stack before processing function: {:?}",
                            self.scope_stack
                        );
                        let cat = if Self::is_method(&m.sig) {
                            "Method"
                        } else {
                            "AssociatedFunction"
                        };
                        let impl_method_id =
                            self.add(m.sig.ident.to_string(), cat, m.sig.ident.span());
                        eprintln!("DEBUG: Created function entity with id: {}", impl_method_id);
                        method_ids.push((impl_method_id, m.sig.ident.to_string()));
                        self.trait_hierarchy.register_impl_method(
                            type_id,
                            m.sig.ident.to_string(),
                            impl_method_id,
                        );
                        self.scope_stack.push(impl_method_id);
                        eprintln!(
                            "DEBUG: scope_stack after push function: {:?}",
                            self.scope_stack
                        );
                        self.extract_params(&m.sig);
                        self.visit_block(&m.block);
                        eprintln!(
                            "DEBUG: scope_stack before pop function: {:?}",
                            self.scope_stack
                        );
                        self.scope_stack.pop();
                        eprintln!(
                            "DEBUG: scope_stack after pop function: {:?}",
                            self.scope_stack
                        );
                    }
                    ImplItem::Const(c) => {
                        // 处理impl块中的常量
                        let const_id =
                            self.add(c.ident.to_string(), "AssociatedConst", c.ident.span());
                        const_ids.push(const_id);
                    }
                    _ => {}
                }
            }

            // 为所有方法建立与类型的Define关系
            for &(method_id, _) in &method_ids {
                self.add_rel(type_id, method_id, "Define");
            }

            // 为所有常量建立与类型的Define关系
            for &const_id in &const_ids {
                self.add_rel(type_id, const_id, "Define");
            }
        }
        eprintln!("DEBUG: scope_stack before pop: {:?}", self.scope_stack);
        self.scope_stack.pop();
        eprintln!("DEBUG: scope_stack after pop: {:?}", self.scope_stack);
        self.current_impl_type_id = None;
        assert_eq!(*self.scope_stack.last().unwrap(), parent);
        syn::visit::visit_item_impl(self, node);
    }

    fn visit_item_fn(&mut self, node: &ItemFn) {
        let cat = if Self::is_method(&node.sig) {
            "Method"
        } else if self.scope_stack.len() >= 2 {
            let parent_index = self.scope_stack.len() - 2;
            if let Some(&parent_id) = self.scope_stack.get(parent_index) {
                if parent_id >= 0 && (parent_id as usize) < self.entities.len() {
                    match self.entities[parent_id as usize].category {
                        "Struct" | "Enum" | "Union" => "AssociatedFunction",
                        _ => "Function",
                    }
                } else {
                    "Function"
                }
            } else {
                "Function"
            }
        } else {
            "Function"
        };
        let fn_id = self.add(node.sig.ident.to_string(), cat, node.sig.ident.span());
        self.scope_stack.push(fn_id);
        for gp in &node.sig.generics.params {
            if let GenericParam::Lifetime(lt) = gp {
                let lt_id = self.extract_lifetime(&lt.lifetime);
                self.add_rel(fn_id, lt_id, "Define");
            }
        }
        for input in &node.sig.inputs {
            match input {
                FnArg::Receiver(_) => {}
                FnArg::Typed(pat_type) => {
                    let name;
                    let span;
                    let ty = &pat_type.ty;
                    match &*pat_type.pat {
                        Pat::Ident(pat_ident) => {
                            name = pat_ident.ident.to_string();
                            span = pat_ident.ident.span();
                        }
                        Pat::Type(pat_inner) => {
                            if let Pat::Ident(pat_ident) = &*pat_inner.pat {
                                name = pat_ident.ident.to_string();
                                span = pat_ident.ident.span();
                            } else {
                                eprintln!(
                                    "Unsupported pattern in function parameter: {:?}",
                                    pat_type.pat
                                );
                                continue;
                            }
                        }
                        _ => {
                            eprintln!(
                                "Unsupported pattern in function parameter: {:?}",
                                pat_type.pat
                            );
                            continue;
                        }
                    }
                    let param_id = self.add(name.clone(), "Parameter", span);
                    self.add_rel(fn_id, param_id, "Contain");
                    eprintln!("Function parameter: {} : {:?}", name, ty);
                    if let Some(trait_id) = self.extract_trait_object_type(ty) {
                        self.param_type_map.insert(param_id, trait_id);
                        eprintln!(" Trait ID extracted: {}", trait_id);
                    } else {
                        eprintln!(" NO Trait ID extracted for type: {:?}", ty);
                    }
                }
            }
        }
        self.visit_block(&node.block);
        self.scope_stack.pop();
    }

    fn visit_expr_call(&mut self, node: &ExprCall) {
        // 获取调用者的实体ID（函数或方法）
        let caller = self.get_proper_caller();
        // 检查被调用的函数是否为路径表达式（如 foo() 或 module::foo()）
        if let Expr::Path(path_expr) = &*node.func {
            // 尝试解析函数路径
            if let Some(callee_id) = self.resolve_path(&path_expr.path) {
                // 如果解析成功，建立调用关系
                self.add_rel(caller, callee_id, "Call");
            } else if let Some(ident) = path_expr.path.get_ident() {
                // 如果解析失败但能获取标识符，将其添加到未解析调用列表中
                self.unresolved_calls
                    .push((caller, ident.to_string(), ident.span()));
            }
        }
        // 调用syn库的默认实现继续遍历子节点
        syn::visit::visit_expr_call(self, node);
    }

    fn visit_expr_path(&mut self, node: &ExprPath) {
        let user = self.get_proper_caller();
        if node.path.segments.len() > 1 {
            if let Some(id) = self.resolve_path(&node.path) {
                // 检查目标实体的类别，避免为常量创建Call关系
                if id >= 0 && (id as usize) < self.entities.len() {
                    let category = self.entities[id as usize].category;
                    match category {
                        // 对于函数和方法，创建Call关系
                        "Function" | "Method" | "AssociatedFunction" => {
                            self.add_rel(user, id, "Call");
                        }
                        // 对于常量，创建UseVar关系而不是Call关系
                        "Constant" | "AssociatedConst" => {
                            self.add_rel(user, id, "UseVar");
                        }
                        // 对于其他类型的实体，默认创建Call关系（保持原有行为）
                        _ => {
                            self.add_rel(user, id, "Call");
                        }
                    }
                } else {
                    // 如果无法确定实体类别，默认创建Call关系（保持原有行为）
                    self.add_rel(user, id, "Call");
                }
                return;
            }
        }
        if let Some(ident) = node.path.get_ident() {
            let name = ident.to_string();
            for &scope in self.scope_stack.iter().rev() {
                // 检查变量使用
                if let Some(map) = self.owner2name2id.get(&(scope, "Variable")) {
                    if let Some(&var_id) = map.get(&name) {
                        self.add_rel(user, var_id, "UseVar");
                        return;
                    }
                }
                // 检查参数使用
                if let Some(map) = self.owner2name2id.get(&(scope, "Parameter")) {
                    if let Some(&param_id) = map.get(&name) {
                        self.add_rel(user, param_id, "UseVar");
                        return;
                    }
                }
                // 检查常量使用
                if let Some(map) = self.owner2name2id.get(&(scope, "Constant")) {
                    if let Some(&const_id) = map.get(&name) {
                        self.add_rel(user, const_id, "UseVar");
                        return;
                    }
                }
            }
        }
        syn::visit::visit_expr_path(self, node);
    }

    fn visit_expr_assign(&mut self, node: &ExprAssign) {
        // 处理左操作数的修改关系
        self.process_assignment_left_operand(&node.left);

        // 遍历右操作数
        self.visit_expr(&node.right);
    }

    fn visit_expr_binary(&mut self, node: &ExprBinary) {
        // 检查是否是复合赋值操作
        match node.op {
            // 处理复合赋值操作符
            BinOp::AddAssign(_)
            | BinOp::SubAssign(_)
            | BinOp::MulAssign(_)
            | BinOp::DivAssign(_)
            | BinOp::RemAssign(_)
            | BinOp::BitAndAssign(_)
            | BinOp::BitOrAssign(_)
            | BinOp::BitXorAssign(_)
            | BinOp::ShlAssign(_)
            | BinOp::ShrAssign(_) => {
                // 处理左操作数的修改关系
                self.process_assignment_left_operand(&node.left);

                // 只遍历操作数，避免对左操作数重复处理
                self.visit_expr(&node.left);
                self.visit_expr(&node.right);
                return;
            }
            _ => {
                // 对于其他二元操作，使用默认处理
            }
        }

        // 对于非复合赋值的二元操作，使用默认处理
        syn::visit::visit_expr_binary(self, node);
    }

    fn visit_expr_reference(&mut self, node: &ExprReference) {
        // 不再在表达式级别直接记录引用关系
        // 引用关系应该在变量声明时建立
        syn::visit::visit_expr_reference(self, node);
    }

    fn visit_item_const(&mut self, node: &ItemConst) {
        self.add(node.ident.to_string(), "Constant", node.ident.span());
    }

    fn visit_item_static(&mut self, node: &ItemStatic) {
        self.add(node.ident.to_string(), "Static", node.ident.span());
    }

    fn visit_item_type(&mut self, node: &ItemType) {
        self.add(node.ident.to_string(), "TypeAlias", node.ident.span());
    }

    fn visit_expr_async(&mut self, node: &ExprAsync) {
        let async_id = self.add(
            format!("async_{}", self.next_id),
            "AsyncBlock",
            node.async_token.span,
        );
        self.scope_stack.push(async_id);
        for stmt in &node.block.stmts {
            self.visit_stmt(stmt);
        }
        self.scope_stack.pop();
    }

    fn visit_expr_closure(&mut self, node: &ExprClosure) {
        let closure_id = self.add(format!("closure_{}", self.next_id), "Closure", node.span());
        self.scope_stack.push(closure_id);
        if let Expr::Block(ExprBlock { block, .. }) = &*node.body {
            for stmt in &block.stmts {
                self.visit_stmt(stmt);
            }
        }
        self.scope_stack.pop();
    }

    fn visit_item_foreign_mod(&mut self, node: &ItemForeignMod) {
        let abi = node
            .abi
            .name
            .as_ref()
            .map(|l| l.value())
            .unwrap_or_else(|| "C".to_string());
        let block_id = self.add(
            format!("extern_{}", abi),
            "ExternalBlock",
            node.abi.extern_token.span,
        );
        self.scope_stack.push(block_id);
        for item in &node.items {
            match item {
                ForeignItem::Fn(f) => {
                    self.add(f.sig.ident.to_string(), "Function", f.sig.ident.span());
                }
                ForeignItem::Static(s) => {
                    self.add(s.ident.to_string(), "Static", s.ident.span());
                }
                ForeignItem::Type(t) => {
                    self.add(t.ident.to_string(), "TypeAlias", t.ident.span());
                }
                _ => {}
            }
        }
        self.scope_stack.pop();
        let parent = *self.scope_stack.last().unwrap();
        self.add_rel(parent, block_id, "Contain");
    }

    fn visit_expr_struct(&mut self, node: &'ast ExprStruct) {
        if let Some(struct_id) = self.resolve_path(&node.path) {
            let caller = self.get_proper_caller();
            // 不再创建Call关系，避免在构造结构体字面量时提取出Call关系
            // self.add_rel(caller, struct_id, "Call");
        }
        syn::visit::visit_expr_struct(self, node);
    }

    fn visit_block(&mut self, node: &Block) {
        // 判断是否需要为当前块创建作用域
        let should_create_scope = match self.scope_stack.last() {
            // 检查作用域栈顶的实体
            Some(&id) if id >= 0 && (id as usize) < self.entities.len() => {
                // 如果栈顶实体的类别是函数、方法、闭包、异步块或块，则需要创建作用域
                matches!(
                    self.entities[id as usize].category,
                    "Function" | "Method" | "Closure" | "AsyncBlock" | "Block"
                )
            }
            // 其他情况不需要创建作用域
            _ => false,
        };

        if should_create_scope {
            // 如果需要创建作用域，使用with_block_scope方法处理块
            self.with_block_scope(node.span(), |this| {
                // 输出调试信息
                eprintln!(
                    "DEBUG: Inside with_block_scope, scope_stack: {:?}",
                    this.scope_stack
                );
                // 遍历块中的所有语句
                for stmt in &node.stmts {
                    this.visit_stmt(stmt);
                }
            });
        } else {
            // 如果不需要创建作用域，直接遍历块中的所有语句
            for stmt in &node.stmts {
                self.visit_stmt(stmt);
            }
        }
    }

    fn visit_stmt(&mut self, stmt: &Stmt) {
        // 根据语句类型进行不同处理
        match stmt {
            // 处理局部变量声明语句（let语句）
            Stmt::Local(local) => {
                // 创建一个向量来存储创建的变量ID
                let mut var_ids = Vec::new();

                // 定义一个递归函数来处理模式
                fn process_pattern(pat: &Pat, visitor: &mut Extractor, var_ids: &mut Vec<i32>) {
                    match pat {
                        // 处理标识符模式，如 let x = 5;
                        Pat::Ident(pat_ident) => {
                            // 获取变量名
                            let name = pat_ident.ident.to_string();
                            // 为变量创建实体
                            let var_id =
                                visitor.add(name.clone(), "Variable", pat_ident.ident.span());
                            // 建立变量与当前作用域的包含关系
                            visitor.add_rel(
                                *visitor.scope_stack.last().unwrap(),
                                var_id,
                                "Contain",
                            );
                            // 将变量ID添加到向量中
                            var_ids.push(var_id);
                        }
                        // 处理元组模式，如 let (x, y) = (1, 2);
                        Pat::Tuple(tuple_pat) => {
                            // 递归处理元组中的每个元素
                            for elem_pat in &tuple_pat.elems {
                                process_pattern(elem_pat, visitor, var_ids);
                            }
                        }
                        // 处理切片模式，如 let [x, y] = [1, 2];
                        Pat::Slice(slice_pat) => {
                            // 递归处理切片中的每个元素
                            for elem_pat in &slice_pat.elems {
                                process_pattern(elem_pat, visitor, var_ids);
                            }
                        }
                        // 处理引用模式，如 let &x = &y;
                        Pat::Reference(ref_pat) => {
                            // 递归处理引用的内部模式
                            process_pattern(&ref_pat.pat, visitor, var_ids);
                        }
                        // 处理括号模式，如 let (x) = y;
                        Pat::Paren(paren_pat) => {
                            // 递归处理括号内的模式
                            process_pattern(&paren_pat.pat, visitor, var_ids);
                        }
                        // 处理带类型注解的模式，如 let x: i32 = 5;
                        Pat::Type(pat_type) => {
                            // 递归处理类型注解内的模式
                            process_pattern(&pat_type.pat, visitor, var_ids);
                        }
                        _ => {
                            // 对于其他未处理的模式类型，不创建变量实体
                        }
                    }
                }

                // 处理不同的模式类型
                match &local.pat {
                    // 处理简单标识符模式
                    Pat::Ident(pat_ident) => {
                        // 获取变量名
                        let name = pat_ident.ident.to_string();
                        // 为变量创建实体
                        let var_id = self.add(name.clone(), "Variable", pat_ident.ident.span());
                        // 建立变量与当前作用域的包含关系
                        self.add_rel(*self.scope_stack.last().unwrap(), var_id, "Contain");
                        // 将变量ID添加到向量中
                        var_ids.push(var_id);

                        // 特别处理引用类型的初始化
                        if let Some(init) = &local.init {
                            // 处理引用初始化关系
                            self.process_reference_initialization(var_id, &init.expr);
                        }
                    }
                    // 处理带类型注解的模式
                    Pat::Type(pat_type) => {
                        // 递归处理带类型的模式
                        process_pattern(&pat_type.pat, self, &mut var_ids);

                        // 为所有创建的变量添加类型信息
                        for &var_id in &var_ids {
                            // 提取trait对象类型信息
                            if let Some(trait_id) = self.extract_trait_object_type(&pat_type.ty) {
                                // 记录参数类型映射
                                self.param_type_map.insert(var_id, trait_id);
                            }
                        }

                        // 特别处理引用类型的初始化
                        if let Some(init) = &local.init {
                            // 如果有创建变量，处理第一个变量的引用初始化关系
                            if let Some(&var_id) = var_ids.first() {
                                self.process_reference_initialization(var_id, &init.expr);
                            }
                        }
                    }
                    // 处理元组模式
                    Pat::Tuple(tuple_pat) => {
                        // 处理不带类型的元组模式
                        for pat in &tuple_pat.elems {
                            process_pattern(pat, self, &mut var_ids);
                        }
                    }
                    // 处理切片模式
                    Pat::Slice(slice_pat) => {
                        // 处理不带类型的数组切片模式
                        for pat in &slice_pat.elems {
                            process_pattern(pat, self, &mut var_ids);
                        }
                    }
                    // 处理引用模式
                    Pat::Reference(ref_pat) => {
                        // 处理引用模式
                        process_pattern(&ref_pat.pat, self, &mut var_ids);
                    }
                    // 处理其他模式类型
                    _ => {
                        // 对于其他未处理的模式类型，尝试递归处理
                        process_pattern(&local.pat, self, &mut var_ids);

                        // 特别处理引用类型的初始化
                        if let Some(init) = &local.init {
                            // 如果有创建变量，处理第一个变量的引用初始化关系
                            if let Some(&var_id) = var_ids.first() {
                                self.process_reference_initialization(var_id, &init.expr);
                            }
                        }
                    }
                }

                // 为所有创建的变量推断类型
                if let Some(init) = &local.init {
                    // 遍历所有创建的变量
                    for &var_id in &var_ids {
                        // 推断变量类型
                        self.infer_variable_type(var_id, &init.expr);
                    }
                }

                // 调用syn库的默认实现继续遍历子节点
                syn::visit::visit_stmt(self, stmt);
            }
            // 处理其他类型的语句
            _ => syn::visit::visit_stmt(self, stmt),
        }
    }
    fn visit_expr_unary(&mut self, node: &ExprUnary) {
        match node.op {
            UnOp::Deref(_) => {
                // 处理解引用操作 *expr
                if let Expr::Path(path_expr) = &*node.expr {
                    if let Some(ident) = path_expr.path.get_ident() {
                        // 记录使用了引用变量
                        let user = self.get_proper_caller();
                        self.record_var_use(user, &ident.to_string());
                    }
                }
            }
            _ => {
                // 其他一元操作使用默认处理
            }
        }

        // 继续遍历子表达式
        syn::visit::visit_expr_unary(self, node);
    }
}
