//// 小工具：目前仅 span→Range 的转换函数
use crate::entity::Position;
use proc_macro2::Span;
pub fn span_to_range(span: Span) -> crate::entity::Range {
    //，它接受一个Span参数并返回crate::entity::Range类型。这个函数的作用是将Span转换为Range。
    let start = span.start(); //获取span的起始位置信息，start()方法返回一个包含行号和列号的结构体。
    let end = span.end(); //获取span的结束位置信息，end()方法也返回一个包含行号和列号的结构体。
    if start.line == 0 || end.line == 0 {
        //检查起始位置或结束位置的行号是否为0，这通常表示无效的位置信息。
        return crate::entity::Range {
            start: Position { line: 0, column: 0 },
            end: None,
        };
    }
    crate::entity::Range {
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
