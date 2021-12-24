use std::collections::HashMap;

// 数据库与java 类型转化
pub fn get_java_map() -> HashMap<String, String> {
    let res: HashMap<String, String> = [
        ("bigint".to_string(), "String".to_string()),
        ("char".to_string(), "String".to_string()),
        ("date".to_string(), "Timestamp".to_string()),
        ("datetime".to_string(), "Timestamp".to_string()),
        ("decimal".to_string(), "Float".to_string()),
        ("double".to_string(), "Float".to_string()),
        ("float".to_string(), "Float".to_string()),
        ("int".to_string(), "Integer".to_string()),
        ("integer".to_string(), "Integer".to_string()),
        ("longtext".to_string(), "String".to_string()),
        ("mediumint".to_string(), "Integer".to_string()),
        ("mediumtext".to_string(), "String".to_string()),
        ("numeric".to_string(), "Float".to_string()),
        ("smallint".to_string(), "Integer".to_string()),
        ("text".to_string(), "String".to_string()),
        ("time".to_string(), "Timestamp".to_string()),
        ("timestamp".to_string(), "Timestamp".to_string()),
        ("tinyint".to_string(), "Integer".to_string()),
        ("tinytext".to_string(), "String".to_string()),
        ("varchar".to_string(), "String".to_string()),
        ].iter().cloned().collect();
    res
}

// 得到需要引入java库的类型map
pub fn get_need_lib_type_map() -> HashMap<String, String> {
    let res: HashMap<String, String> = [
        ("Timestamp".to_string(), "java.sql.Timestamp".to_string())
        ].iter().cloned().collect();
    res
}