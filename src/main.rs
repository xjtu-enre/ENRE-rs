//// 程序唯一入口：解析命令行→读文件→驱动 Extractor→序列化 JSON
//// 程序唯一入口：解析命令行→读文件→驱动 Extractor→序列化 JSON
use crate::entity::{Entity, Output, Relation};
use crate::extractor::Extractor;
use std::{collections::HashSet, env, fs};
use syn::visit::Visit;
use syn::*;
mod entity;
mod extractor;
mod hierarchy;
mod util;
mod visitor;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let path = env::args()
        .nth(1)
        .expect("usage: extractor <rust-source-file>");
    let source = fs::read_to_string(&path)?;
    let ast = parse_file(&source)?;
    //dbg!(&ast);
    let mut ex = Extractor::new(0);
    ex.visit_file(&ast);
    ex.resolve_unresolved_calls();

    // 收集所有block实体的ID
    let block_ids: HashSet<i32> = ex
        .entities
        .iter()
        .filter(|e| e.category == "Block")
        .map(|e| e.id)
        .collect();

    // 修复实体的parent字段，跳过block实体
    let mut fixed_entities: Vec<Entity> = ex
        .entities
        .into_iter()
        .map(|mut e| {
            // 如果实体的parent是block实体，则需要找到有效的祖先实体
            if let Some(parent_id) = e.parent {
                if block_ids.contains(&parent_id) {
                    // 需要找到第一个非block的祖先实体
                    let mut current_parent_id = parent_id;
                    while block_ids.contains(&current_parent_id) {
                        // 从关系中查找当前block实体的parent
                        if let Some(relation) = ex
                            .relations
                            .iter()
                            .find(|r| r.to == current_parent_id && r.category == "Define")
                        {
                            current_parent_id = relation.from;
                        } else {
                            // 如果找不到Define关系，就将parent设为None
                            e.parent = None;
                            break;
                        }
                    }
                    // 确保最终的parent不是block实体
                    if block_ids.contains(&current_parent_id) {
                        e.parent = None;
                    } else {
                        e.parent = Some(current_parent_id);
                    }
                }
            }
            e
        })
        .collect();

    // 重建跨越block实体的关系
    let mut fixed_relations = ex.relations.clone();

    // 为每个被过滤的block实体重建"Define"关系
    for &block_id in &block_ids {
        // 找到从其他实体到block实体的"Define"关系
        let source_relations: Vec<Relation> = ex
            .relations
            .iter()
            .filter(|r| r.to == block_id && r.category == "Define")
            .cloned()
            .collect();

        // 找到从block实体到其他实体的"Define"关系
        let target_relations: Vec<Relation> = ex
            .relations
            .iter()
            .filter(|r| r.from == block_id && r.category == "Define")
            .cloned()
            .collect();

        // 为每个源实体和目标实体之间创建新的"Define"关系
        for source_rel in &source_relations {
            for target_rel in &target_relations {
                // 创建跨越block的新关系
                let new_relation = Relation {
                    from: source_rel.from,
                    to: target_rel.to,
                    category: "Define", // 修正：使用&str而不是String
                };

                // 检查是否已存在相同的关系，避免重复
                if !fixed_relations.iter().any(|r| {
                    r.from == new_relation.from
                        && r.to == new_relation.to
                        && r.category == new_relation.category
                }) {
                    fixed_relations.push(new_relation);
                }
            }
        }
    }

    // 过滤掉block实体
    let filtered_entities: Vec<Entity> = fixed_entities
        .into_iter()
        .filter(|e| !block_ids.contains(&e.id))
        .collect();

    // 过滤掉涉及block实体的关系
    let filtered_relations: Vec<Relation> = fixed_relations
        .into_iter()
        .filter(|r| !block_ids.contains(&r.from) && !block_ids.contains(&r.to))
        .collect();

    let output = Output {
        entities: filtered_entities,
        relations: filtered_relations,
        meta: (),
    };
    let json = serde_json::to_string_pretty(&output)?;
    let output_path = "output.json";
    fs::write(output_path, json)?;
    println!("✅ 已成功保存到 {}", output_path);
    Ok(())
}
