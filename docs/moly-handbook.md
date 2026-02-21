# Moly Handbook

> AI Context Document for Moly App Migration

This document provides a comprehensive overview of Moly's architecture, patterns, and codebase structure. It is designed as context for AI agents performing the migration from Makepad's old `live_design!` DSL to the new Splash `script_mod!` system.

For the migration patterns themselves, see [splash-migration-guide.md](./splash-migration-guide.md). For the Splash language reference, see [splash-language-reference.md](./splash-language-reference.md).

---

## Table of Contents

1. [What is Moly](#1-what-is-moly)
2. [Repository Structure](#2-repository-structure)
3. [Dependency Graph](#3-dependency-graph)
4. [App Entry Point and Startup Flow](#4-app-entry-point-and-startup-flow)
5. [The Store Pattern](#5-the-store-pattern)
6. [Module Registration Chain](#6-module-registration-chain)
7. [Module Map: `src/shared/`](#7-module-map-srcshared)
8. [Module Map: `src/chat/`](#8-module-map-srcchat)
9. [Module Map: `src/landing/`](#9-module-map-srclanding)
10. [Module Map: `src/settings/`](#10-module-map-srcsettings)
11. [Module Map: `src/mcp/`](#11-module-map-srcmcp)
12. [Module Map: `src/my_models/`](#12-module-map-srcmy_models)
13. [Module Map: `src/data/`](#13-module-map-srcdata)
14. [MolyKit Architecture](#14-molykit-architecture)
15. [MolyKit Widget Inventory](#15-molykit-widget-inventory)
16. [Key Patterns in Moly](#16-key-patterns-in-moly)
17. [Key Patterns in MolyKit](#17-key-patterns-in-molykit)
18. [Custom Shaders](#18-custom-shaders)
19. [Action System](#19-action-system)
20. [Async Patterns](#20-async-patterns)
21. [Widget Inventory Summary](#21-widget-inventory-summary)
22. [Migration Risk Assessment](#22-migration-risk-assessment)
23. [File-by-File Migration Order](#23-file-by-file-migration-order)

---

## 1. What is Moly

Moly is a cross-platform desktop/web/mobile AI chat application built with the Makepad UI framework. It allows users to:

- Chat with multiple AI providers (OpenAI-compatible, MolyServer local models, DeepInquire, OpenClaw, MoFa, OpenAI Realtime)
- Browse and download local LLM models via MolyServer
- Configure providers, API keys, and MCP tool servers
- Manage multiple concurrent chats with LRU eviction
- Use speech-to-text input
- View file attachments and citations in chat

The app is split into three main crates:

- **moly** — The application binary with all screens and business logic
- **moly-kit** — A reusable AI chat widget library for Makepad applications
- **moly-sync** — Sync utilities (minor crate)

---

## 2. Repository Structure

```
moly/
├── src/
│   ├── main.rs              # Entry point: Tokio runtime setup, calls app_main()
│   ├── lib.rs               # Module declarations
│   ├── app.rs               # App struct, MolyRoot, navigation, event dispatch
│   ├── runtime.rs           # Native-only runtime utilities
│   ├── capture.rs           # Context capture (clipboard-like)
│   ├── shared/              # Shared widgets, styles, utilities (11 files with live_design!)
│   ├── chat/                # Chat feature (14 files with live_design!)
│   ├── landing/             # Model discovery/download (12 files with live_design!)
│   ├── settings/            # Provider settings (7 files with live_design!)
│   ├── mcp/                 # MCP server config (2 files with live_design!)
│   ├── my_models/           # Downloaded model management (5 files with live_design!)
│   └── data/                # Data layer: Store, Chats, Downloads, Search, Providers (no live_design!)
├── moly-kit/
│   ├── src/
│   │   ├── lib.rs           # Crate root: exports utils, widgets, prelude, aitk
│   │   ├── prelude.rs       # Re-exports widget Rust types + aitk prelude
│   │   ├── widgets.rs       # Widget registration chain, cx.link() for theme
│   │   ├── widgets/         # 23 widget files (23 live_design! blocks)
│   │   └── utils/           # Audio, scraping, makepad utilities
│   └── examples/
│       └── moly-mini/       # Minimal example app using MolyKit
├── moly-sync/               # Sync utilities
├── resources/               # Icons, images, fonts
│   ├── icons/               # SVG icons
│   └── images/              # PNG images (provider logos, etc.)
├── Cargo.toml               # Workspace + app manifest
└── docs/                    # Migration documentation (this file, etc.)
```

---

## 3. Dependency Graph

```
moly (app binary)
├── makepad-widgets          (git: wyeworks/makepad)
├── makepad-code-editor      (git: wyeworks/makepad)
├── moly-kit                 (path: ./moly-kit)
│   ├── makepad-widgets
│   ├── makepad-code-editor
│   ├── aitk                 (AI toolkit: protocol, clients, spawn)
│   │   ├── aitk-protocol    (Bot, BotId, Message, ChatState, etc.)
│   │   ├── aitk-client      (OpenAiClient, RouterClient, MapClient, etc.)
│   │   └── aitk-utils       (spawn, async helpers)
│   └── math_widget          (LaTeX math rendering widget)
├── moly-protocol            (git: moly-ai/moly-local — Model, File, FileId, etc.)
├── moly-sync                (path: ./moly-sync)
├── tokio                    (native async runtime)
├── reqwest                  (HTTP client)
├── serde / serde_json       (serialization)
├── chrono                   (date/time)
├── futures                  (async primitives — preferred over tokio channels)
├── robius-open              (URL opening)
├── robius-url-handler       (deep link handling)
└── uuid                     (unique IDs)
```

Key points:
- **aitk** is the AI toolkit, re-exported by moly-kit as `moly_kit::aitk`
- **aitk::prelude** is re-exported as `moly_kit::prelude::*` (Bot, BotId, Message, ChatController, etc.)
- **moly-protocol** provides MolyServer-specific types (Model, File, PendingDownload, etc.)

---

## 4. App Entry Point and Startup Flow

### `src/main.rs`

**Native** (non-wasm):
1. Sets working directory to executable's parent (critical for macOS bundles)
2. Registers URL handler for deep links
3. Creates Tokio multi-thread runtime with IO + time enabled
4. Calls `moly::app::app_main()` inside `block_on`

**WASM**:
1. Initializes logger without timestamps
2. Calls `moly::app::app_main()` directly

### `src/app.rs`

The `App` struct is the top-level Makepad application:

```rust
#[derive(Live, LiveHook)]
pub struct App {
    #[live] pub ui: WidgetRef,
    #[rust] pub store: Option<Store>,
    #[rust] timer: Timer,
    #[rust] download_retry_attempts: usize,
    #[rust] file_id: Option<FileId>,
}
```

**Startup sequence**:
1. `app_main!(App)` registers the app
2. `LiveRegister::live_register(cx)` registers all DSL modules (see section 6)
3. On `Event::Startup`: hides the body, calls `Store::load_into_app()` (async)
4. `Store::load_into_app()` runs on a spawned task:
   - Loads preferences from disk
   - Creates MolyClient
   - Loads chats from disk
   - Initializes current chat, syncs with MolyServer, loads provider connections
   - Uses `app_runner().defer()` to inject the Store back into the App on the UI thread
5. Once Store is loaded, `MolyRoot` widget allows rendering; loading screen hides

### `MolyRoot` widget

A wrapper that gates `draw_walk` and `handle_event` on `scope.data.get::<Store>().is_some()`. This prevents the UI from rendering before the Store is loaded.

```rust
#[derive(Live, Widget, LiveHook)]
pub struct MolyRoot {
    #[deref] view: View,
}
```

### `UiRunner` pattern

`UiRunner<T>` is a Makepad utility for bridging async code back to the UI thread. Moly exposes a global runner via `app_runner()`:

```rust
pub fn app_runner() -> UiRunner<App> {
    UiRunner::new(0) // 0 is reserved for AppMain implementor
}
```

Usage: `app_runner().defer(|app, cx, _| { ... })` — runs closure on UI thread with access to App.

Widget-level runners: `self.ui_runner()` returns `UiRunner<Self>`, used by `ChatScreen`, `ChatView`, etc.

---

## 5. The Store Pattern

`Store` is a plain Rust struct (not a Makepad Live type) that holds all application state. It is passed through the widget tree via `Scope::with_data()`.

```rust
pub struct Store {
    pub search: Search,
    pub downloads: Downloads,
    pub chats: Chats,
    pub preferences: Preferences,
    pub bot_context: Option<BotContext>,
    moly_client: MolyClient,
    pub provider_syncing_status: ProviderSyncingStatus,
    pub provider_icons: Vec<LiveDependency>,
}
```

**How it flows**:
1. `App.store: Option<Store>` — owned by the App
2. `App::handle_event` creates `Scope::with_data(store)` and passes it to `self.ui.handle_event(cx, event, scope)`
3. Widgets access it via `scope.data.get::<Store>()` or `scope.data.get_mut::<Store>()`

**Key subsystems**:
- `Search` — model search/browse, featured models, sorting
- `Downloads` — pending/completed downloads, download file operations
- `Chats` — chat sessions, provider registry, available bots
- `Preferences` — persisted settings (providers, MCP config, STT config)
- `BotContext` — wrapper around BotClient + loaded bots + MCP tool manager
- `MolyClient` — HTTP client for MolyServer local API

---

## 6. Module Registration Chain

In `App::live_register(cx)`:

```rust
fn live_register(cx: &mut Cx) {
    makepad_widgets::live_design(cx);      // Makepad built-in widgets
    moly_kit::widgets::live_design(cx);    // MolyKit widgets + theme

    crate::shared::live_design(cx);        // Shared styles, widgets, utils
    crate::landing::live_design(cx);       // Landing/discover screens
    crate::chat::live_design(cx);          // Chat screens
    crate::my_models::live_design(cx);     // My models screen
    crate::settings::live_design(cx);      // Settings screens
    crate::mcp::live_design(cx);           // MCP screen
}
```

Each module's `live_design(cx)` function calls `live_design(cx)` on its sub-modules in dependency order. For example, `shared::live_design(cx)` registers:
`meta` → `list` → `styles` → `resource_imports` → `widgets` → `popup_notification` → `external_link` → `download_notification_popup` → `tooltip` → `desktop_buttons` → `moly_server_popup`

**Migration note**: All these `live_design(cx)` calls become `script_mod(vm)` calls, and `LiveRegister` becomes `ScriptRegister`.

---

## 7. Module Map: `src/shared/`

Core shared components and styles used across all screens.

| File | Widget/Content | `live_design!` | Notes |
|------|---------------|----------------|-------|
| `styles.rs` | Color constants, font definitions, `RoundedInnerShadowView` | Yes | Defines `TRANSPARENT`, `PRIMARY_COLOR`, `SIDEBAR_BG_COLOR`, `MAIN_BG_COLOR`, font templates, custom shadow shader |
| `widgets.rs` | `VerticalFiller`, `HorizontalFiller`, `Line`, `FadeView`, `AttributeTag`, `SidebarMenuButton`, `MolyButton`, `MolyRadioButtonTab`, `MolyTextInput`, `MolySlider`, `MolySwitch`, `TogglePanelButton`, `MolyTogglePanel` | Yes | Heavy custom shaders on most widgets |
| `resource_imports.rs` | `ICON_CLOSE`, `ICON_COPY` | Yes | `dep("crate://self/...")` pattern |
| `meta.rs` | Metadata display widget | Yes | |
| `list.rs` | List utility widget | Yes | |
| `tooltip.rs` | Tooltip widget | Yes | |
| `popup_notification.rs` | `PopupNotification` widget | Yes | |
| `external_link.rs` | `ExternalLink` widget | Yes | |
| `download_notification_popup.rs` | `DownloadNotificationPopup` | Yes | |
| `desktop_buttons.rs` | `MolyDesktopButton` | Yes | Windows-style min/max/close buttons |
| `moly_server_popup.rs` | `MolyServerPopup` | Yes | Server unreachable notification |
| `actions.rs` | `ChatAction`, `DownloadAction` enums | No | `#[derive(Clone, DefaultNone, Debug)]` |
| `bot_context.rs` | `BotContext` struct | No | `Arc<Mutex<InnerBotContext>>`, bridges BotClient to ChatControllers |
| `utils.rs` | Utility functions | No | |
| `utils/` | Submodules: filesystem, attachments, version | No | |

---

## 8. Module Map: `src/chat/`

The main chat feature. Most complex module.

| File | Widget | `live_design!` | Notes |
|------|--------|----------------|-------|
| `chat_screen.rs` | `ChatScreen` | Yes | Top-level chat container, `AdaptiveView` for mobile/desktop, creates `BotContext`, `#[derive(Live, LiveHook, Widget)]` |
| `chat_view.rs` | `ChatView` | Yes | Wraps MolyKit `Chat` widget, manages `ChatController`, bot context binding, attachment persistence via `Glue` plugin. Uses `WidgetMatchEvent`. Heavy `apply_over` usage for mobile padding. |
| `chats_deck.rs` | `ChatsDeck` | Yes | Manages multiple concurrent `ChatView` instances via `HashMap<ChatId, ChatViewRef>`. Uses `WidgetRef::new_from_ptr(cx, self.chat_view_template)` for dynamic creation. LRU eviction at 10 max. |
| `chat_history_panel.rs` | `ChatHistoryPanel` | Yes | Side panel with chat list and toggle |
| `chat_history.rs` | `ChatHistory` | Yes | Chat history list using `PortalList` |
| `chat_history_card.rs` | `ChatHistoryCard` | Yes | Individual chat entry in history |
| `chat_history_card_options.rs` | `ChatHistoryCardOptions` | Yes | Context menu for chat cards |
| `chat_params.rs` | `ChatParams` | Yes | Model parameter sliders (currently disabled) |
| `chat_screen_mobile.rs` | `ChatScreenMobile` | Yes | Mobile layout with `StackNavigation` |
| `deep_inquire_content.rs` | `DeepInquireContent` | Yes | Custom content widget for DeepInquire provider |
| `deep_inquire_stages.rs` | `DeepInquireStages` | Yes | Stage progress display |
| `delete_chat_modal.rs` | `DeleteChatModal` | Yes | Confirmation modal |
| `entity_button.rs` | `EntityButton` | Yes | Clickable entity/bot button |
| `model_info.rs` | `ModelInfo` | Yes | Model information display |
| `moly_bot_filter.rs` | `MolyBotFilter` | No | Implements `BotFilter` trait for filtering bots in selector |
| `shared.rs` | Shared chat constants/styles | Yes | |

---

## 9. Module Map: `src/landing/`

Model discovery and download screen.

| File | Widget | `live_design!` | Notes |
|------|--------|----------------|-------|
| `landing_screen.rs` | `LandingScreen` | Yes | Main discovery view |
| `model_list.rs` | `ModelList` | Yes | Uses `PortalList` for model cards |
| `model_card.rs` | `ModelCard`, `ModelCardViewAllModal` | Yes | Model display card with download links |
| `downloads.rs` | `Downloads` (widget) | Yes | Active downloads view |
| `download_item.rs` | `DownloadItem` | Yes | Individual download progress |
| `search_bar.rs` | `SearchBar` | Yes | Search input |
| `search_loading.rs` | `SearchLoading` | Yes | Loading indicator |
| `sorting.rs` | `Sorting` | Yes | Sort controls |
| `model_files.rs` | `ModelFiles` | Yes | File list for a model |
| `model_files_list.rs` | `ModelFilesList` | Yes | Uses `PortalList` |
| `model_files_item.rs` | `ModelFilesItem` | Yes | Individual file row |
| `model_files_tags.rs` | `ModelFilesTags` | Yes | Tag display |
| `shared.rs` | Shared landing constants | Yes | |

---

## 10. Module Map: `src/settings/`

Provider configuration screens.

| File | Widget | `live_design!` | Notes |
|------|--------|----------------|-------|
| `providers_screen.rs` | `ProvidersScreen` | Yes | Main providers list |
| `providers.rs` | `Providers` widget, `ConnectionSettings` | Yes | Provider list using `PortalList` |
| `provider_view.rs` | `ProviderView` | Yes | Individual provider detail/edit |
| `add_provider_modal.rs` | `AddProviderModal` | Yes | Add custom provider dialog |
| `moly_server_screen.rs` | `MolyServerScreen` | Yes | MolyServer-specific settings |
| `sync_modal.rs` | `SyncModal` | Yes | Sync progress modal |
| `utilities_modal.rs` | `UtilitiesModal` | Yes | Utility settings |

---

## 11. Module Map: `src/mcp/`

MCP (Model Context Protocol) server configuration.

| File | Widget | `live_design!` | Notes |
|------|--------|----------------|-------|
| `mcp_screen.rs` | `McpScreen` | Yes | Main MCP configuration screen |
| `mcp_servers.rs` | `McpServers` | Yes | Server list and JSON config editor |

---

## 12. Module Map: `src/my_models/`

Downloaded model management.

| File | Widget | `live_design!` | Notes |
|------|--------|----------------|-------|
| `my_models_screen.rs` | `MyModelsScreen` | Yes | Main screen |
| `downloaded_files_table.rs` | `DownloadedFilesTable` | Yes | Table of downloaded files using `PortalList` |
| `downloaded_files_row.rs` | `DownloadedFilesRow` | Yes | Individual file row |
| `model_info_modal.rs` | `ModelInfoModal` | Yes | Model detail modal |
| `delete_model_modal.rs` | `DeleteModelModal` | Yes | Delete confirmation modal |

---

## 13. Module Map: `src/data/`

Pure data layer. No `live_design!` blocks. No Makepad widget code.

| File | Content | Notes |
|------|---------|-------|
| `store.rs` | `Store` struct, `StoreAction`, `ModelWithDownloadInfo` | Central state container |
| `chats/` | `Chats` (session manager), `Chat` (individual chat), `ChatId` | Chat persistence, provider registry, available bots |
| `downloads/` | `Downloads`, `download.rs` | Download management, progress tracking |
| `search/` | `Search`, `SortCriteria` | Model search, featured models, sorting |
| `providers.rs` | `Provider`, `ProviderType`, `ProviderBot`, `ProviderConnectionStatus` | Provider definitions |
| `preferences.rs` | `Preferences`, `ProviderPreferences` | Persisted settings |
| `moly_client.rs` | `MolyClient` | HTTP client for MolyServer local API |
| `bot_fetcher.rs` | `should_include_bot()` | Bot filtering logic |
| `deep_inquire_client.rs` | `DeepInquireClient`, `DeepInquireCustomContent` | Custom client for DeepInquire provider |
| `openclaw_client.rs` | `OpenClawClient` | Custom client for OpenClaw provider |
| `mcp_servers.rs` | `McpServersConfig`, `McpServerConfig` | MCP server configuration |
| `supported_providers.rs` | `SupportedProvider`, `load_supported_providers()` | Loads from `supported_providers.json` |
| `supported_providers.json` | JSON | List of known providers with URLs, types, supported models |
| `capture.rs` | `CaptureAction`, capture manager | Clipboard-like context capture |

**Migration note**: The data layer has zero `live_design!` blocks. It uses `LiveDependency` (for provider icons) and `DefaultNone` derive (for actions), both of which will need updating, but no DSL migration.

---

## 14. MolyKit Architecture

MolyKit is a reusable AI chat widget library. It provides:

1. **`Chat` widget** — A complete chat UI (messages list + prompt input + model selector)
2. **`ChatController`** — State management for a chat session (bots, messages, streaming)
3. **Plugin system** — `ChatControllerPlugin` trait for reacting to state mutations
4. **Re-exported aitk** — `moly_kit::aitk` and `moly_kit::prelude::*`

### Registration chain (`widgets.rs`)

```rust
pub fn live_design(cx: &mut Cx) {
    theme_moly_kit_light::live_design(cx);
    cx.link(live_id!(moly_kit_theme), live_id!(theme_moly_kit_light));
    // ... 20+ widget registrations in dependency order
}
```

**Critical migration note**: The `cx.link()` call is how MolyKit sets its theme. This mechanism is removed in new Makepad. The replacement is `mod.theme = me` in `script_mod!`.

### Module structure

- `widgets.rs` — Registration chain + `cx.link()` for theme namespace
- `widgets/` — All widget implementations
- `utils/` — Audio (STT), scraping, Makepad helpers
- `prelude.rs` — Re-exports widget Rust types + `aitk::prelude::*`

---

## 15. MolyKit Widget Inventory

| File | Widget | Notes |
|------|--------|-------|
| `chat.rs` | `Chat` | Top-level chat composite (Messages + PromptInput + ModelSelector + SttInput) |
| `messages.rs` | `Messages` | Message list using `PortalList`, custom content registration |
| `chat_line.rs` | `ChatLine` + 9 variants | `UserLine`, `BotLine`, `LoadingLine`, `ActionLine`, `SttLine`, etc. |
| `prompt_input.rs` | `PromptInput` | `#[deref] CommandTextInput`, attachment support, submit button with custom circular shader |
| `model_selector.rs` | `ModelSelector` | Bot/model selector dropdown |
| `model_selector_list.rs` | `ModelSelectorList` | Filterable bot list (uses `PortalList`) |
| `model_selector_item.rs` | `ModelSelectorItem` | Individual model entry |
| `realtime.rs` | `Realtime` | Real-time audio chat widget |
| `standard_message_content.rs` | `StandardMessageContent` | Default message content renderer |
| `message_markdown.rs` | `MessageMarkdown` | Markdown rendering with code blocks |
| `message_loading.rs` | `MessageLoading` | Loading animation |
| `message_thinking_block.rs` | `MessageThinkingBlock` | "Thinking" expandable block |
| `avatar.rs` | `Avatar` | User/bot avatar display |
| `citation.rs` | `Citation` | Individual citation display |
| `citation_list.rs` | `CitationList` | Citation list |
| `moly_modal.rs` | `MolyModal` | Reusable modal dialog |
| `slot.rs` | `Slot` | `#[wrap]` widget for extensible content slots |
| `attachment_list.rs` | `AttachmentList` | Attachment display list |
| `attachment_view.rs` | `AttachmentView` | Individual attachment |
| `attachment_viewer_modal.rs` | `AttachmentViewerModal` | Full-screen attachment viewer |
| `image_view.rs` | `ImageView` | Image display widget |
| `stt_input.rs` | `SttInput` | Speech-to-text input widget |
| `theme_moly_kit_light.rs` | Theme definitions | Light theme constants and styles |

---

## 16. Key Patterns in Moly

### Pattern 1: `apply_over` + `live!{}`

**Used extensively throughout the codebase**. This is the single most labor-intensive migration item.

```rust
// Example from chat_view.rs — responsive padding
self.prompt_input(ids!(chat.prompt)).apply_over(
    cx,
    live! { padding: {bottom: 50, left: 20, right: 20} },
);
```

```rust
// Example from chats_deck.rs — cached widget padding
self.view.apply_over(
    cx,
    live! { padding: {top: 18, bottom: 0, right: 28, left: 28} },
);
```

**Migration**: Replace with direct field mutation + `redraw(cx)`, or use `AdaptiveView` / Animator. There is no `apply_over` + `live!{}` equivalent in new Makepad.

### Pattern 2: `WidgetRef::new_from_ptr`

Used by `ChatsDeck` for dynamic chat view creation:

```rust
let chat_view = WidgetRef::new_from_ptr(cx, self.chat_view_template);
```

Where `chat_view_template: Option<LivePtr>` is declared with `#[live]`.

**Migration**: Use `ScriptObjectRef` for templates, `cx.with_vm(|vm| WidgetRef::script_from_value(vm, val))` for creation.

### Pattern 3: `Scope::with_data` for Store passing

```rust
let scope = &mut Scope::with_data(store);
self.ui.handle_event(cx, event, scope);
```

**Migration**: This pattern is unchanged in new Makepad.

### Pattern 4: `UiRunner` for async→UI bridge

```rust
let ui = self.ui_runner();
spawn(async move {
    let _ = context.load().await;
    ui.defer_with_redraw(move |me, _cx, _scope| {
        me.creating_bot_context = false;
    });
});
```

**Migration**: Verify `UiRunner` API is unchanged. The `spawn` function is from aitk, not Makepad.

### Pattern 5: `WidgetMatchEvent` for action handling

```rust
impl WidgetMatchEvent for ChatScreen {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        if self.button(ids!(new_chat_button)).clicked(&actions) { ... }
        for action in actions { ... }
    }
}
```

Called via `self.widget_match_event(cx, event, scope)` in `handle_event`.

**Migration**: This pattern is unchanged in new Makepad.

### Pattern 6: `dep("crate://self/...")` for resources

```rust
// In live_design!
ICON_CHAT = dep("crate://self/resources/icons/chat.svg")
```

**Migration**: `crate_resource("self:resources/icons/chat.svg")`

### Pattern 7: Radio button navigation

```rust
let radio_button_set = self.ui.radio_button_set(ids_array!(
    sidebar_menu.chat_tab,
    sidebar_menu.moly_server_tab,
    sidebar_menu.providers_tab,
));
if let Some(selected_tab) = radio_button_set.selected(cx, actions) { ... }
```

**Migration**: Verify `radio_button_set` API exists in new Makepad.

### Pattern 8: `AdaptiveView` for responsive layout

```rust
// In live_design!
adaptive_view = <AdaptiveView> {
    Mobile = { ... }
    Desktop = { ... }
}
```

**Migration**: `AdaptiveView` exists in new Makepad. Syntax changes: remove commas, use new inheritance syntax.

### Pattern 9: `ids!()` and `ids_array!()` macros

```rust
self.ui.view(ids!(body)).set_visible(cx, false);
self.ui.widget(ids!(application_pages.chat_frame)).set_visible(cx, true);
```

**Migration**: These macros are unchanged in new Makepad.

### Pattern 10: `Image.source` with `dep()`

```rust
<Image> {
    width: 50, height: 50,
    source: (ICON_MOLYSERVER),
}
```

**Migration**: `source` becomes `src`, `dep(...)` becomes `crate_resource(...)`.

---

## 17. Key Patterns in MolyKit

### Pattern 1: `#[deref]` on non-View widget

```rust
// PromptInput extends CommandTextInput, not View
pub struct PromptInput {
    #[deref] deref: CommandTextInput,
    ...
}
```

**Migration**: `#[deref]` pattern is unchanged.

### Pattern 2: `#[wrap]` widget (`Slot`)

```rust
#[derive(Live, Widget, LiveHook)]
pub struct Slot {
    #[wrap] widget: Widget,
}
```

**Migration**: `#[wrap]` pattern is unchanged.

### Pattern 3: `cx.link()` for theme namespace

```rust
cx.link(live_id!(moly_kit_theme), live_id!(theme_moly_kit_light));
```

**Migration**: Removed. Use `mod.theme = me` in script_mod.

### Pattern 4: `ChatController` plugin system

```rust
impl ChatControllerPlugin for Glue {
    fn on_state_mutation(&mut self, mutation: &ChatStateMutation, state: &ChatState) { ... }
    fn on_state_ready(&mut self, state: &ChatState, mutations: &[ChatStateMutation]) { ... }
}
```

**Migration**: This is pure Rust code in aitk, not Makepad DSL. No migration needed.

### Pattern 5: Custom content registration

```rust
self.messages(ids!(chat.messages))
    .write()
    .register_custom_content(DeepInquireCustomContent::new(self.deep_inquire_content));
```

Where `deep_inquire_content: LivePtr` is stored for later widget creation.

**Migration**: `LivePtr` → `ScriptObjectRef`.

### Pattern 6: `read()/write()` convenience methods

```rust
chat.read().prompt_input_ref().widget(ids!(model_selector)).as_model_selector().set_grouping(...);
```

**Migration**: These are generated by Makepad's Widget derive. Should be unchanged.

### Pattern 7: Complex custom shaders (AIAnimation orb)

MolyKit has several complex shaders:
- AI animation orb with noise/FBM
- Circular submit button
- Hover effects
- Gradient bars

**Migration**: Shader syntax changes (see migration guide sections 10-14).

---

## 18. Custom Shaders

Files with significant custom shaders (beyond simple backgrounds):

### Moly app
| File | Shader | Complexity |
|------|--------|------------|
| `shared/styles.rs` | `RoundedInnerShadowView` pixel shader | Medium — inner shadow calculation |
| `shared/widgets.rs` | `SidebarMenuButton`, `MolyButton`, `MolyTextInput`, `MolySlider`, `MolySwitch` | Medium — custom hover, cursor, selection, slider rail, toggle pill |
| `chat/chat_view.rs` | `PromptInputWithShadow`, `SttInputWithShadow` | High — shadow vertex + pixel shaders with `GaussShadow` |
| `shared/widgets.rs` (`MolyButton`) | Icon rotation via `clip_and_transform_vertex` | Medium — rotation matrix |

### MolyKit
| File | Shader | Complexity |
|------|--------|------------|
| `prompt_input.rs` | Circular submit button, gradient bar | High |
| `message_loading.rs` | AI animation orb (noise, FBM, animated) | High |
| `avatar.rs` | Hover effects | Low |
| `chat_line.rs` | Line separator gradients | Low |
| `theme_moly_kit_light.rs` | Theme-specific shader overrides | Low |

---

## 19. Action System

Moly uses Makepad's action system for cross-widget communication.

### Custom actions in Moly

```rust
// src/shared/actions.rs
#[derive(Clone, DefaultNone, Debug)]
pub enum ChatAction {
    StartWithoutEntity,
    Start(BotId),
    ChatSelected(ChatId),
    None,
}

#[derive(Clone, DefaultNone, Debug)]
pub enum DownloadAction {
    Play(FileId),
    Pause(FileId),
    Cancel(FileId),
    None,
}
```

```rust
// src/app.rs
#[derive(Clone, DefaultNone, Debug)]
pub enum NavigationAction {
    NavigateToProviders,
    NavigateToMyModels,
    None,
}

#[derive(Clone, DefaultNone, Debug)]
pub enum StoreAction {
    Search(String),
    ResetSearch,
    Sort(SortCriteria),
    None,
}
```

Other actions: `ModelFileItemAction`, `DownloadFileAction`, `DownloadNotificationPopupAction`, `MolyServerPopupAction`, `MolyClientAction`, `DeleteModelModalAction`, `ConnectionSettingsAction`, `CaptureAction`.

**Migration**: `#[derive(DefaultNone)]` → `#[derive(Default)]` + `#[default]` attribute on the `None` variant.

### Action dispatch

Actions are posted via:
- `cx.action(MyAction::Variant)` — from widget context
- `Cx::post_action(MyAction::Variant)` — from anywhere (static method)

Actions are received in `handle_actions` via `action.cast::<MyAction>()`.

**Migration**: Unchanged.

---

## 20. Async Patterns

### Spawning

Moly uses `moly_kit::aitk::utils::asynchronous::spawn` for all async operations. This handles platform differences:
- **Native**: Spawns on Tokio runtime (requires `Send`)
- **WASM**: Uses `wasm_bindgen_futures::spawn_local` (no `Send` required)

### Pattern: spawn + UiRunner.defer

```rust
spawn(async move {
    // Do async work (HTTP, file I/O, etc.)
    let result = some_async_op().await;
    
    // Bridge back to UI thread
    app_runner().defer(move |app, cx, _| {
        app.store.as_mut().unwrap().some_field = result;
        app.ui.redraw(cx);
    });
});
```

### Pattern: spawn + widget UiRunner

```rust
let ui = self.ui_runner();
spawn(async move {
    let _ = context.load().await;
    ui.defer_with_redraw(move |me, _cx, _scope| {
        me.creating_bot_context = false;
    });
});
```

### Channel preference

Per project guidelines: **Favor `futures` crate channels over `tokio` ones** for cross-platform support.

---

## 21. Widget Inventory Summary

### Total `live_design!` blocks by module

| Module | Count |
|--------|-------|
| `src/shared/` | 11 |
| `src/chat/` | 14 |
| `src/landing/` | 12 |
| `src/settings/` | 7 |
| `src/mcp/` | 2 |
| `src/my_models/` | 5 |
| `src/app.rs` | 1 |
| **Moly total** | **~52** |
| **MolyKit** | **~23** |
| **Grand total** | **~75** |

### Total `#[derive(Live...)]` structs

| Crate | Count | Notes |
|-------|-------|-------|
| Moly app | ~57 | Mix of `Live`, `Live + Widget`, `Live + LiveHook + Widget` |
| MolyKit | ~22 | All are widgets |

---

## 22. Migration Risk Assessment

### High risk (most labor-intensive)

1. **`apply_over` + `live!{}` removal** — Used in ~15+ locations across moly and moly-kit. No direct replacement; each usage needs case-by-case analysis (direct field mutation, Animator, or AdaptiveView).

2. **Complex shaders** — Shadow vertex shaders in `chat_view.rs`, AI animation orb in `message_loading.rs`, rotation in `MolyButton`. These need careful Splash shader syntax translation.

3. **`WidgetRef::new_from_ptr` + `LivePtr` templates** — Used by `ChatsDeck` (critical path for multi-chat) and `DeepInquireCustomContent`. Needs `ScriptObjectRef` migration.

4. **`cx.link()` theme mechanism** — MolyKit's entire theme system depends on this. Needs `mod.theme = me` redesign.

### Medium risk

5. **PortalList migrations** — Used in 5+ files (chat history, model list, downloads, providers, model files). The widget itself exists in new Makepad but syntax differences in item template specification.

6. **`DefaultNone` → `Default` + `#[default]`** — ~10+ action enums across both crates.

7. **`dep(...)` → `crate_resource(...)`** — ~20+ resource references. Mechanical but widespread.

8. **`Image.source` → `Image.src`** — Wherever images are used.

### Low risk (mechanical changes)

9. **Import replacements** — `link widgets; use link::*;` → `use mod.prelude.widgets.*`
10. **Comma removal** — All DSL blocks
11. **Theme constant references** — `(THEME_XYZ)` → `theme.xyz`
12. **Derive changes** — `Live` → `Script`, etc.
13. **`live_design(cx)` → `script_mod(vm)`** — All registration calls

### No risk (unchanged)

- `Scope::with_data` pattern
- `WidgetMatchEvent`
- `UiRunner` pattern (verify API)
- `#[deref]`, `#[wrap]` attributes
- `#[rust]` fields
- `ComponentMap` (type parameter changes only)
- `ids!()`, `ids_array!()` macros
- `AdaptiveView` (exists in new, syntax change only)
- All pure Rust data layer code (`src/data/`)
- aitk types and traits (Bot, BotClient, ChatController, etc.)

---

## 23. File-by-File Migration Order

Recommended order, starting with lowest-dependency files and working up:

### Phase 1: Foundation (no widget dependencies on other moly modules)

1. `src/shared/styles.rs` — Color constants, fonts, `RoundedInnerShadowView`
2. `src/shared/resource_imports.rs` — Icon dep references
3. `src/shared/meta.rs` — Metadata widget
4. `src/shared/list.rs` — List widget

### Phase 2: Shared widgets

5. `src/shared/widgets.rs` — All shared widget definitions (depends on styles)
6. `src/shared/popup_notification.rs`
7. `src/shared/external_link.rs`
8. `src/shared/tooltip.rs`
9. `src/shared/desktop_buttons.rs`
10. `src/shared/download_notification_popup.rs`
11. `src/shared/moly_server_popup.rs`

### Phase 3: MolyKit (independent of moly app)

12. `moly-kit/src/widgets/theme_moly_kit_light.rs`
13. `moly-kit/src/widgets/slot.rs`
14. `moly-kit/src/widgets/avatar.rs`
15. `moly-kit/src/widgets/image_view.rs`
16. `moly-kit/src/widgets/citation.rs`, `citation_list.rs`
17. `moly-kit/src/widgets/attachment_view.rs`, `attachment_list.rs`, `attachment_viewer_modal.rs`
18. `moly-kit/src/widgets/moly_modal.rs`
19. `moly-kit/src/widgets/message_loading.rs`, `message_thinking_block.rs`
20. `moly-kit/src/widgets/message_markdown.rs`, `standard_message_content.rs`
21. `moly-kit/src/widgets/chat_line.rs`
22. `moly-kit/src/widgets/messages.rs`
23. `moly-kit/src/widgets/model_selector_item.rs`, `model_selector_list.rs`, `model_selector.rs`
24. `moly-kit/src/widgets/stt_input.rs`
25. `moly-kit/src/widgets/prompt_input.rs`
26. `moly-kit/src/widgets/chat.rs`
27. `moly-kit/src/widgets/realtime.rs`
28. `moly-kit/src/widgets.rs` — Registration chain + theme linking

### Phase 4: Feature screens (depend on shared + MolyKit)

29. `src/landing/shared.rs`
30. `src/landing/search_bar.rs`, `search_loading.rs`, `sorting.rs`
31. `src/landing/model_files_tags.rs`, `model_files_item.rs`, `model_files_list.rs`, `model_files.rs`
32. `src/landing/model_card.rs`
33. `src/landing/download_item.rs`, `downloads.rs`
34. `src/landing/model_list.rs`
35. `src/landing/landing_screen.rs`
36. `src/chat/shared.rs`
37. `src/chat/entity_button.rs`, `model_info.rs`
38. `src/chat/deep_inquire_stages.rs`, `deep_inquire_content.rs`
39. `src/chat/delete_chat_modal.rs`
40. `src/chat/chat_history_card_options.rs`, `chat_history_card.rs`, `chat_history.rs`
41. `src/chat/chat_history_panel.rs`
42. `src/chat/chat_params.rs`
43. `src/chat/chat_view.rs` — (high risk: apply_over, shadow shaders)
44. `src/chat/chats_deck.rs` — (high risk: new_from_ptr, apply_over)
45. `src/chat/chat_screen_mobile.rs`
46. `src/chat/chat_screen.rs`
47. `src/settings/` — all files
48. `src/mcp/` — all files
49. `src/my_models/` — all files

### Phase 5: App root

50. `src/app.rs` — (depends on everything)

### Phase 6: Rust-side changes

51. All `#[derive(Live...)]` → `#[derive(Script...)]` changes
52. All `#[derive(DefaultNone)]` → `#[derive(Default)]` + `#[default]`
53. All `LiveRegister` → `ScriptRegister` implementations
54. `apply_over` replacement (case-by-case across all files)
55. `LivePtr` → `ScriptObjectRef` changes
56. `WidgetRef::new_from_ptr` → `WidgetRef::script_from_value` changes

### Phase 7: Verification

57. Compile and fix all errors
58. Test each screen visually
59. Test mobile layout (AdaptiveView, responsive padding)
60. Test streaming chat, bot switching, attachment handling
61. Test provider connections and model downloads
