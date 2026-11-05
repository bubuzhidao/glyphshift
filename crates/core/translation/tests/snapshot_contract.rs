use glyphshift_domain::Generation;
use glyphshift_translation::TranslationSnapshot;

#[test]
fn bulk_location_entries_match_sequential_entry_updates() {
    let seed = TranslationSnapshot::empty(Generation::new(7)).with_entry_for_adapters(
        "menu",
        "Open",
        "旧值",
        ["adapter.synthetic"],
    );
    let sequential = seed
        .clone()
        .with_entry("menu", "Open", "打开")
        .with_entry("menu", "Close", "关闭");
    let bulk = seed.with_entries_at_location(
        "menu",
        [("Open", "打开"), ("Close", "关闭")],
    );

    assert_eq!(bulk, sequential);
}

#[test]
fn adapter_batch_entries_match_sequential_entry_updates() {
    let seed = TranslationSnapshot::empty(Generation::new(9));
    let sequential = seed
        .clone()
        .with_entry_for_adapters("menu", "Open", "打开", ["adapter.a"])
        .with_entry_for_adapters("dialog", "Close", "关闭", ["adapter.b"]);
    let batch = seed.with_entries_for_adapters_batch([
        ("menu", "Open", "打开", vec!["adapter.a"]),
        ("dialog", "Close", "关闭", vec!["adapter.b"]),
    ]);

    assert_eq!(batch, sequential);
}

#[test]
fn contextual_adapter_batch_entries_match_sequential_entry_updates() {
    let seed = TranslationSnapshot::empty(Generation::new(11));
    let sequential = seed
        .clone()
        .with_context_entry_for_adapters(
            "menu",
            "section",
            "Tools",
            "Open",
            "打开",
            ["adapter.a"],
        )
        .with_context_entry_for_adapters(
            "dialog",
            "section",
            "File",
            "Close",
            "关闭",
            ["adapter.b"],
        );
    let batch = seed.with_context_entries_for_adapters_batch([
        (
            "menu",
            "section",
            "Tools",
            "Open",
            "打开",
            vec!["adapter.a"],
        ),
        (
            "dialog",
            "section",
            "File",
            "Close",
            "关闭",
            vec!["adapter.b"],
        ),
    ]);

    assert_eq!(batch, sequential);
}
