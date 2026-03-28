use gpui::{PathPromptOptions, PromptLevel, TestAppContext};

#[test]
fn test_prompt_level_variants_exist() {
    let _info = PromptLevel::Info;
    let _warning = PromptLevel::Warning;
    let _critical = PromptLevel::Critical;
}

#[test]
fn test_path_prompt_options_construction() {
    let opts = PathPromptOptions {
        files: true,
        directories: true,
        multiple: true,
        prompt: Some("Select files".into()),
    };
    assert!(opts.files);
    assert!(opts.directories);
    assert!(opts.multiple);
}

#[test]
fn test_path_prompt_options_defaults() {
    let opts = PathPromptOptions {
        files: true,
        directories: false,
        multiple: false,
        prompt: None,
    };
    assert!(opts.files);
    assert!(!opts.directories);
    assert!(!opts.multiple);
    assert!(opts.prompt.is_none());
}

// Note: prompt_for_paths() and prompt_for_new_path() are not implemented
// in the GPUI test platform — calling them panics with "not implemented".
// These APIs open native OS dialogs and can only be tested manually.
