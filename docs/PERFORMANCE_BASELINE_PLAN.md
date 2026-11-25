# ATOM Studio Performance Baseline Plan

No benchmark numbers are available yet. The source audit indicates likely costs (provider latency, agent handoffs, Composio/network, browser and office conversion, media processing, bootstrap), but this is not measured performance evidence.

## Baseline before porting

1. Choose representative user journeys: simple assistant response, orchestrator-to-specialist handoff, parallel independent specialist work, research, data analysis, document/PPTX generation, image generation, video/audio operation, and cold launcher startup.
2. Record environment exactly: OS/build, CPU model/core count, RAM, storage, GPU if relevant, Python/Node/package versions, provider/model, network, warm/cold cache, input sizes, and dependency versions.
3. Separate local overhead from remote time. Instrument spans for queue wait, agent scheduling, provider request/TTFT/stream duration, tool execution, serialization, rendering/transcoding, persistence and total completion. Keep model/network costs outside engine-only latency claims.
4. Measure cold start and warm idle CPU/memory; peak memory and process count; task/tool dispatch overhead; UI input/stream responsiveness; artifact throughput/latency; error/cancellation behavior.
5. Repeat enough runs to report sample count, median and tail (p95 where meaningful), variance, and failures. Do not compare different model/provider settings as if they isolate Rust impact.

## Rust comparison

Replay the same captured prompts/inputs and environment. Compare end-to-end and per-span measurements, resource bounds and artifact correctness. Use Criterion for deterministic in-process hot paths only after profiling identifies them. Include process/IPC boundary cost for desktop workflows. Re-run on each supported platform and in release builds.

## Acceptance targets

Numerical targets are intentionally unset until baseline collection and target hardware decisions. First goals are no regression in capability/correctness, bounded resource use, responsive cancellation, and no UI stalls. After baselines, agree targets per journey (startup, first token, task completion, memory budget, throughput) and keep raw reproducible data under `rust/benchmarks/` or CI artifacts.
