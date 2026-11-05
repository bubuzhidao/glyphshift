use glyphshift_ai_translation::{
    AiTranslation, FilterPolicy, SkipReason, TranslationItem, TranslationPlanRequest,
};

#[test]
fn planner_selects_only_untranslated_meaningful_text_and_explains_every_skip() {
    let mut translation = AiTranslation::new();
    let request = TranslationPlanRequest::new(
        "dictionary.pending",
        7,
        "en-US",
        "zh-CN",
        [
            TranslationItem::untranslated("help", "&Help"),
            TranslationItem::untranslated("zero", "0"),
            TranslationItem::untranslated("counter", "0/4"),
            TranslationItem::untranslated("size", "1920 x 1080"),
            TranslationItem::untranslated("rate", "25.00fps"),
            TranslationItem::untranslated("letter", "A"),
            TranslationItem::untranslated("save", "Save"),
            TranslationItem::translated("open", "Open", "打开"),
            TranslationItem::untranslated("ignored", "Hidden").ignored(),
        ],
    )
    .with_filter_policy(FilterPolicy::default());

    let plan = translation
        .plan_translation(request)
        .expect("plan translation");

    assert_eq!(plan.snapshot_revision(), 7);
    assert_eq!(
        plan.candidates()
            .iter()
            .map(|candidate| candidate.source())
            .collect::<Vec<_>>(),
        vec!["&Help", "Save"]
    );
    assert_eq!(
        plan.skip_reason("zero"),
        Some(SkipReason::PureNumberOrSymbols)
    );
    assert_eq!(
        plan.skip_reason("counter"),
        Some(SkipReason::NumericMeasurement)
    );
    assert_eq!(
        plan.skip_reason("size"),
        Some(SkipReason::NumericMeasurement)
    );
    assert_eq!(
        plan.skip_reason("rate"),
        Some(SkipReason::NumericMeasurement)
    );
    assert_eq!(
        plan.skip_reason("letter"),
        Some(SkipReason::SingleCharacter)
    );
    assert_eq!(
        plan.skip_reason("open"),
        Some(SkipReason::AlreadyTranslated)
    );
    assert_eq!(plan.skip_reason("ignored"), Some(SkipReason::Ignored));
    assert_eq!((plan.eligible_count(), plan.skipped_count()), (2, 7));
}

#[test]
fn coordinate_readouts_are_hidden_while_meaningful_text_stays() {
    let mut translation = AiTranslation::new();
    let request = TranslationPlanRequest::new(
        "dictionary.pending",
        3,
        "en-US",
        "zh-CN",
        [
            TranslationItem::untranslated(
                "cursor",
                "X:-2357.841mil Y:-1175.928mil  Grid: 1mil",
            ),
            TranslationItem::untranslated("origin", "X:0mil Y:0mil    Grid:100mil"),
            TranslationItem::untranslated(
                "board",
                "Board Size: Horizontal 3944.912mil Vertical 4732.313mil",
            ),
            TranslationItem::untranslated("density", "Density: 38.35%"),
            TranslationItem::untranslated("snap", "Snap Distance 8mil"),
            TranslationItem::untranslated(
                "selected_pad",
                "Pad U501-3(1933.831mil,2316.404mil) on Bottom",
            ),
            TranslationItem::untranslated(
                "arc",
                "Arc (1340.599mil,2336.999mil) on Bottom Overlay",
            ),
            TranslationItem::untranslated(
                "net_radius",
                "Net:  Radius:25.055mil Angle1:225.00 Angle2:315.00   Component C110 Comment:1uF Footprint: C0402",
            ),
            TranslationItem::untranslated(
                "net_size",
                "Net:GND_bis X-Size:26.536mil Y-Size:98.446mil Hole Type:Round Hole:0mil   Component U501 Comment:CA-IS3641LVW Footprint: SOIC-16_L10.3-W7.5-P1.27-LS10.3-BL",
            ),
            TranslationItem::untranslated("status", "1 object is selected"),
            TranslationItem::untranslated(
                "window_title",
                "IMP_anesthesia_MCU.PrjPcb - Altium Designer Enterprise (25.4.2)",
            ),
            TranslationItem::untranslated("save", "Save"),
            TranslationItem::untranslated("version", "Version 2"),
            TranslationItem::untranslated("bodies", "3D Bodies"),
            TranslationItem::untranslated("zoom", "Zoom to 5 mil grid"),
        ],
    )
    .with_filter_policy(FilterPolicy::default());

    let plan = translation
        .plan_translation(request)
        .expect("plan translation");

    let mut candidates = plan
        .candidates()
        .iter()
        .map(|candidate| candidate.source().to_owned())
        .collect::<Vec<_>>();
    candidates.sort();
    assert_eq!(
        candidates,
        vec![
            "1 object is selected",
            "3D Bodies",
            "IMP_anesthesia_MCU.PrjPcb - Altium Designer Enterprise (25.4.2)",
            "Save",
            "Version 2",
            "Zoom to 5 mil grid",
        ]
    );
    for id in [
        "cursor",
        "origin",
        "board",
        "density",
        "snap",
        "selected_pad",
        "arc",
        "net_radius",
        "net_size",
    ] {
        assert_eq!(
            plan.skip_reason(id),
            Some(SkipReason::NumericMeasurement),
            "{id} must stay verbatim"
        );
    }
    assert_eq!((plan.eligible_count(), plan.skipped_count()), (6, 9));
}

#[test]
fn component_values_and_parameter_fields_are_hidden_without_hiding_identifiers() {
    let mut translation = AiTranslation::new();
    let request = TranslationPlanRequest::new(
        "dictionary.pending",
        4,
        "en-US",
        "zh-CN",
        [
            TranslationItem::untranslated("resistor", "4.7kΩ"),
            TranslationItem::untranslated("capacitor", "100nF"),
            TranslationItem::untranslated("inductor", "10uH"),
            TranslationItem::untranslated(
                "footprint",
                "Footprint: SOIC-16_L10.3-W7.5-P1.27-LS10.3-BL",
            ),
            TranslationItem::untranslated("manufacturer", "Manufacturer: Texas Instruments"),
            TranslationItem::untranslated("part", "Manufacturer Part: MSP430F5529IPN"),
            TranslationItem::untranslated("designator", "Designator: R25"),
            TranslationItem::untranslated("net_name", "VCC_5V_IN"),
            TranslationItem::untranslated("standard", "RS-274-X"),
            TranslationItem::untranslated("fuse_designator", "F3"),
            TranslationItem::untranslated("size_label", "Total Size 15MB"),
            TranslationItem::untranslated("comment_label", "Comment:"),
            TranslationItem::untranslated("manufacturer_label", "Manufacturer"),
            TranslationItem::untranslated("save", "Save"),
        ],
    )
    .with_filter_policy(FilterPolicy::default());

    let plan = translation
        .plan_translation(request)
        .expect("plan translation");

    let mut candidates = plan
        .candidates()
        .iter()
        .map(|candidate| candidate.source().to_owned())
        .collect::<Vec<_>>();
    candidates.sort();
    assert_eq!(
        candidates,
        vec![
            "Comment:",
            "F3",
            "Manufacturer",
            "RS-274-X",
            "Save",
            "Total Size 15MB",
            "VCC_5V_IN",
        ]
    );
    for id in [
        "resistor",
        "capacitor",
        "inductor",
        "footprint",
        "manufacturer",
        "part",
        "designator",
    ] {
        assert_eq!(
            plan.skip_reason(id),
            Some(SkipReason::NumericMeasurement),
            "{id} must stay verbatim"
        );
    }
    assert_eq!((plan.eligible_count(), plan.skipped_count()), (7, 7));
}

#[test]
fn containing_digits_is_an_opt_in_filter_not_a_hidden_default() {
    let mut translation = AiTranslation::new();
    let items = [TranslationItem::untranslated("version", "Version 2")];

    let default_plan = translation
        .plan_translation(TranslationPlanRequest::new(
            "dictionary.pending",
            1,
            "en-US",
            "zh-CN",
            items.clone(),
        ))
        .expect("default plan");
    let strict_plan = translation
        .plan_translation(
            TranslationPlanRequest::new("dictionary.pending", 1, "en-US", "zh-CN", items)
                .with_filter_policy(FilterPolicy::default().with_skip_text_containing_digits(true)),
        )
        .expect("strict plan");

    assert_eq!(default_plan.eligible_count(), 1);
    assert_eq!(
        strict_plan.skip_reason("version"),
        Some(SkipReason::ContainsDigit)
    );
}

#[test]
fn planner_filters_common_non_translatable_formats_and_protects_format_tokens() {
    let mut translation = AiTranslation::new();
    let policy = FilterPolicy::default()
        .with_max_source_chars(Some(24))
        .with_excluded_patterns([r"(?i)^debug_"]);
    let plan = translation
        .plan_translation(
            TranslationPlanRequest::new(
                "probe.synthetic",
                9,
                "en-US",
                "zh-CN",
                [
                    TranslationItem::untranslated("url", "https://example.invalid/guide"),
                    TranslationItem::untranslated("email", "person@example.invalid"),
                    TranslationItem::untranslated("path", "assets/ui/file.json"),
                    TranslationItem::untranslated("shortcut", "Ctrl+Shift+S"),
                    TranslationItem::untranslated("long", "This source is longer than allowed"),
                    TranslationItem::untranslated("custom", "DEBUG_INTERNAL"),
                    TranslationItem::untranslated("first-save", "Save"),
                    TranslationItem::untranslated("duplicate-save", "Save"),
                    TranslationItem::untranslated("formatted", "File {count} of %1"),
                ],
            )
            .with_filter_policy(policy),
        )
        .expect("plan filtered translation");

    assert_eq!(plan.skip_reason("url"), Some(SkipReason::Url));
    assert_eq!(plan.skip_reason("email"), Some(SkipReason::Email));
    assert_eq!(plan.skip_reason("path"), Some(SkipReason::FilePath));
    assert_eq!(plan.skip_reason("shortcut"), Some(SkipReason::Shortcut));
    assert_eq!(plan.skip_reason("long"), Some(SkipReason::TooLong));
    assert_eq!(plan.skip_reason("custom"), Some(SkipReason::CustomPattern));
    assert_eq!(
        plan.skip_reason("duplicate-save"),
        Some(SkipReason::DuplicateSource)
    );
    assert_eq!(plan.skip_reason("formatted"), None);
    let formatted = plan
        .candidates()
        .iter()
        .find(|candidate| candidate.item_id() == "formatted")
        .expect("formatted candidate");
    assert_eq!(
        formatted
            .protected_tokens()
            .iter()
            .map(Box::as_ref)
            .collect::<Vec<_>>(),
        vec!["{count}", "%1"]
    );
}
