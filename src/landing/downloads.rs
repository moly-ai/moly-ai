use crate::data::store::Store;
use makepad_widgets::*;
use moly_protocol::data::PendingDownloadsStatus;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let ICON_COLLAPSE = crate_resource("self://resources/icons/collapse.svg")

    let CollapseButton = MolyButton {
        width: Fit
        height: Fit
        draw_icon +: {
            svg: (ICON_COLLAPSE)
            color: #667085
        }
        icon_walk +: {width: 18 height: Fit}
    }

    let Header = View {
        width: Fill
        height: Fit
        spacing: 25
        padding: Inset {right: 43}

        Label {
            margin: Inset {right: 20.0}
            draw_text +: {
                text_style: theme.font_bold {font_size: 9}
                color: #000
            }
            text: "Model Downloads"
        }

        downloading_count := Label {
            draw_text +: {
                text_style: theme.font_regular {font_size: 9}
                color: #099250
            }
            text: "1 downloading"
        }

        paused_count := Label {
            draw_text +: {
                text_style: theme.font_regular {font_size: 9}
                color: #667085
            }
            text: "1 paused"
        }

        failed_count := Label {
            draw_text +: {
                text_style: theme.font_regular {font_size: 9}
                color: #B42318
            }
            text: "1 failed"
        }

        VerticalFiller {}

        collapse := CollapseButton {
            draw_icon +: { rotation_angle: 180.0 }
        }
    }

    let Content = View {
        width: Fill
        height: 350

        list := PortalList {
            width: Fill
            height: Fill

            DownloadItem := mod.widgets.DownloadItem {}
        }
    }

    mod.widgets.DownloadsBase = #(Downloads::register_widget(vm))
    mod.widgets.Downloads = set_type_default() do mod.widgets.DownloadsBase {
        ..mod.widgets.SolidView
        width: Fill
        height: Fit
        flow: Down

        show_bg: true
        draw_bg +: {
            color: #FCFCFD
        }

        // TODO there is a better way to have only top-border?
        Line { draw_bg +: { color: #EAECF0 }}
        Header {
            padding: Inset {top: 20.0 bottom: 20.0 left: 43.0}
        }
        content := Content {
            height: 0
            padding: Inset {top: 12.0 bottom: 12.0 left: 43.0 right: 43.0}
        }

        content_animation_progress: 0.0
        animator: Animator {
            content: {
                default: @collapse
                expand: AnimatorState {
                    redraw: true
                    from: {all: Forward {duration: 0.3}}
                    ease: ExpDecay {d1: 0.80 d2: 0.97}
                    apply: {content_animation_progress: 1.0}
                }
                collapse: AnimatorState {
                    redraw: true
                    from: {all: Forward {duration: 0.3}}
                    ease: ExpDecay {d1: 0.80 d2: 0.97}
                    apply: {content_animation_progress: 0.0}
                }
            }
        }
    }
}

#[derive(Animator, Script, ScriptHook, Widget)]
pub struct Downloads {
    #[source]
    source: ScriptObjectRef,

    #[deref]
    view: View,

    #[live]
    content_animation_progress: f64,

    #[apply_default]
    animator: Animator,
}

impl Widget for Downloads {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);

        if self.animator_handle_event(cx, event).must_redraw() {
            let height = self.content_animation_progress * 350.0;
            let mut content = self.view(cx, ids!(content));
            script_apply_eval!(cx, content, { height: #(height) });
            self.redraw(cx);
        }

        match event.hits(cx, self.view.area()) {
            Hit::FingerUp(fe) => {
                if fe.was_tap() {
                    self.toggle_collapse(cx);
                }
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let store = scope.data.get::<Store>().unwrap();
        let pending_downloads = &store.downloads.pending_downloads;

        let downloads_count = pending_downloads.len();

        let download_count = pending_downloads
            .iter()
            .filter(|d| {
                matches!(
                    d.status,
                    PendingDownloadsStatus::Downloading | PendingDownloadsStatus::Initializing
                )
            })
            .count();
        self.label(cx, ids!(downloading_count))
            .set_text(cx, &format!("{} downloading", download_count));

        let paused_count = pending_downloads
            .iter()
            .filter(|d| matches!(d.status, PendingDownloadsStatus::Paused))
            .count();
        self.label(cx, ids!(paused_count))
            .set_text(cx, &format!("{} paused", paused_count));

        let failed_count = pending_downloads
            .iter()
            .filter(|d| matches!(d.status, PendingDownloadsStatus::Error))
            .count();

        if failed_count > 0 {
            self.label(cx, ids!(failed_count))
                .set_text(cx, &format!("{} failed", failed_count));
        } else {
            self.label(cx, ids!(failed_count)).set_text(cx, "");
        }

        while let Some(view_item) = self.view.draw_walk(cx, &mut Scope::empty(), walk).step() {
            if let Some(mut list) = view_item.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, downloads_count);
                while let Some(item_id) = list.next_visible_item(cx) {
                    let item = list.item(cx, item_id, id!(DownloadItem));

                    if item_id < downloads_count {
                        let download = &pending_downloads[item_id];
                        item.draw_all(cx, &mut Scope::with_data(&mut download.clone()));
                    }
                }
            }
        }

        DrawStep::done()
    }
}

impl WidgetMatchEvent for Downloads {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        if self.button(cx, ids!(collapse)).clicked(&actions) {
            self.toggle_collapse(cx);
        }
    }
}

impl Downloads {
    fn toggle_collapse(&mut self, cx: &mut Cx) {
        if self.animator.in_state(cx, ids!(content.collapse)) {
            self.animator_play(cx, ids!(content.expand));
            self.set_collapse_button_open(cx, true)
        } else {
            self.animator_play(cx, ids!(content.collapse));
            self.set_collapse_button_open(cx, false)
        }
    }

    fn set_collapse_button_open(&mut self, cx: &mut Cx, is_open: bool) {
        let rotation_angle = if is_open { 0.0 } else { 180.0 };
        let mut collapse_btn = self.button(cx, ids!(collapse));
        script_apply_eval!(cx, collapse_btn, {
            draw_icon +: { rotation_angle: #(rotation_angle) }
        });
    }
}
