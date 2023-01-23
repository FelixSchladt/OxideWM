use crate::*;

pub fn run_and_exit() {
    empty_test();

    //Config Loading
    load_config_from_file();
    load_config_from_repository_file();
    load_config_from_wrong_datatype_file();

    println!("Test runs finished.");
    std::process::exit(0);
}

#[test]
pub fn empty_test() {
    assert_eq!(true, true);
}


#[test]
pub fn load_config_from_file() {
    let mock_file_path = &format!("{}/test_files/config.yml", std::env::var("PWD").unwrap());
    let cfg = Config::new(Some(mock_file_path));

    assert_eq!(cfg.cmds.len(), 1);
    assert_eq!(cfg.exec.len(), 1);
    assert_eq!(cfg.exec_always.len(), 0);
    assert_eq!(cfg.cmds[0].keys.len(), 2);
    assert_eq!(cfg.cmds[0].keys[0], "A".to_string());
    assert_eq!(cfg.cmds[0].keys[1], "t".to_string());
    assert_eq!(cfg.cmds[0].args, Some("kitty".to_string()));
    assert_eq!(cfg.exec[0], "./target/debug/oxide-bar".to_string());
    assert_eq!(cfg.border_width, 8);
    assert_eq!(cfg.border_color, "0x008000");
    assert_eq!(cfg.border_focus_color, "0xFFFF00");
    assert_eq!(cfg.gap, 8);
}

#[test]
pub fn load_config_from_repository_file() {
    let cfg = Config::new(None);

    assert_eq!(cfg.cmds.len(), 15);
    assert_eq!(cfg.exec.len(), 1);
    assert_eq!(cfg.exec_always.len(), 0);
    assert_eq!(cfg.cmds[0].keys.len(), 3);
    assert_eq!(cfg.cmds[0].keys[0], "A".to_string());
    assert_eq!(cfg.cmds[0].keys[1], "S".to_string());
    assert_eq!(cfg.cmds[0].keys[2], "e".to_string());
    assert_eq!(cfg.cmds[0].args, None);
    assert_eq!(cfg.exec[0], "./target/debug/oxide-bar".to_string());
    assert_eq!(cfg.border_width, 8);
    assert_eq!(cfg.border_color, "0x008000");
    assert_eq!(cfg.border_focus_color, "0xFFFF00");
    assert_eq!(cfg.gap, 8);
}

#[test]
pub fn load_config_from_wrong_datatype_file() {
    let mock_file_path = &format!("{}/test_files/invalid_datatypes.yml", std::env::var("PWD").unwrap());
    let cfg = Config::new(Some(mock_file_path));

    assert_eq!(cfg.cmds.len(), 1);
    assert_eq!(cfg.exec.len(), 0);
    assert_eq!(cfg.exec_always.len(), 0);
    assert_eq!(cfg.cmds[0].keys.len(), 1);
    assert_eq!(cfg.cmds[0].keys[0], "A".to_string());
    assert_eq!(cfg.cmds[0].keys[1], "t".to_string());
    assert_eq!(cfg.cmds[0].args, Some("kitty".to_string()));
    assert_eq!(cfg.border_width, 3);
    assert_eq!(cfg.border_color, "0xFFFFFF");
    assert_eq!(cfg.border_focus_color, "0x000000");
    assert_eq!(cfg.gap, 3);
}
