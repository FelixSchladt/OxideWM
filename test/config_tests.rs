use crate::*;

pub fn get_file_path(filename: &str) -> String {
    format!(
        "{}/test/test_files/{}",
        std::env::var("PWD").unwrap(),
        filename
    )
}

#[test]
pub fn load_config_from_file() {
    let cfg = Config::new(Some(&get_file_path("config.yml")));

    assert_eq!(cfg.cmds.len(), 1);
    assert_eq!(cfg.exec.len(), 1);
    assert_eq!(cfg.exec_always.len(), 0);
    assert_eq!(cfg.cmds[0].keys.len(), 2);
    assert_eq!(cfg.cmds[0].keys[0], "A".to_string());
    assert_eq!(cfg.cmds[0].keys[1], "t".to_string());
    assert_eq!(cfg.cmds[0].args, Some("kitty".to_string()));
    assert_eq!(cfg.exec[0], "./target/debug/oxide-bar".to_string());
    assert_eq!(cfg.border_color, "0x008000");
    assert_eq!(cfg.border_focus_color, "0xFFFF00");
    assert_eq!(cfg.gap, 8);
}

#[test]
pub fn load_config_from_wrong_datatype_file() {
    let cfg = Config::new(Some(&get_file_path("invalid_datatypes.yml")));

    assert_eq!(cfg.cmds.len(), 1);
    assert_eq!(cfg.exec.len(), 0);
    assert_eq!(cfg.exec_always.len(), 0);
    assert_eq!(cfg.cmds[0].keys.len(), 1);
    assert_eq!(cfg.cmds[0].keys[0], "A".to_string());
    assert_eq!(cfg.cmds[0].keys[1], "t".to_string());
    assert_eq!(cfg.border_width, 3);
    assert_eq!(cfg.border_color, "0xFFFFFF");
    assert_eq!(cfg.border_focus_color, "0x000000");
    assert_eq!(cfg.gap, 3);
}

#[test]
pub fn load_config_with_missing_values() {
    let cfg = Config::new(Some(&get_file_path("missing_values.yml")));

    assert_eq!(cfg.cmds.len(), 1);
    assert_eq!(cfg.exec.len(), 0);
    assert_eq!(cfg.exec_always.len(), 0);
    assert_eq!(cfg.cmds[0].keys.len(), 2);
    assert_eq!(cfg.cmds[0].keys[0], "A".to_string());
    assert_eq!(cfg.cmds[0].keys[1], "t".to_string());
    assert_eq!(cfg.border_width, 8);
    assert_eq!(cfg.border_color, "0x008000");
    assert_eq!(cfg.border_focus_color, "0x000000");
    assert_eq!(cfg.gap, 3);
}
