import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Activity, Check, Layers3, MonitorCog, Moon, Sun } from "lucide-react";

type Destination = "workspace" | "appearance";
type Theme = "dark" | "light";
type Density = "comfortable" | "compact";

function readPreference<T extends string>(
  key: string,
  allowed: readonly T[],
  fallback: T,
): T {
  try {
    const value = localStorage.getItem(key);
    return allowed.includes(value as T) ? (value as T) : fallback;
  } catch {
    return fallback;
  }
}

function App() {
  const [destination, setDestination] = useState<Destination>("workspace");
  const [theme, setTheme] = useState<Theme>(() =>
    readPreference("atom-theme", ["dark", "light"], "dark"),
  );
  const [density, setDensity] = useState<Density>(() =>
    readPreference("atom-density", ["comfortable", "compact"], "comfortable"),
  );
  const [ipcMessage, setIpcMessage] = useState("Not checked yet");
  const [checkingIpc, setCheckingIpc] = useState(false);

  useEffect(() => {
    document.documentElement.dataset.theme = theme;
    document.documentElement.dataset.density = density;
    try {
      localStorage.setItem("atom-theme", theme);
      localStorage.setItem("atom-density", density);
    } catch {
      // Preferences remain available for this session when storage is disabled.
    }
  }, [theme, density]);

  async function testRustConnection() {
    setCheckingIpc(true);
    setIpcMessage("Contacting the Rust backend…");
    try {
      setIpcMessage(
        await invoke<string>("ipc_smoke_test", { name: "ATOM Studio" }),
      );
    } catch (error) {
      setIpcMessage(`IPC check failed: ${String(error)}`);
    } finally {
      setCheckingIpc(false);
    }
  }

  return (
    <div className="app-frame">
      <aside className="sidebar" aria-label="Application sidebar">
        <a
          className="brand-lockup"
          href="#workspace"
          aria-label="ATOM Studio home"
          onClick={() => setDestination("workspace")}
        >
          <span className="brand-mark">
            <Layers3 size={19} strokeWidth={2.2} />
          </span>
          <span className="brand-copy">
            <span className="brand-name">
              ATOM <span>Studio</span>
            </span>
            <span className="brand-caption">CREATIVE WORKSPACE</span>
          </span>
        </a>
        <div className="workspace-switcher" aria-label="Current workspace">
          <span className="workspace-avatar">A</span>
          <span className="workspace-label">
            <strong>My workspace</strong>
            <small>Personal</small>
          </span>
        </div>
        <span className="nav-heading">WORKSPACE</span>
        <nav className="primary-nav" aria-label="Workspace navigation">
          <button
            className={`nav-item ${destination === "workspace" ? "active" : ""}`}
            data-nav-id="workspace"
            type="button"
            onClick={() => setDestination("workspace")}
            aria-current={destination === "workspace" ? "page" : undefined}
          >
            <Layers3 size={17} />
            <span>Workspace</span>
          </button>
        </nav>
        <div className="sidebar-spacer" />
        <nav
          className="primary-nav sidebar-bottom"
          aria-label="Application settings"
        >
          <button
            className={`nav-item ${destination === "appearance" ? "active" : ""}`}
            data-nav-id="appearance"
            type="button"
            onClick={() => setDestination("appearance")}
            aria-current={destination === "appearance" ? "page" : undefined}
          >
            <MonitorCog size={17} />
            <span>Appearance</span>
          </button>
        </nav>
        <div className="sidebar-runtime">
          <span className="runtime-dot" />
          <span>
            <strong>Desktop shell</strong>
            <small>Foundation ready</small>
          </span>
        </div>
        <div className="profile-row">
          <span className="profile-avatar">A</span>
          <span>
            <strong>ATOM Studio</strong>
            <small>Local profile</small>
          </span>
        </div>
      </aside>

      <main className="main-panel">
        <header className="topbar">
          <div className="breadcrumbs">
            <span>My workspace</span>
            <span className="breadcrumb-slash">/</span>
            <strong>
              {destination === "workspace" ? "Workspace" : "Appearance"}
            </strong>
          </div>
          <span className="build-chip">
            <span /> DESKTOP FOUNDATION
          </span>
        </header>
        <div className="page-scroll">
          {destination === "workspace" ? (
            <>
              <section className="welcome-row">
                <div>
                  <span className="eyebrow">ATOM STUDIO · DESKTOP</span>
                  <h1>Make room for big ideas.</h1>
                  <p className="welcome-subtitle">
                    A focused home for your projects, ideas, and the work you
                    create.
                  </p>
                </div>
              </section>
              <section className="hero-card" aria-labelledby="hero-title">
                <div className="hero-glow" />
                <div className="hero-content">
                  <span className="hero-kicker">
                    A CONSIDERED SPACE FOR YOUR WORK
                  </span>
                  <h2 id="hero-title">
                    Your tools.
                    <br />
                    <span>Your momentum.</span>
                  </h2>
                  <p>
                    ATOM Studio is the desktop foundation for OpenSwarm’s
                    existing capabilities.
                  </p>
                  <div className="hero-meta">
                    <Layers3 size={15} />
                    <span>Rust-powered desktop</span>
                    <span className="meta-separator">·</span>
                    <span>Workspace foundation</span>
                  </div>
                </div>
                <div className="hero-art" aria-hidden="true">
                  <div className="orbit orbit-one" />
                  <div className="orbit orbit-two" />
                  <div className="atom-core">
                    <Layers3 size={34} />
                  </div>
                </div>
              </section>
              <section className="section-heading">
                <div>
                  <span className="section-overline">FOUNDATION</span>
                  <h2>Your workspace, ready to grow</h2>
                </div>
              </section>
              <section className="setup-grid">
                <article className="setup-card">
                  <span className="card-step">01 — DESKTOP</span>
                  <h3>Native application shell</h3>
                  <p>
                    A focused desktop workspace with a Rust backend and a
                    responsive interface.
                  </p>
                  <div className="card-footer">
                    <span className="status-pill ready">
                      <span />
                      Ready
                    </span>
                  </div>
                </article>
                <article className="setup-card">
                  <span className="card-step">02 — NEXT PHASE</span>
                  <h3>OpenSwarm capabilities</h3>
                  <p>
                    Existing agents and workflows will be integrated
                    incrementally after this foundation.
                  </p>
                  <div className="card-footer">
                    <span className="status-pill planned">
                      <span />
                      Migration planned
                    </span>
                  </div>
                </article>
              </section>
              <section className="ipc-panel" aria-labelledby="ipc-heading">
                <div className="ipc-copy">
                  <div className="ipc-icon">
                    <Activity size={17} />
                  </div>
                  <div>
                    <span className="section-overline">FOUNDATION CHECK</span>
                    <h2 id="ipc-heading">Rust connection</h2>
                    <p>Verify the frontend ↔ Rust desktop bridge.</p>
                  </div>
                </div>
                <div className="ipc-action">
                  <output className="ipc-result" aria-live="polite">
                    {ipcMessage}
                  </output>
                  <button
                    className="ipc-button"
                    type="button"
                    onClick={testRustConnection}
                    disabled={checkingIpc}
                  >
                    {checkingIpc ? "Checking…" : "Test connection"}
                  </button>
                </div>
              </section>
            </>
          ) : (
            <>
              <section className="welcome-row">
                <div>
                  <span className="eyebrow">PERSONALIZE YOUR WORKSPACE</span>
                  <h1>Appearance</h1>
                  <p className="welcome-subtitle">
                    Adjust the visual theme and information density.
                  </p>
                </div>
              </section>
              <section className="preference-section">
                <div className="section-heading">
                  <div>
                    <span className="section-overline">COLOR THEME</span>
                    <h2>Choose your appearance</h2>
                  </div>
                </div>
                <div className="choice-grid">
                  <button
                    className="choice-card"
                    data-theme-choice="dark"
                    type="button"
                    aria-pressed={theme === "dark"}
                    onClick={() => setTheme("dark")}
                  >
                    <Moon size={19} />
                    <span>
                      <strong>Dark</strong>
                      <small>Easy on the eyes in low light</small>
                    </span>
                    {theme === "dark" && (
                      <Check className="choice-check" size={17} />
                    )}
                  </button>
                  <button
                    className="choice-card"
                    data-theme-choice="light"
                    type="button"
                    aria-pressed={theme === "light"}
                    onClick={() => setTheme("light")}
                  >
                    <Sun size={19} />
                    <span>
                      <strong>Light</strong>
                      <small>A brighter workspace</small>
                    </span>
                    {theme === "light" && (
                      <Check className="choice-check" size={17} />
                    )}
                  </button>
                </div>
              </section>
              <section className="preference-section">
                <div className="section-heading">
                  <div>
                    <span className="section-overline">LAYOUT</span>
                    <h2>Information density</h2>
                  </div>
                </div>
                <div className="choice-grid">
                  <button
                    className="choice-card"
                    data-density-choice="comfortable"
                    type="button"
                    aria-pressed={density === "comfortable"}
                    onClick={() => setDensity("comfortable")}
                  >
                    <span className="density-icon">Aa</span>
                    <span>
                      <strong>Comfortable</strong>
                      <small>More space between elements</small>
                    </span>
                    {density === "comfortable" && (
                      <Check className="choice-check" size={17} />
                    )}
                  </button>
                  <button
                    className="choice-card"
                    data-density-choice="compact"
                    type="button"
                    aria-pressed={density === "compact"}
                    onClick={() => setDensity("compact")}
                  >
                    <span className="density-icon compact-icon">Aa</span>
                    <span>
                      <strong>Compact</strong>
                      <small>Fit more content on screen</small>
                    </span>
                    {density === "compact" && (
                      <Check className="choice-check" size={17} />
                    )}
                  </button>
                </div>
              </section>
            </>
          )}
          <footer className="page-footer">
            <span>
              ATOM STUDIO <span className="footer-dot">·</span> DESKTOP
              FOUNDATION
            </span>
            <span>OpenSwarm runtime integration is a later phase</span>
          </footer>
        </div>
      </main>
    </div>
  );
}

export default App;
