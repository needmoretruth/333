// Shared by every page: the numbers that follow the site's node, the time left in the epoch,
// and the copy buttons. The server already wrote every number into the HTML; this only keeps
// them current while the page stays open.

const EVERY_MS = 15000;

/** The observation the server embedded, or null. */
export function embedded() {
  const node = document.getElementById("network-data");
  if (!node) return null;
  try {
    return JSON.parse(node.textContent);
  } catch {
    return null;
  }
}

/** Fetch the observation again; null when the site cannot say. */
export async function fetchNetwork() {
  try {
    const answer = await fetch("/api/network", { cache: "no-store" });
    return answer.ok ? await answer.json() : null;
  } catch {
    return null;
  }
}

/** Call `apply` with every fresh observation, starting with the embedded one. */
export function follow(apply) {
  const first = embedded();
  if (first) apply(first);
  let timer = 0;
  const tick = async () => {
    const next = await fetchNetwork();
    if (next) apply(next);
  };
  const start = () => {
    if (!timer) timer = setInterval(tick, EVERY_MS);
  };
  const stop = () => {
    clearInterval(timer);
    timer = 0;
  };
  document.addEventListener("visibilitychange", () => {
    if (document.hidden) stop();
    else {
      tick();
      start();
    }
  });
  start();
}

/** The parts of an observation every page shows. */
export function figures(net) {
  const status = (net && net.status) || {};
  return {
    answering: num(status.answering),
    roll: num(status.roll),
    epoch: num(net && net.epoch),
    ends: net && typeof net.epoch_ends === "number" ? net.epoch_ends : null,
    running: !!(net && net.running),
  };
}

function num(value) {
  return typeof value === "number" && Number.isFinite(value) ? value : null;
}

/** A number as people read it, or a dash for one nobody can produce. */
export function shown(value) {
  return value === null ? "—" : value.toLocaleString("en-US");
}

/** Hours and minutes left, written short. */
export function left(ms) {
  if (ms === null) return "—";
  const minutes = Math.max(0, Math.round((ms - Date.now()) / 60000));
  const h = Math.floor(minutes / 60);
  const m = minutes % 60;
  return h > 0 ? `in ${h} h ${m} min` : `in ${m} min`;
}

function header(net) {
  const f = figures(net);
  const state = document.querySelector('[data-live="state"]');
  if (!state) return;
  const dot = state.querySelector(".dot");
  const word = state.querySelector(".word");
  dot.className = `dot ${f.running ? "ok" : "bad"}`;
  word.textContent = f.running ? "This site's node is awake" : "This site's node is not running";
}

function countdowns(ends) {
  for (const time of document.querySelectorAll("[data-countdown]")) {
    const at = ends ?? Date.parse(time.getAttribute("datetime"));
    if (!Number.isFinite(at)) continue;
    time.dateTime = new Date(at).toISOString();
    time.textContent = left(at);
    time.title = `${new Date(at).toISOString().slice(0, 16).replace("T", " ")} UTC`;
  }
}

function copies() {
  // One listener for the whole page, so buttons drawn later copy too.
  document.addEventListener("click", async (e) => {
    const button = e.target.closest && e.target.closest("[data-copy]");
    if (!button) return;
    const code = button.parentElement.querySelector("code");
    try {
      await navigator.clipboard.writeText(code.textContent.trim());
      button.textContent = "Copied";
      button.setAttribute("data-done", "");
    } catch {
      const range = document.createRange();
      range.selectNodeContents(code);
      const selection = getSelection();
      selection.removeAllRanges();
      selection.addRange(range);
      button.textContent = "Selected";
    }
    setTimeout(() => {
      button.textContent = "Copy";
      button.removeAttribute("data-done");
    }, 1800);
  });
}

/** Wire up what every page shares. `more` receives each observation as well. */
export function start(more) {
  copies();
  countdowns(null);
  follow((net) => {
    header(net);
    countdowns(figures(net).ends);
    if (more) more(net);
  });
  setInterval(() => countdowns(null), 30000);
}
