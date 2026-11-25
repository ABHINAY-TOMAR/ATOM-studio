import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import { setTimeout as delay } from "node:timers/promises";
import { resolve } from "node:path";

const debugPort = 9227;
const binary = resolve("../../target/debug/atom-studio-desktop.exe");
const vite = spawn(
  process.execPath,
  ["node_modules/vite/bin/vite.js", "--host", "127.0.0.1"],
  { cwd: process.cwd(), stdio: "ignore" },
);
let app;
let socket;

async function waitForPage() {
  const deadline = Date.now() + 20_000;
  while (Date.now() < deadline) {
    if (vite.exitCode !== null)
      throw new Error(`Vite exited with code ${vite.exitCode}`);
    try {
      const response = await fetch("http://127.0.0.1:1420");
      if (response.ok) return;
    } catch {
      await delay(250);
    }
  }
  throw new Error("Vite did not become ready on 127.0.0.1:1420");
}

try {
  await waitForPage();
  app = spawn(resolve(binary), [], {
    cwd: process.cwd(),
    env: {
      ...process.env,
      WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS: `--remote-debugging-port=${debugPort}`,
    },
    stdio: "ignore",
  });

  let page;
  const deadline = Date.now() + 20_000;
  while (Date.now() < deadline) {
    if (app.exitCode !== null)
      throw new Error(`Desktop process exited with code ${app.exitCode}`);
    try {
      const response = await fetch(`http://127.0.0.1:${debugPort}/json/list`);
      if (response.ok) {
        const targets = await response.json();
        page = targets.find(
          (target) =>
            target.type === "page" &&
            target.url?.startsWith("http://127.0.0.1:1420/") &&
            target.webSocketDebuggerUrl,
        );
        if (page) break;
      }
    } catch {
      await delay(250);
    }
  }
  assert.ok(page, "WebView2 remote debugging endpoint did not appear");

  socket = new WebSocket(page.webSocketDebuggerUrl);
  await new Promise((resolveOpen, reject) => {
    socket.addEventListener("open", resolveOpen, { once: true });
    socket.addEventListener("error", reject, { once: true });
  });

  let commandId = 0;
  const pending = new Map();
  const browserErrors = [];
  const requestUrls = new Map();
  socket.addEventListener("message", (event) => {
    const message = JSON.parse(event.data);
    if (
      message.method === "Runtime.consoleAPICalled" &&
      message.params.type === "error"
    ) {
      browserErrors.push(
        message.params.args
          .map((arg) => arg.value ?? arg.description ?? "")
          .join(" "),
      );
    }
    if (message.method === "Network.requestWillBeSent") {
      requestUrls.set(message.params.requestId, message.params.request.url);
    }
    if (message.method === "Network.loadingFailed") {
      browserErrors.push(
        `Network ${message.params.errorText}: ${requestUrls.get(message.params.requestId) ?? message.params.requestId}`,
      );
    }
    if (
      message.method === "Network.responseReceived" &&
      message.params.response.status >= 400
    ) {
      browserErrors.push(
        `HTTP ${message.params.response.status}: ${message.params.response.url}`,
      );
    }
    if (message.method === "Runtime.exceptionThrown") {
      browserErrors.push(
        message.params.exceptionDetails.exception?.description ??
          message.params.exceptionDetails.text,
      );
    }
    if (
      message.method === "Log.entryAdded" &&
      message.params.entry.level === "error"
    ) {
      browserErrors.push(message.params.entry.text);
    }
    if (message.id && pending.has(message.id)) {
      const { resolve: resolveCommand, reject } = pending.get(message.id);
      pending.delete(message.id);
      if (message.error) reject(new Error(message.error.message));
      else resolveCommand(message.result);
    }
  });

  function send(method, params = {}) {
    const id = ++commandId;
    return new Promise((resolveCommand, reject) => {
      pending.set(id, { resolve: resolveCommand, reject });
      socket.send(JSON.stringify({ id, method, params }));
    });
  }

  async function evaluate(expression) {
    const result = await send("Runtime.evaluate", {
      expression,
      returnByValue: true,
      awaitPromise: true,
    });
    if (result.exceptionDetails) {
      throw new Error(
        result.exceptionDetails.exception?.description ??
          result.exceptionDetails.text,
      );
    }
    return result.result.value;
  }

  await send("Runtime.enable");
  await send("Log.enable");
  await send("Page.enable");
  await send("Network.enable");
  const renderDeadline = Date.now() + 10_000;
  let buttonReady = false;
  while (Date.now() < renderDeadline) {
    buttonReady = await evaluate(
      "Boolean(document.querySelector('.ipc-button'))",
    );
    if (buttonReady) break;
    await delay(100);
  }
  if (!buttonReady) {
    const importStatus = await evaluate(
      "import('/src/main.tsx').then(() => 'main module loaded').catch(error => `main module failed: ${error.stack ?? error}`)",
    );
    const pageState = await evaluate(
      "JSON.stringify({href: location.href, title: document.title, body: document.body.innerText, root: document.querySelector('#root')?.innerHTML, scripts: [...document.scripts].map(s => s.src), resources: performance.getEntriesByType('resource').map(r => r.name)})",
    );
    throw new Error(
      `React did not render the IPC test button: ${importStatus}; ${pageState}; browser errors: ${browserErrors.join(" | ")}`,
    );
  }

  const destinations = JSON.parse(
    await evaluate(
      "JSON.stringify([...document.querySelectorAll('[data-nav-id]')].map((item) => item.textContent.trim()))",
    ),
  );
  assert.deepEqual(destinations, ["Workspace", "Appearance"]);

  await evaluate("document.querySelector('[data-nav-id=appearance]').click()");
  assert.equal(
    await evaluate("document.querySelector('h1')?.textContent.trim()"),
    "Appearance",
  );

  await evaluate("document.querySelector('[data-theme-choice=light]').click()");
  assert.equal(
    await evaluate("document.documentElement.dataset.theme"),
    "light",
  );
  assert.equal(
    await evaluate(
      "document.querySelector('[data-theme-choice=light]').getAttribute('aria-pressed')",
    ),
    "true",
  );

  await evaluate(
    "document.querySelector('[data-density-choice=compact]').click()",
  );
  assert.equal(
    await evaluate("document.documentElement.dataset.density"),
    "compact",
  );

  await send("Emulation.setDeviceMetricsOverride", {
    width: 640,
    height: 800,
    deviceScaleFactor: 1,
    mobile: true,
  });
  const responsiveDeadline = Date.now() + 1_000;
  let sidebarWidth = await evaluate(
    "getComputedStyle(document.querySelector('.sidebar')).width",
  );
  while (sidebarWidth !== "66px" && Date.now() < responsiveDeadline) {
    await delay(25);
    sidebarWidth = await evaluate(
      "getComputedStyle(document.querySelector('.sidebar')).width",
    );
  }
  assert.equal(sidebarWidth, "66px");
  await send("Emulation.clearDeviceMetricsOverride");
  await evaluate("document.querySelector('[data-theme-choice=dark]').click()");
  await evaluate(
    "document.querySelector('[data-density-choice=comfortable]').click()",
  );
  await evaluate("document.querySelector('[data-nav-id=workspace]').click()");
  assert.equal(
    await evaluate("document.querySelector('h1')?.textContent.trim()"),
    "Make room for big ideas.",
  );

  await evaluate("document.querySelector('.ipc-button').click()");
  let text = "";
  const ipcDeadline = Date.now() + 5_000;
  while (Date.now() < ipcDeadline) {
    text = await evaluate(
      "document.querySelector('.ipc-result')?.textContent ?? ''",
    );
    if (text.includes("Rust IPC is working.")) break;
    if (text.startsWith("IPC check failed:")) break;
    await delay(100);
  }

  assert.match(text, /Hello, ATOM Studio — Rust IPC is working\./);
  console.log(`Verified frontend → Tauri IPC → Rust → frontend: ${text}`);
} finally {
  socket?.close();
  if (app?.exitCode === null) app.kill();
  if (vite.exitCode === null) vite.kill();
}
