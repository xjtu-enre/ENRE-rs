// 实体总数：79
// 关系总数：141

// [实体] Module: crate::command, crate::common, crate::structure::dict, crate::structure::dict_ht (通过 use 导入)
// [关系] Use: 使用 crate 中的多个模块和项
use crate::{
    command::Command,
    common,
    structure::{dict::Dict, dict_ht::DictEntry},
};
// [实体] Module: PartialEq (通过 use 导入)
// [关系] Use: 使用 PartialEq trait
use PartialEq;
// [实体] Module: ToString (通过 use 导入)
// [关系] Use: 使用 ToString trait
use ToString;
// [实体] Module: common::date_util (通过 use 导入, 重命名为 DateUtil)
// [关系] Use: 使用 common::date_util 模块并重命名为 DateUtil
use common::date_util as DateUtil;
// [实体] Module: std::hash, std::vec (通过 use 导入)
// [关系] Use: 使用 std::hash 和 std::vec 模块
use std::{hash::Hash, vec};
// [实体] Module: tokio::sync::mpsc (通过 use 导入)
// [关系] Use: 使用 tokio::sync::mpsc 模块
use tokio::sync::mpsc;

// [实体] Struct: DbHandler
// [关系] Define: 定义结构体 DbHandler
// [关系] Inherit: 通过 #[derive(Debug)] 继承 Debug trait 的行为
// [关系] Contain: DbHandler 包含 sender_list 字段
#[derive(Debug)]
pub struct DbHandler {
    // [实体] Variable: sender_list (字段)
    sender_list: Vec<crate::MpscSender>,
}

// [实体] Struct: KedisKey
// [关系] Define: 定义结构体 KedisKey
// [关系] Inherit: 通过 #[derive(Debug)] 继承 Debug trait 的行为
// [关系] Contain: KedisKey 包含 key, ttl 字段
#[derive(Debug)]
pub struct KedisKey {
    // [实体] Variable: key (字段)
    key: String,
    // [实体] Variable: ttl (字段)
    ttl: i128,
}

// [关系] Impl: 为 KedisKey 实现方法
impl KedisKey {
    // [实体] Function/Method: new
    // [关系] Define: 定义方法 new
    // [实体] Variable: key (参数)
    pub fn new(key: String) -> Self {
        // [关系] Call: 调用 KedisKey 结构体字面量
        // [关系] UseVar: 使用变量 key
        return KedisKey { key, ttl: -1 };
    }
    // [实体] Function/Method: set_ttl
    // [关系] Define: 定义方法 set_ttl
    // [实体] Variable: ttl (参数)
    pub fn set_ttl(&mut self, ttl: i128) {
        // [关系] Modify: 修改 self.ttl
        // [关系] UseVar: 使用变量 self.ttl, ttl
        self.ttl = ttl;
    }
}

// [实体] Trait: PartialEq (通过 impl 知道其存在)
// [关系] Impl: 实现 PartialEq trait for KedisKey
impl PartialEq for KedisKey {
    // [实体] Function/Method: eq
    // [关系] Define: 定义方法 eq
    // [实体] Variable: other (参数)
    fn eq(&self, other: &Self) -> bool {
        // [关系] UseVar: 使用变量 self.key, other.key
        self.key == other.key
    }
}

// [实体] Trait: Hash (通过 impl 知道其存在)
// [关系] Impl: 实现 Hash trait for KedisKey
impl Hash for KedisKey {
    // [实体] Function/Method: hash
    // [关系] Define: 定义方法 hash
    // [实体] Variable: state (参数)
    // [实体] Variable: H (泛型参数)
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // [关系] Call: 调用 self.key.hash
        // [关系] UseVar: 使用变量 self.key, state
        self.key.hash(state);
    }
}

// [实体] Trait: ToString (通过 impl 知道其存在)
// [关系] Impl: 实现 ToString trait for KedisKey
impl ToString for KedisKey {
    // [实体] Function/Method: to_string
    // [关系] Define: 定义方法 to_string
    fn to_string(&self) -> String {
        // [关系] Call: 调用 self.key.clone
        // [关系] UseVar: 使用变量 self.key
        return self.key.clone();
    }
}

// [实体] Struct: Db
// [关系] Define: 定义结构体 Db
// [关系] Inherit: 通过 #[derive(Debug)] 继承 Debug trait 的行为
// [关系] Contain: Db 包含 dict, sender, receiver 字段
#[derive(Debug)]
pub struct Db {
    // [实体] Variable: dict (字段)
    dict: Dict<KedisKey, Structure>,
    // [实体] Variable: sender (字段)
    sender: crate::MpscSender,
    // [实体] Variable: receiver (字段)
    receiver: crate::MpscReceiver,
}

// [实体] Enum: Structure
// [关系] Define: 定义枚举 Structure
// [关系] Inherit: 通过 #[derive(Debug)] 继承 Debug trait 的行为
#[derive(Debug)]
pub enum Structure {
    // [实体] Variable: String (变体)
    String(String),
    // [实体] Variable: Hash (变体)
    Hash,
    // [实体] Variable: List (变体)
    List,
    // [实体] Variable: Set (变体)
    Set,
    // [实体] Variable: SortSet (变体)
    SortSet,
}

// [关系] Impl: 为 DbHandler 实现方法
impl DbHandler {
    // [实体] Function/Method: new
    // [关系] Define: 定义方法 new
    // [实体] Variable: size (参数)
    // [实体] Variable: db_list (局部变量)
    // [实体] Variable: sender_list (局部变量)
    pub fn new(size: usize) -> Self {
        // [关系] Call: 调用 vec![]
        // [关系] UseVar: 使用变量 db_list
        let mut db_list = vec![];
        // [关系] Call: 调用 vec![]
        // [关系] UseVar: 使用变量 sender_list
        let mut sender_list = vec![];
        // [实体] Variable: idx (for 循环变量)
        // [关系] UseVar: 使用变量 size
        for _idx in 0..size {
            // [实体] Variable: db (局部变量)
            // [关系] Call: 调用 Db::new
            // [关系] UseVar: 使用变量 db
            let db = Db::new();
            // [关系] Call: 调用 sender_list.push, db.sender.clone
            // [关系] Modify: 修改 sender_list
            // [关系] UseVar: 使用变量 sender_list, db.sender
            sender_list.push(db.sender.clone());
            // [关系] Call: 调用 db_list.push
            // [关系] Modify: 修改 db_list
            // [关系] UseVar: 使用变量 db_list, db
            db_list.push(db);
        }
        // [实体] Variable: db (for 循环变量)
        // [关系] UseVar: 使用变量 db_list
        for mut db in db_list {
            // [关系] Call: 调用 tokio::spawn, async move, db.run().await
            // [关系] UseVar: 使用变量 db
            tokio::spawn(async move {
                db.run().await;
            });
        }
        // [关系] Call: 调用 DbHandler 结构体字面量
        // [关系] UseVar: 使用变量 sender_list
        return DbHandler { sender_list };
    }
    // [实体] Function/Method: get_sender
    // [关系] Define: 定义方法 get_sender
    // [实体] Variable: idx (参数)
    pub fn get_sender(&self, idx: usize) -> Option<crate::MpscSender> {
        // [关系] Call: 调用 self.sender_list.get, map, item.clone
        // [关系] UseVar: 使用变量 self.sender_list, idx
        return self.sender_list.get(idx).map(|item| item.clone());
    }
    // [实体] Function/Method: get_size
    // [关系] Define: 定义方法 get_size
    pub fn get_size(&self) -> usize {
        // [关系] Call: 调用 self.sender_list.len
        // [关系] UseVar: 使用变量 self.sender_list
        return self.sender_list.len();
    }
}

// [关系] Impl: 为 Db 实现方法
impl Db {
    // [实体] Function/Method: new
    // [关系] Define: 定义方法 new
    pub fn new() -> Self {
        // [实体] Variable: tx (局部变量)
        // [实体] Variable: rx (局部变量)
        // [关系] Call: 调用 mpsc::channel
        // [关系] UseVar: 使用变量 tx, rx
        let (tx, rx) = mpsc::channel(1024);
        // [关系] Call: 调用 Db 结构体字面量
        // [关系] Call: 调用 Dict::new
        // [关系] UseVar: 使用变量 dict, sender, receiver, tx, rx
        return Db {
            dict: Dict::new(1024),
            sender: tx,
            receiver: rx,
        };
    }
    // [实体] Function/Method: run
    // [关系] Define: 定义方法 run
    async fn run(&mut self) {
        // [实体] Variable: sender (while let 变量)
        // [实体] Variable: command (while let 变量)
        // [关系] Call: 调用 self.receiver.recv().await
        // [关系] UseVar: 使用变量 self.receiver
        while let Some((sender, command)) = self.receiver.recv().await {
            // [实体] Variable: frame (局部变量)
            // [实体] Variable: get (match arm 变量)
            // [实体] Variable: set (match arm 变量)
            // [实体] Variable: scan (match arm 变量)
            // [实体] Variable: ttl (match arm 变量)
            // [实体] Variable: exists (match arm 变量)
            // [实体] Variable: expire (match arm 变量)
            // [关系] UseVar: 使用变量 command
            let frame = match command {
                // [关系] Call: 调用 get.apply
                // [关系] UseVar: 使用变量 get, self
                Command::Get(get) => get.apply(self),
                // [关系] Call: 调用 set.apply
                // [关系] UseVar: 使用变量 set, self
                Command::Set(set) => set.apply(self),
                // [关系] Call: 调用 scan.apply
                // [关系] UseVar: 使用变量 scan, self
                Command::Scan(scan) => scan.apply(self),
                // [关系] Call: 调用 scan.apply (注意这里原文是 Command::Type(scan) => scan.apply(self), 可能是笔误)
                // [关系] UseVar: 使用变量 scan, self
                Command::Type(scan) => scan.apply(self),
                // [关系] Call: 调用 ttl.apply
                // [关系] UseVar: 使用变量 ttl, self
                Command::Ttl(ttl) => ttl.apply(self),
                // [关系] Call: 调用 exists.apply
                // [关系] UseVar: 使用变量 exists, self
                Command::Exists(exists) => exists.apply(self),
                // [关系] Call: 调用 expire.apply
                // [关系] UseVar: 使用变量 expire, self
                Command::Expire(expire) => expire.apply(self),
                _ => Err("Error".into()),
            };
            // [关系] Call: 调用 sender.send
            // [关系] UseVar: 使用变量 sender, frame
            let _ = sender.send(frame);
        }
    }
    // [实体] Function/Method: get_pattern_entry
    // [关系] Define: 定义方法 get_pattern_entry
    // [实体] Variable: pre_idx (参数)
    // [实体] Variable: match_str (参数)
    // [实体] Variable: count (参数)
    // [实体] Variable: item (局部变量)
    // [实体] Variable: list (局部变量)
    // [实体] Variable: key (for 循环变量)
    pub fn get_pattern_entry(
        &mut self,
        pre_idx: usize,
        match_str: String,
        count: usize,
    ) -> crate::Result<(usize, Vec<String>)> {
        // [关系] Call: 调用 self.dict.get_pattern_entry
        // [关系] UseVar: 使用变量 self.dict, pre_idx, match_str, count
        let item = self.dict.get_pattern_entry(pre_idx, match_str, count)?;
        // [关系] Call: 调用 vec![]
        // [关系] UseVar: 使用变量 list
        let mut list = vec![];
        // [关系] UseVar: 使用变量 item.1
        for key in item.1 {
            // [关系] Call: 调用 KedisKey::new, self.remove_expired_key
            // [关系] Call: 调用 key.clone
            // [关系] UseVar: 使用变量 key
            if !self.remove_expired_key(&KedisKey::new(key.clone())) {
                // [关系] Call: 调用 list.push
                // [关系] Modify: 修改 list
                // [关系] UseVar: 使用变量 list, key
                list.push(key);
            }
        }
        // [关系] Call: 调用 Ok
        // [关系] UseVar: 使用变量 item.0, list
        return Ok((item.0, list));
    }
    // [实体] Function/Method: insert
    // [关系] Define: 定义方法 insert
    // [实体] Variable: key (参数)
    // [实体] Variable: value (参数)
    pub fn insert(&mut self, key: KedisKey, value: Structure) -> Option<Structure> {
        // [关系] Call: 调用 self.remove_expired_key
        // [关系] UseVar: 使用变量 self, key
        self.remove_expired_key(&key);
        // [关系] UseVar: 使用变量 key.ttl
        if key.ttl > -1 {
            // [关系] Call: 调用 self.dict.remove
            // [关系] Modify: 修改 self.dict
            // [关系] UseVar: 使用变量 self.dict, key
            self.dict.remove(&key);
        }
        // [关系] Call: 调用 self.dict.insert
        // [关系] Modify: 修改 self.dict
        // [关系] UseVar: 使用变量 self.dict, key, value
        return self.dict.insert(key, value);
    }

    // [实体] Function/Method: get
    // [关系] Define: 定义方法 get
    // [实体] Variable: key (参数)
    // [实体] Variable: entry (局部变量)
    pub fn get(&mut self, key: &KedisKey) -> Option<&Structure> {
        // [关系] Call: 调用 self.remove_expired_key
        // [关系] UseVar: 使用变量 self, key
        self.remove_expired_key(key);
        // [关系] Call: 调用 self.dict.get
        // [关系] UseVar: 使用变量 self.dict, key
        let entry = self.dict.get(key)?;
        // [关系] Call: 调用 entry.value.as_ref
        // [关系] UseVar: 使用变量 entry.value
        return entry.value.as_ref();
    }

    // [实体] Function/Method: get_mut
    // [关系] Define: 定义方法 get_mut
    // [实体] Variable: key (参数)
    // [实体] Variable: entry (局部变量)
    pub fn get_mut(&mut self, key: &KedisKey) -> Option<&mut Structure> {
        // [关系] Call: 调用 self.remove_expired_key
        // [关系] UseVar: 使用变量 self, key
        self.remove_expired_key(key);
        // [关系] Call: 调用 self.dict.get_mut
        // [关系] UseVar: 使用变量 self.dict, key
        let entry = self.dict.get_mut(key)?;
        // [关系] Call: 调用 entry.value.as_mut
        // [关系] UseVar: 使用变量 entry.value
        return entry.value.as_mut();
    }
    // [实体] Function/Method: get_entry
    // [关系] Define: 定义方法 get_entry
    // [实体] Variable: key (参数)
    // [实体] Variable: entry (局部变量)
    pub fn get_entry(&mut self, key: &KedisKey) -> Option<&DictEntry<KedisKey, Structure>> {
        // [关系] Call: 调用 self.remove_expired_key
        // [关系] UseVar: 使用变量 self, key
        self.remove_expired_key(key);
        // [关系] Call: 调用 self.dict.get
        // [关系] UseVar: 使用变量 self.dict, key
        return self.dict.get(key);
    }

    // [实体] Function/Method: get_mut_entry
    // [关系] Define: 定义方法 get_mut_entry
    // [实体] Variable: key (参数)
    // [实体] Variable: entry (局部变量)
    pub fn get_mut_entry(&mut self, key: &KedisKey) -> Option<&mut DictEntry<KedisKey, Structure>> {
        // [关系] Call: 调用 self.remove_expired_key
        // [关系] UseVar: 使用变量 self, key
        self.remove_expired_key(key);
        // [关系] Call: 调用 self.dict.get_mut
        // [关系] UseVar: 使用变量 self.dict, key
        return self.dict.get_mut(key);
    }

    // [实体] Function/Method: remove
    // [关系] Define: 定义方法 remove
    // [实体] Variable: key (参数)
    pub fn remove(&mut self, key: &KedisKey) -> Option<Structure> {
        // [关系] Call: 调用 self.dict.remove
        // [关系] Modify: 修改 self.dict
        // [关系] UseVar: 使用变量 self.dict, key
        return self.dict.remove(key);
    }

    // [实体] Function/Method: remove_expired_key
    // [关系] Define: 定义方法 remove_expired_key
    // [实体] Variable: key (参数)
    // [实体] Variable: entry (局部变量)
    // [实体] Variable: entry (if let 变量)
    fn remove_expired_key(&mut self, key: &KedisKey) -> bool {
        // [关系] Call: 调用 self.dict.get
        // [关系] UseVar: 使用变量 self.dict, key
        let entry = self.dict.get(key);
        // [关系] UseVar: 使用变量 entry
        if let Some(entry) = entry {
            // [关系] Call: 调用 Self::check_expired, entry.key.ttl
            // [关系] UseVar: 使用变量 entry.key.ttl
            if Self::check_expired(entry.key.ttl) {
                // [关系] Call: 调用 self.dict.remove
                // [关系] Modify: 修改 self.dict
                // [关系] UseVar: 使用变量 self.dict, key
                self.dict.remove(key);
                return true;
            }
        }
        return false;
    }

    // [实体] Function/Method: check_expired
    // [关系] Define: 定义方法 check_expired
    // [实体] Variable: ttl (参数)
    fn check_expired(ttl: i128) -> bool {
        // [关系] Call: 调用 DateUtil::get_now_date_time_as_millis
        // [关系] UseVar: 使用变量 ttl
        if ttl >= 0 && ttl <= DateUtil::get_now_date_time_as_millis() {
            return true;
        }
        return false;
    }
}

// [关系] Impl: 为 Structure 实现方法
impl Structure {
    // [实体] Function/Method: get_type
    // [关系] Define: 定义方法 get_type
    pub fn get_type(&self) -> &str {
        // [关系] UseVar: 使用变量 self
        return match self {
            // [关系] UseVar: 使用变量 Structure::String
            Structure::String(_) => "string",
            // [关系] UseVar: 使用变量 Structure::Hash
            Structure::Hash => "hash",
            // [关系] UseVar: 使用变量 Structure::List
            Structure::List => "list",
            // [关系] UseVar: 使用变量 Structure::Set
            Structure::Set => "set",
            // [关系] UseVar: 使用变量 Structure::SortSet
            Structure::SortSet => "zset",
        };
    }
}

// [关系] Impl: 为 KedisKey 实现方法
impl KedisKey {
    // [实体] Function/Method: get_expired_by_seconds
    // [关系] Define: 定义方法 get_expired_by_seconds
    // [实体] Variable: ttl (局部变量)
    pub fn get_expired_by_seconds(&self) -> String {
        // [关系] Call: 调用 DateUtil::get_now_date_time_as_millis
        // [关系] Modify: 修改 ttl
        // [关系] UseVar: 使用变量 self.ttl
        let mut ttl = self.ttl - DateUtil::get_now_date_time_as_millis();
        // [关系] UseVar: 使用变量 ttl
        if ttl <= 0 {
            // [关系] Modify: 修改 ttl
            // [关系] UseVar: 使用变量 ttl
            ttl = -1;
        } else {
            // [关系] Modify: 修改 ttl
            // [关系] UseVar: 使用变量 ttl
            ttl /= 1000;
        }
        // [关系] Call: 调用 ttl.to_string
        // [关系] UseVar: 使用变量 ttl
        return ttl.to_string();
    }
}
