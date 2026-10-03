// Shared by every page: the numbers that follow the site's node, the time left in the epoch,
// and the copy buttons. The server already wrote every number into the HTML; this only keeps
// them current while the page stays open.

const EVERY_MS = 15000;

const root = document.documentElement;

/** The page's language, and the path its links in that language begin with. */
export const lang = root.lang || "en";
export const base = root.dataset.base || "";

/** The words this page's scripts say, in its language, as the server embedded them. */
const WORDS = (() => {
  try {
    return JSON.parse(document.getElementById("words").textContent) || {};
  } catch {
    return {};
  }
})();

const rules = {};
function category(n, type) {
  try {
    rules[type] = rules[type] || new Intl.PluralRules(lang, { type });
    return rules[type].select(n);
  } catch {
    return "other";
  }
}

/** A number as people read it in this page's language; anything else as it is. */
export function number(value) {
  return typeof value === "number" ? value.toLocaleString(lang) : String(value);
}

/**
 * One message, filled in. A message with a selector is an object naming its variable
 * (`$`), whether it counts or ranks (`type`), its default (`*`) and each variant.
 * Arguments are not escaped here; whoever puts the result into HTML escapes them first.
 */
export function say(key, args = {}) {
  let said = WORDS[key];
  if (said === undefined) return key;
  if (said && typeof said === "object") {
    const n = args[said.$];
    const picked = typeof n === "number" ? said[`=${n}`] ?? said[category(n, said.type)] : undefined;
    said = picked ?? said[said["*"]];
  }
  return String(said).replace(/\{\$([A-Za-z0-9_-]+)\}/g, (_, name) => (name in args ? number(args[name]) : ""));
}

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
  return value === null ? "—" : value.toLocaleString(lang);
}

/** Hours and minutes left, written short. */
export function left(ms) {
  if (ms === null) return "—";
  const minutes = Math.max(0, Math.round((ms - Date.now()) / 60000));
  const h = Math.floor(minutes / 60);
  const m = minutes % 60;
  return h > 0 ? say("js-in-hours", { h, m }) : say("js-in-minutes", { m });
}

function header(net) {
  const f = figures(net);
  const state = document.querySelector('[data-live="state"]');
  if (!state) return;
  const dot = state.querySelector(".dot");
  const word = state.querySelector(".word");
  dot.className = `dot ${f.running ? "ok" : "bad"}`;
  word.textContent = f.running ? say("js-state-awake") : say("js-state-not-running");
}

/** Which epoch of this line it is: counted from the first epoch any node was admitted in. */
export function lineEpoch(net) {
  const epoch = net && typeof net.epoch === "number" ? net.epoch : null;
  let first = null;
  for (const node of (net && net.nodes) || []) {
    if (typeof node.admitted === "number" && (first === null || node.admitted < first)) first = node.admitted;
  }
  return epoch === null || first === null || first > epoch ? null : epoch - first + 1;
}

/** The epoch number and which of this line's it is, wherever the page shows them. */
function epochs(net) {
  const f = figures(net);
  if (f.epoch === null) return;
  for (const el of document.querySelectorAll("[data-epoch]")) el.textContent = shown(f.epoch);
  const n = lineEpoch(net);
  for (const el of document.querySelectorAll(".line-epoch")) el.textContent = n === null ? "" : say("js-line-epoch", { n });
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
      button.textContent = say("js-copied");
      button.setAttribute("data-done", "");
    } catch {
      const range = document.createRange();
      range.selectNodeContents(code);
      const selection = getSelection();
      selection.removeAllRanges();
      selection.addRange(range);
      button.textContent = say("js-selected");
    }
    setTimeout(() => {
      button.textContent = say("js-copy");
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
    // A stale observation's epoch and end time are from when it was made; the server
    // wrote the ones the clock gives now, so those stay.
    if (!net.stale) {
      epochs(net);
      countdowns(figures(net).ends);
    }
    if (more) more(net);
  });
  setInterval(() => countdowns(null), 30000);
}
