use crate::ui_toast_system::{
    UI_TOAST_HEIGHT, UI_TOAST_SLIDE_IN_TIME, UI_TOAST_SLIDE_OUT_TIME, UI_TOAST_STAY_TIME,
    UI_TOAST_TOP_PIXELS, UIToastLifecycleAction, ui_toast_lifecycle_action, ui_toast_top_for_age,
};

#[test]
fn ui_toast_keeps_current_until_stay_time_finishes() {
    let age_before_stay_finishes = UI_TOAST_SLIDE_IN_TIME + UI_TOAST_STAY_TIME - 0.01;

    assert_eq!(
        ui_toast_lifecycle_action(
            age_before_stay_finishes,
            UI_TOAST_SLIDE_IN_TIME,
            UI_TOAST_STAY_TIME,
            UI_TOAST_SLIDE_OUT_TIME,
            true,
        ),
        UIToastLifecycleAction::KeepCurrent
    );
}

#[test]
fn ui_toast_replaces_current_in_place_when_queue_is_waiting() {
    let age_after_stay_finishes = UI_TOAST_SLIDE_IN_TIME + UI_TOAST_STAY_TIME;

    assert_eq!(
        ui_toast_lifecycle_action(
            age_after_stay_finishes,
            UI_TOAST_SLIDE_IN_TIME,
            UI_TOAST_STAY_TIME,
            UI_TOAST_SLIDE_OUT_TIME,
            true,
        ),
        UIToastLifecycleAction::ReplaceCurrent
    );
    assert_eq!(
        ui_toast_top_for_age(
            UI_TOAST_SLIDE_IN_TIME,
            UI_TOAST_HEIGHT,
            UI_TOAST_SLIDE_IN_TIME,
            UI_TOAST_STAY_TIME,
            UI_TOAST_SLIDE_OUT_TIME,
        ),
        UI_TOAST_TOP_PIXELS
    );
}

#[test]
fn ui_toast_slides_out_only_when_queue_is_empty() {
    let age_after_lifetime = UI_TOAST_SLIDE_IN_TIME + UI_TOAST_STAY_TIME + UI_TOAST_SLIDE_OUT_TIME;

    assert_eq!(
        ui_toast_lifecycle_action(
            age_after_lifetime,
            UI_TOAST_SLIDE_IN_TIME,
            UI_TOAST_STAY_TIME,
            UI_TOAST_SLIDE_OUT_TIME,
            false,
        ),
        UIToastLifecycleAction::DespawnCurrent
    );
}
