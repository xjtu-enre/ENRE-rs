//// TraitHierarchy：专门处理 trait 继承链与 CHA（类层次分析）
use std::collections::{HashMap, HashSet};
pub struct TraitHierarchy {
    pub subtraits: HashMap<i32, Vec<i32>>, //记录trait之间的继承关系
    pub implementations: HashMap<i32, Vec<i32>>, //记录trait与具体类型的实现关系
    pub trait_methods: HashMap<(i32, String), i32>, // 记录trait中定义的方法（trait_id + 方法名 -> 方法实体id）
    pub impl_methods: HashMap<i32, HashMap<String, i32>>, //记录具体类型实现的方法（类型id -> 方法名 -> 方法实体id）
}

impl TraitHierarchy {
    pub fn new() -> Self {
        //用于创建TraitHierarchy实例。
        Self {
            subtraits: HashMap::new(),
            implementations: HashMap::new(),
            trait_methods: HashMap::new(),
            impl_methods: HashMap::new(),
        }
    }

    pub fn add_subtrait(&mut self, parent_trait_id: i32, subtrait_id: i32) {
        //用于添加trait之间的继承关系。参数为父trait ID和子trait ID。
        //在subtraits HashMap中，以parent_trait_id为键获取或创建一个空的Vec，然后将subtrait_id添加到该Vec中。
        self.subtraits
            .entry(parent_trait_id)
            .or_default()
            .push(subtrait_id);
    }

    pub fn add_implementation(&mut self, trait_id: i32, type_id: i32) {
        //用于添加trait与具体类型的实现关系。参数为trait ID和类型ID。
        //在implementations HashMap中，以trait_id为键获取或创建一个空的Vec，然后将type_id添加到该Vec中。
        self.implementations
            .entry(trait_id)
            .or_default()
            .push(type_id);
    }

    pub fn register_trait_method(&mut self, trait_id: i32, method_name: String, method_id: i32) {
        //用于注册trait中定义的方法。参数为trait ID、方法名和方法ID。
        //在trait_methods HashMap中，以(trait_id, method_name)元组为键插入method_id。
        self.trait_methods
            .insert((trait_id, method_name), method_id);
    }

    pub fn register_impl_method(&mut self, type_id: i32, method_name: String, method_id: i32) {
        //用于注册具体类型实现的方法。参数为类型ID、方法名和方法ID。
        //以type_id为键获取或创建一个空的HashMap，然后在该HashMap中插入(method_name, method_id)键值对。
        self.impl_methods
            .entry(type_id)
            .or_default()
            .insert(method_name, method_id);
    }

    pub fn cha_find_method_targets(&self, trait_id: i32, method_name: &str) -> Vec<i32> {
        //用于CHA算法中查找方法的目标。参数为trait ID和方法名，返回值是可能的目标方法ID列表。
        let mut visited = HashSet::new(); //创建一个HashSet用于跟踪已访问的trait ID，避免重复处理。
        let mut result = Vec::new(); //创建一个Vec用于存储找到的目标方法ID。
        self.cha_find_method_targets_impl(trait_id, method_name, &mut visited, &mut result); //调用内部递归方法cha_find_method_targets_impl来实际查找方法目标
        result.sort_unstable(); //排序
        result.dedup(); //去重
        result
    }

    pub fn cha_find_method_targets_impl(
        //实现CHA算法的核心逻辑。参数包括trait ID、方法名、已访问集合和输出向量。
        &self,
        trait_id: i32,
        method_name: &str,
        visited: &mut HashSet<i32>,
        out: &mut Vec<i32>,
    ) {
        if !visited.insert(trait_id) {
            //尝试将trait_id插入已访问集合，如果插入失败（说明已存在），则直接返回，避免重复处理。
            return;
        }

        if let Some(implementations) = self.implementations.get(&trait_id) {
            //尝试从implementations HashMap中获取实现了trait_id的所有类型。
            for &type_id in implementations {
                //遍历所有实现了该trait的类型。
                if let Some(method_map) = self.impl_methods.get(&type_id) {
                    //尝试从impl_methods HashMap中获取该类型实现的所有方法。
                    if let Some(&method_id) = method_map.get(method_name) {
                        //尝试在方法映射中查找指定名称的方法。
                        out.push(method_id);
                    }
                }
            }
        }
    }
}
