# ATOM Studio UI/UX Redesign Specification

## Product UX principles

- Desktop-native, dark-first, information-dense but calm; use progressive disclosure for tools and settings.
- Organize around work and deliverables, not a blank chat canvas or an agent catalog.
- Make agent/task/tool activity observable and explain what is waiting, running, blocked, failed or complete.
- Clearly separate model reasoning/output from tool activity; never expose hidden chain-of-thought.
- Confirm consequential external actions and make permissions visible before execution.
- Use original ATOM visual identity. Do not copy OpenCode or the Agency Swarm TUI.

## Information architecture

1. **Workspace**: user-selected project roots, recent workspaces, scoped file access.
2. **Sessions**: conversation/task history with filters, resume and archive.
3. **Work surface**: prompt/composer, streaming response, selected agent/team/workflow (only existing OpenSwarm agents), task progress and activity timeline.
4. **Deliverables**: documents, slide decks, images, video/audio and generated files, grouped by session/project with preview and open-in-folder actions.
5. **Integrations**: connected services and granted permissions; connection state and revoke controls.
6. **Settings**: provider/model credentials/configuration, default agent behavior, workspace/security permissions, notifications, appearance and diagnostics.

## Main desktop layout

- Persistent left navigation: workspace switcher, primary destinations, recent sessions.
- Center work area: task/conversation content and prompt composer, with clear session identity and context.
- Optional right activity/inspector panel: agents, task graph/status, tool calls, provider and artifact details. Collapsible to protect working space.
- Global status: active tasks, connection/provider status and notifications; avoid always-on noisy metrics.
- Responsive adaptation: narrow window collapses sidebar/inspector into drawers; do not rely on hover-only controls.

## Core interaction flows

- **Start work:** choose workspace/context, describe outcome, optionally select a specialist or allow orchestrator routing. Show current tool permissions and provider readiness.
- **Run:** stream answer and discrete agent/task/activity events. User can cancel; queued/running/waiting states are distinct. Tool details show safe summaries and side-effect classification.
- **Approve:** before consequential send/delete/modify actions, show the exact action and target, with allow/deny or scope options where supported. This approval is enforced by engine policy.
- **Review output:** render artifact cards/previews and file locations; provide retry/edit/open actions supported by actual tools.
- **Recover:** errors explain cause and next step, offer retry only when safe; preserve partial work and state.
- **Configure:** provider/model settings have validation, key status, redacted secret inputs, test connection, and per-feature capability indicators.

## Visual system

Establish tokens before screens: semantic colors for background/surface/border/text/accent/success/warning/error/focus; spacing scale; typography scale; density presets; radius/elevation; keyboard focus states. Use a consistent icon set with text labels for high-impact actions. Dark-first is the default, but token architecture should permit a light theme without duplicating styles. Meet WCAG 2.2 AA contrast where applicable, keyboard navigation, visible focus, reduced-motion preference and screen-reader names.

## Required states

Every network/task-driven view includes loading, empty, offline/provider-unavailable, permission-required, partial-result, cancelled, timeout, error and success states. Activity views distinguish sensitive redacted data from expandable safe details. Avoid fabricated progress percentages when the backend cannot report them.

## MVP screens

- Welcome/workspace selection and first-run provider configuration.
- Main workspace/session view with conversation, task activity and cancellation.
- Agent/tool activity inspector.
- Deliverables browser for existing artifact types.
- Integrations and permission management.
- Settings/provider and model configuration.
- Session history/reopen.

No voice studio, advanced video studio, advanced image studio, AutoClip, Voicebox, or future-engine pages.
