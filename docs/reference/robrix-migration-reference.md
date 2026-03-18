# Robrix Makepad 2.0 Migration Reference

**Source PR:** https://github.com/project-robius/robrix/pull/761
**Branch:** `migrate_to_makepad_2.0`
**State:** OPEN (WIP)

This is the migration PR for Robrix (a Matrix chat client built with Makepad) from the old
`live_design!` / Live DSL system to the new Splash / `script_mod!` system. The checklist below
documents what works and what upstream Makepad PRs were needed to fix issues.

## Upstream Makepad fixes made during the Robrix migration

| Issue | Makepad PR |
|-------|-----------|
| Tooltip broken | https://github.com/makepad/makepad/pull/881 |
| CalloutTooltip (new widget) | https://github.com/makepad/makepad/pull/882 |
| IconRotated (new widget, later removed) | https://github.com/makepad/makepad/pull/883 |
| CircleView / Avatar broken | https://github.com/makepad/makepad/pull/887 |
| TextInput color_empty_hover/focus not used | https://github.com/makepad/makepad/pull/927 |
| Hideable dock tab bars | https://github.com/makepad/makepad/pull/937 |
| HtmlLink broken | https://github.com/makepad/makepad/pull/948 |
| Tooltip hover-in needs click first | https://github.com/makepad/makepad/pull/950 |
| find_within returns deepest match | https://github.com/makepad/makepad/pull/956 |
| Context menu missing labels/icons | https://github.com/makepad/makepad/pull/961 |

## Widget migration checklist (from PR description)

### Makepad-internal widgets
- [x] Tooltip
- [x] Hideable dock tab bars
- [x] CalloutTooltip
- [x] Icon
- [x] IconRotated (removed from Robrix, no longer needed)
- [x] TextInput
- [x] HtmlLink
- [x] Tooltip behavior (hover-in)

### `/src/shared/`
- [x] Avatar
- [x] bouncing dots (animation)
- [x] collapsible header
- [x] confirmation modal
- [x] expand arrow (newly created for 2.0)
- [x] helpers
- [x] html or plaintext
  - [ ] needs formatting fixes for text spacing
  - [x] RobrixHtmlLink
  - [ ] MatrixHtmlSpan: implemented but not fully tested
- [ ] icon_button
  - [ ] RobrixIconButton: works, but hover/down styling needs improvement
    - Note: Makepad's new Button no longer has a `hover` animator for `draw_icon`
  - [x] IconButton: removed, everything uses RobrixIconButton
- [ ] ImageViewer: rotation works, panning and scroll-to-zoom do not
- [x] jump to bottom button
- [ ] mentionable text input
- [x] popup list of notifications (fix apply_over)
- [x] restore status view
- [x] room filter input bar
- [x] styles (RobrixTextInput, SimpleTextInput removed)
- [x] Text or Image
- [x] Timestamp
- [x] UnreadBadge
- [x] VerificationBadge

### `src/home/`
- [x] edited indicator
- [ ] editing pane
- [x] event reaction list
- [x] event source modal
- [x] home screen
- [x] Invite modal
- [x] invite screen
- [x] light themed dock (full rework done)
- [ ] link preview
- [x] loading pane
- [x] location preview
- [x] main desktop ui
- [x] main mobile ui
- [x] navigation_tab_bar (including default radio button selection on startup)
- [x] new message context menu
- [x] room context menu
- [x] room image viewer
- [x] room read receipt
- [x] room screen (Message, CondensedMessage, ImageMessage, CondensedImageMessage)
- [ ] rooms list entry: works, but `update_preview_colors()` styling was removed
- [x] rooms list header
- [x] rooms list
- [x] rooms sidebar
- [x] search messages (not yet used)
- [x] space lobby
- [x] spaces bar (works on desktop, untested on mobile)
- [x] tombstone footer
- [x] welcome screen

### `src/login/`
- [x] LoginScreen
- [x] LoginStatusModal

### `src/logout/`
- [x] logout confirm modal

### `src/room/`
- [x] reply preview
- [ ] room input bar (issues with mentionable text input)
- [x] typing notice

### `src/settings/`
- [x] SettingsScreen
- [x] AccountSettings

### Other
- [x] FadeView: fully removed, no longer needed
- [x] User profile sliding pane
- [ ] join leave room modal
- [ ] verification modal
- [ ] app.rs: works, image viewer code can be simplified

### Notable observations
- AdaptiveView is now fully working in 2.0 (mobile views work)
- FadeView was completely removed (no longer needed in new system)
- IconRotated was added then removed (no longer needed)
- SimpleTextInput was removed in favor of RobrixTextInput
- Button no longer has a `hover` animator for `draw_icon` in new Makepad
- PortalList had a scroll-jump bug when scrolling upwards at bottom (fixed)
