use crate::aitk::utils::asynchronous::spawn;
use crate::utils::scraping::*;
use makepad_widgets::defer_with_redraw::DeferWithRedraw;
use makepad_widgets::*;
use url::Url;

script_mod! {
    use mod.prelude.widgets.*

    mod.widgets.CitationBase = #(Citation::register_widget(vm))
    mod.widgets.Citation = set_type_default() do mod.widgets.CitationBase {
        flow: Down
        height: Fit
        cursor: MouseCursor.Hand
        width: 170
        padding: 6
        spacing: 5
        draw_bg +: {
            color: #xf2f2f2
            border_radius: 3
        }

        View {
            height: Fit
            align: Align { y: 0.5 }
            icon := Image {
                width: 16
                height: 16
                src: crate_resource("self://resources/link.png")
            }

            site := Label {
                draw_text +: {
                    text_style: theme.font_bold { font_size: 9 }
                    color: #555
                }
            }
        }

        title := Label {
            draw_text +: {
                text_style +: { font_size: 9 }
                color: #000
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
pub enum CitationAction {
    Open(String),
    #[default]
    None,
}

#[derive(Script, ScriptHook, Widget)]
pub struct Citation {
    #[source]
    source: ScriptObjectRef,

    #[deref]
    deref: View,

    #[rust]
    url: Option<String>,
}

impl Widget for Citation {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref.draw_walk(cx, scope, walk)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.ui_runner().handle(cx, event, scope, self);
        self.deref.handle_event(cx, event, scope);

        if let Hit::FingerUp(fu) = event.hits(cx, self.area()) {
            if fu.was_tap() {
                if let Some(url) = &self.url {
                    cx.widget_action(self.widget_uid(), CitationAction::Open(url.clone()));
                }
            }
        }
    }
}

impl Citation {
    pub fn set_url_once(&mut self, cx: &mut Cx, url: String) {
        if self.url.is_some() {
            return;
        }

        self.set_url(cx, url);
    }

    fn set_url(&mut self, cx: &mut Cx, url: String) {
        self.url = Some(url);
        self.set_initial_info(cx);

        let Ok(()) = self.try_refine_info(cx) else {
            return;
        };

        self.try_fetch_info(cx);
    }

    fn set_initial_info(&mut self, cx: &mut Cx) {
        let site = self.label(cx, ids!(site));
        let title = self.label(cx, ids!(title));
        let url = self.url.as_deref().unwrap();

        site.set_text(cx, url);
        title.set_text(cx, url);
    }

    fn try_refine_info(&mut self, cx: &mut Cx) -> Result<(), ()> {
        let site = self.label(cx, ids!(site));
        let title = self.label(cx, ids!(title));
        let url = self.url.as_deref().unwrap();

        let url = Url::parse(url).map_err(|_| ())?;
        let host = url.host_str().ok_or(())?;
        let path = url.path();

        site.set_text(cx, host);
        title.set_text(cx, path);
        Ok(())
    }

    fn try_fetch_info(&mut self, _cx: &mut Cx) {
        let url = self.url.clone().unwrap();
        let ui = self.ui_runner();
        spawn(async move {
            let Ok(document) = fetch_html(&url).await else {
                return;
            };

            if let Some(title) = extract_title(&document) {
                ui.defer_with_redraw(move |me: &mut Citation, cx, _| {
                    me.label(cx, ids!(title)).set_text(cx, &title);
                });
            }
        });
    }
}
