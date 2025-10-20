//  // 纯数据结构：Entity、Relation、Location、Output 以及它们的序列化定义
use serde::Serialize; //导入serde库中的Serialize trait，用于将结构体序列化为JSON格式。
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    //将Span转换为Range时使用
    pub line: usize,
    pub column: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Range {
    pub start: Position,
    pub end: Option<Position>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub file: i32,
    pub identifier: Range,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block: Option<Range>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    pub id: i32,
    pub name: String,
    pub qualified_name: String,
    pub parent: Option<i32>,
    pub category: &'static str,
    pub location: Option<Location>,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Relation {
    pub from: i32,
    pub to: i32,
    pub category: &'static str,
}

#[derive(Serialize)]
pub struct Output {
    pub entities: Vec<Entity>,
    pub relations: Vec<Relation>,
    pub meta: (),
}
