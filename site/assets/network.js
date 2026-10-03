// The network page. The founder sits fixed at the centre; every node handed the file stands one
// ring further out than the node that handed it over, and the testimony between nodes pulls
// lightly across the rings. Everything drawn comes from the site node's observation: signed
// handovers, signed testimony, signed heartbeats. Nothing is inferred and nothing is invented.

const { start, figures, shown, say } = await import(`./live.js${new URL(import.meta.url).search}`);

const d3 = window.d3;
const canvas = document.querySelector(".chart");
const frame = document.querySelector(".sky-frame");
const card = document.querySelector(".card");
const empty = document.querySelector(".sky-empty");
const said = document.getElementById("find-said");
const range = document.querySelector('[data-zoom="range"]');
const reduced = matchMedia("(prefers-reduced-motion: reduce)");
const ctx = canvas.getContext("2d");

const RING = 110;
const MY_KEY = "333-my-node";

let net = null;
let nodes = [];
let links = [];
let byId = new Map();
let sim = null;
let view = { x: 0, y: 0, k: 1 };
let size = { w: 0, h: 0 };
let hover = null;
let selected = null;
let mine = remembered();
let colours = tokens();
let firstFit = true;
let touched = false;

function remembered() {
  try {
    return localStorage.getItem(MY_KEY);
  } catch {
    return null;
  }
}

function remember(id) {
  mine = id;
  try {
    localStorage.setItem(MY_KEY, id);
  } catch {
    // A private window keeps it for this page only.
  }
}

function tokens() {
  const css = getComputedStyle(document.documentElement);
  const v = (n) => css.getPropertyValue(n).trim();
  return { ink: v("--ink"), dim: v("--dim"), faint: v("--faint"), edge: v("--edge-strong"), ok: v("--ok"), quiet: v("--quiet"), later: v("--later"), ground: v("--surface-flat") };
}

const short = (id) => id.slice(0, 12);
const esc = (v) => String(v).replace(/[&<>"]/g, (m) => `&#${m.charCodeAt(0)};`);
const inEpoch = (epoch) => esc(say("js-network-epoch", { epoch: String(epoch) }));

/** What one node is, in this epoch, as far as this site's node can tell. */
function stateOf(n) {
  if (n.founder) return "founder";
  if (n.admitted === null || n.admitted === undefined) return "seen";
  if (net && typeof n.counts_from === "number" && n.counts_from > net.epoch) return "later";
  return n.answered_now ? "ok" : "quiet";
}

const STATE_WORDS = {
  founder: say("js-network-state-founder"),
  ok: say("js-network-state-ok"),
  quiet: say("js-network-state-quiet"),
  later: say("js-network-state-later"),
  seen: say("js-network-state-seen"),
};

/** A state as a dot and its words. The site node's own state is whether it is running. */
function stateLine(n) {
  const d = n.data;
  if (n.site) {
    const up = !!(net && net.running);
    return `<span class="state"><span class="dot ${up ? "ok" : "bad"}"></span>${esc(up ? say("js-network-awake") : say("js-network-not-running"))}</span>`;
  }
  return `<span class="state"><span class="dot ${d.state}"></span>${esc(STATE_WORDS[d.state])}</span>`;
}

function radiusOf(n) {
  if (n.founder) return 13;
  if (n.site) return 10;
  return n.data.state === "seen" ? 4 : 7;
}

/** Turn an observation into simulation nodes and links, keeping positions already settled. */
function build(next) {
  net = next;
  const old = byId;
  byId = new Map();
  nodes = (next.nodes || []).map((data) => {
    const was = old.get(data.id);
    const n = was || { id: data.id };
    n.data = { ...data, state: null };
    n.founder = !!data.founder;
    n.site = !!data.site;
    n.data.state = stateOf(n.data);
    byId.set(n.id, n);
    return n;
  });
  // Generations: how many handovers each node is from a founder.
  const children = new Map();
  for (const e of next.edges || []) {
    if (e.kind !== "handover") continue;
    if (!children.has(e.from)) children.set(e.from, []);
    children.get(e.from).push(e.to);
  }
  const queue = nodes.filter((n) => n.founder).map((n) => [n.id, 0]);
  const depth = new Map(queue);
  while (queue.length) {
    const [id, d] = queue.shift();
    for (const c of children.get(id) || []) {
      if (!depth.has(c)) {
        depth.set(c, d + 1);
        queue.push([c, d + 1]);
      }
    }
  }
  const founders = nodes.filter((n) => n.founder);
  nodes.forEach((n, i) => {
    n.depth = depth.has(n.id) ? depth.get(n.id) : null;
    n.children = children.get(n.id) || [];
    if (n.founder) {
      const j = founders.indexOf(n);
      n.fx = founders.length > 1 ? Math.cos((j / founders.length) * Math.PI * 2) * 40 : 0;
      n.fy = founders.length > 1 ? Math.sin((j / founders.length) * Math.PI * 2) * 40 : 0;
    }
    if (n.x === undefined) {
      const parent = byId.get(n.data.sponsor);
      const a = (i * 2.399963) % (Math.PI * 2);
      n.x = (parent && parent.x !== undefined ? parent.x : 0) + Math.cos(a) * 30;
      n.y = (parent && parent.y !== undefined ? parent.y : 0) + Math.sin(a) * 30;
    }
  });
  links = (next.edges || [])
    .filter((e) => byId.has(e.from) && byId.has(e.to) && e.from !== e.to)
    .map((e) => ({ source: byId.get(e.from), target: byId.get(e.to), kind: e.kind, epoch: e.epoch }));
  restart();
}

function restart() {
  if (!sim) {
    sim = d3.forceSimulation().on("tick", schedule);
  }
  sim
    .nodes(nodes)
    .force("link", d3.forceLink(links).id((n) => n.id)
      .distance((l) => (l.kind === "handover" ? RING * 0.8 : RING * 1.4))
      .strength((l) => (l.kind === "handover" ? 0.7 : 0.03)))
    .force("charge", d3.forceManyBody().strength((n) => (n.founder ? -500 : -140)).distanceMax(600))
    .force("collide", d3.forceCollide((n) => radiusOf(n) + 8))
    .force("ring", d3.forceRadial((n) => (n.depth === null ? (maxDepth() + 1) * RING : n.depth * RING), 0, 0).strength((n) => (n.founder ? 0 : 0.35)));
  if (reduced.matches) {
    sim.stop();
    for (let i = 0; i < 300; i++) sim.tick();
    schedule();
  } else {
    sim.alpha(0.9).restart();
  }
  sim.on("end.fit", () => {
    if (!touched) fit();
  });
}

function maxDepth() {
  let m = 0;
  for (const n of nodes) if (n.depth !== null && n.depth > m) m = n.depth;
  return m;
}

// Drawing ----------------------------------------------------------------------------------

let queued = false;
function schedule() {
  if (queued) return;
  queued = true;
  requestAnimationFrame(() => {
    queued = false;
    draw();
  });
}

function resize() {
  const r = canvas.getBoundingClientRect();
  const ratio = Math.min(devicePixelRatio || 1, 2);
  canvas.width = Math.round(r.width * ratio);
  canvas.height = Math.round(r.height * ratio);
  ctx.setTransform(ratio, 0, 0, ratio, 0, 0);
  size = { w: r.width, h: r.height };
  schedule();
}

const toScreen = (n) => [size.w / 2 + view.x + n.x * view.k, size.h / 2 + view.y + n.y * view.k];

function neighbours(n) {
  const set = new Set([n]);
  for (const l of links) {
    if (l.source === n) set.add(l.target);
    if (l.target === n) set.add(l.source);
  }
  return set;
}

/** Until the reader moves the view, it follows the graph: founder centred, everything in frame. */
function follow() {
  if (touched || !nodes.length) return;
  let far = RING;
  for (const n of nodes) far = Math.max(far, Math.hypot(n.x, n.y) + 30);
  const target = Math.max(0.25, Math.min(1.6, (Math.min(size.w, size.h) / 2 - 40) / far));
  view.k += (target - view.k) * (reduced.matches ? 1 : 0.18);
  view.x = 0;
  view.y = 0;
  range.value = String(Math.round(view.k * 100));
}

function draw() {
  follow();
  const { w, h } = size;
  const c = colours;
  ctx.clearRect(0, 0, w, h);
  const focus = hover || selected;
  const near = hover ? neighbours(hover) : null;
  // Rings: one per generation, faint, so the order outwards reads at a glance.
  ctx.lineWidth = 1;
  for (let d = 1; d <= maxDepth(); d++) {
    ctx.strokeStyle = c.edge;
    ctx.globalAlpha = 0.35;
    ctx.setLineDash([2, 6]);
    ctx.beginPath();
    ctx.arc(w / 2 + view.x, h / 2 + view.y, d * RING * view.k, 0, Math.PI * 2);
    ctx.stroke();
  }
  ctx.setLineDash([]);
  for (const l of links) {
    const [x1, y1] = toScreen(l.source);
    const [x2, y2] = toScreen(l.target);
    const lit = !near || (near.has(l.source) && near.has(l.target) && (l.source === focus || l.target === focus));
    ctx.globalAlpha = lit ? (l.kind === "handover" ? 0.75 : 0.45) : 0.08;
    ctx.strokeStyle = l.kind === "handover" ? c.ink : c.faint;
    ctx.lineWidth = l.kind === "handover" ? 1.6 : 1;
    ctx.setLineDash(l.kind === "silent" ? [4, 4] : l.kind === "answered" ? [1, 3] : []);
    ctx.beginPath();
    ctx.moveTo(x1, y1);
    ctx.lineTo(x2, y2);
    ctx.stroke();
  }
  ctx.setLineDash([]);
  const labelAll = nodes.length <= 40 || view.k >= 1.6;
  for (const n of nodes) {
    const [x, y] = toScreen(n);
    if (x < -40 || y < -40 || x > w + 40 || y > h + 40) continue;
    const dim = near && !near.has(n);
    ctx.globalAlpha = dim ? 0.18 : 1;
    disc(n, x, y, radiusOf(n) * Math.max(0.7, Math.min(1.4, view.k)), c);
    const named = labelAll || n.founder || n.site || n.id === mine || n === focus || (hover && near && near.has(n));
    if (named && !dim) {
      ctx.fillStyle = n === focus || n.id === mine ? c.ink : c.dim;
      ctx.font = `${n === focus || n.founder ? 600 : 500} 12px "JetBrains Mono", monospace`;
      ctx.textAlign = "center";
      ctx.textBaseline = "top";
      // Below the founder's halo or the selection ring, whichever reaches further.
      const r = radiusOf(n) * Math.max(0.7, Math.min(1.4, view.k));
      const below = Math.max(n.founder ? r * 2.1 : r, n.id === mine || n === selected ? r + 7 : r) + 6;
      ctx.fillText(n.id === mine ? say("js-network-yours-label", { name: short(n.id) }) : short(n.id), x, y + below);
    }
  }
  ctx.globalAlpha = 1;
}

function disc(n, x, y, r, c) {
  const s = n.data.state;
  if (n.founder) {
    ctx.fillStyle = c.ink;
    ctx.globalAlpha *= 0.16;
    ctx.beginPath();
    ctx.arc(x, y, r * 2.1, 0, Math.PI * 2);
    ctx.fill();
    ctx.globalAlpha /= 0.16;
  }
  ctx.beginPath();
  ctx.arc(x, y, r, 0, Math.PI * 2);
  if (s === "founder" || s === "ok") {
    ctx.fillStyle = c.ink;
    ctx.fill();
    if (s === "ok") {
      ctx.lineWidth = 2;
      ctx.strokeStyle = c.ok;
      ctx.beginPath();
      ctx.arc(x, y, r + 2.5, 0, Math.PI * 2);
      ctx.stroke();
    }
  } else if (s === "seen") {
    ctx.fillStyle = c.faint;
    ctx.fill();
  } else {
    ctx.fillStyle = c.ground;
    ctx.fill();
    ctx.lineWidth = 1.8;
    ctx.strokeStyle = s === "later" ? c.later : c.quiet;
    ctx.setLineDash(s === "later" ? [3, 3] : []);
    ctx.stroke();
    ctx.setLineDash([]);
  }
  if (n.id === mine || n === selected) {
    ctx.lineWidth = 1.5;
    ctx.strokeStyle = c.ink;
    ctx.beginPath();
    ctx.arc(x, y, r + 7, 0, Math.PI * 2);
    ctx.stroke();
  }
}

// Moving around ------------------------------------------------------------------------------

function setZoom(k, cx = size.w / 2, cy = size.h / 2) {
  touched = true;
  const next = Math.max(0.25, Math.min(4, k));
  // Keep the point under (cx, cy) where it is.
  const wx = (cx - size.w / 2 - view.x) / view.k;
  const wy = (cy - size.h / 2 - view.y) / view.k;
  view.k = next;
  view.x = cx - size.w / 2 - wx * next;
  view.y = cy - size.h / 2 - wy * next;
  range.value = String(Math.round(next * 100));
  schedule();
}

/** The founder at the exact centre, scaled so the farthest node still fits. */
function fit() {
  if (!nodes.length) return;
  let far = RING;
  for (const n of nodes) far = Math.max(far, Math.hypot(n.x, n.y) + 30);
  const half = Math.min(size.w, size.h) / 2 - 40;
  view.k = Math.max(0.25, Math.min(1.6, half / far));
  view.x = 0;
  view.y = 0;
  range.value = String(Math.round(view.k * 100));
  schedule();
}

function centreOn(n) {
  view.x = -n.x * view.k;
  view.y = -n.y * view.k;
  schedule();
}

function nodeAt(px, py) {
  let best = null;
  let bestD = Infinity;
  for (const n of nodes) {
    const [x, y] = toScreen(n);
    const d = Math.hypot(px - x, py - y);
    const reach = radiusOf(n) * Math.max(0.7, Math.min(1.4, view.k)) + 10;
    if (d < reach && d < bestD) {
      best = n;
      bestD = d;
    }
  }
  return best;
}

const pointers = new Map();
let gesture = null;

canvas.addEventListener("pointerdown", (e) => {
  canvas.setPointerCapture(e.pointerId);
  const r = canvas.getBoundingClientRect();
  const p = { x: e.clientX - r.left, y: e.clientY - r.top };
  pointers.set(e.pointerId, p);
  if (pointers.size === 2) {
    const [a, b] = [...pointers.values()];
    gesture = { pinch: Math.hypot(a.x - b.x, a.y - b.y), k: view.k };
    return;
  }
  const n = nodeAt(p.x, p.y);
  gesture = { start: p, last: p, node: n, moved: false };
  if (n && !n.founder) {
    n.fx = n.x;
    n.fy = n.y;
    if (!reduced.matches) sim.alphaTarget(0.25).restart();
  }
  canvas.classList.add("dragging");
});

canvas.addEventListener("pointermove", (e) => {
  const r = canvas.getBoundingClientRect();
  const p = { x: e.clientX - r.left, y: e.clientY - r.top };
  if (pointers.has(e.pointerId)) pointers.set(e.pointerId, p);
  if (gesture && gesture.pinch && pointers.size === 2) {
    const [a, b] = [...pointers.values()];
    setZoom(gesture.k * (Math.hypot(a.x - b.x, a.y - b.y) / gesture.pinch), (a.x + b.x) / 2, (a.y + b.y) / 2);
    return;
  }
  if (gesture && gesture.start) {
    const dx = p.x - gesture.last.x;
    const dy = p.y - gesture.last.y;
    if (Math.hypot(p.x - gesture.start.x, p.y - gesture.start.y) > 4) gesture.moved = true;
    gesture.last = p;
    if (gesture.node && !gesture.node.founder) {
      gesture.node.fx += dx / view.k;
      gesture.node.fy += dy / view.k;
      if (reduced.matches) {
        gesture.node.x = gesture.node.fx;
        gesture.node.y = gesture.node.fy;
      }
    } else {
      view.x += dx;
      view.y += dy;
      touched = true;
    }
    schedule();
    return;
  }
  const n = nodeAt(p.x, p.y);
  if (n !== hover) {
    hover = n;
    canvas.classList.toggle("pointing", !!n);
    schedule();
  }
});

function release(e) {
  pointers.delete(e.pointerId);
  canvas.classList.remove("dragging");
  if (!gesture) return;
  if (gesture.start && !gesture.moved) select(gesture.node, false);
  if (gesture.node && !gesture.node.founder) {
    gesture.node.fx = null;
    gesture.node.fy = null;
    sim.alphaTarget(0);
  }
  gesture = null;
}
canvas.addEventListener("pointerup", release);
canvas.addEventListener("pointercancel", release);
canvas.addEventListener("pointerleave", () => {
  if (!gesture && hover) {
    hover = null;
    schedule();
  }
});
canvas.addEventListener("wheel", (e) => {
  e.preventDefault();
  const r = canvas.getBoundingClientRect();
  setZoom(view.k * Math.exp(-e.deltaY * 0.0015), e.clientX - r.left, e.clientY - r.top);
}, { passive: false });

document.querySelector('[data-zoom="in"]').addEventListener("click", () => setZoom(view.k * 1.25));
document.querySelector('[data-zoom="out"]').addEventListener("click", () => setZoom(view.k / 1.25));
document.querySelector('[data-zoom="fit"]').addEventListener("click", () => {
  touched = false;
  fit();
});
range.addEventListener("input", () => setZoom(Number(range.value) / 100));

// Finding your node -------------------------------------------------------------------------

document.querySelector(".find").addEventListener("submit", (e) => {
  e.preventDefault();
  const typed = e.target.elements.node.value.trim().toLowerCase().replace(/^333:/, "");
  if (!/^[0-9a-f]{6,64}$/.test(typed)) {
    said.textContent = say("js-network-find-bad");
    return;
  }
  const found = nodes.filter((n) => n.id.startsWith(typed));
  if (found.length === 0) {
    said.textContent = say("js-network-find-none");
  } else if (found.length > 1) {
    said.textContent = say("js-network-find-many", { count: found.length });
  } else {
    remember(found[0].id);
    said.textContent = say("js-network-find-marked");
    select(found[0], true);
  }
});

// The card for one node --------------------------------------------------------------------

function select(n, centre) {
  selected = n;
  if (n && centre) centreOn(n);
  renderCard();
  schedule();
}

/** The newest epoch somebody signed that this node answered, or its own heartbeat if newer. */
function lastAnswer(d) {
  const a = typeof d.last_answered === "number" ? d.last_answered : null;
  const h = typeof d.last_heartbeat === "number" ? d.last_heartbeat : null;
  return a === null ? h : h === null ? a : Math.max(a, h);
}

function epochsAgo(epoch) {
  if (typeof epoch !== "number" || !net) return "—";
  const d = net.epoch - epoch;
  return esc(d <= 0 ? say("js-network-epoch-now", { epoch: String(epoch) }) : say("js-network-epoch-ago", { epoch: String(epoch), ago: d }));
}

function chips(ids) {
  if (!ids.length) return "—";
  const shownIds = ids.slice(0, 24);
  const more = ids.length > shownIds.length ? `<span class="dim">${esc(say("js-network-more", { count: ids.length - shownIds.length }))}</span>` : "";
  return `<span class="links">${shownIds.map((id) => `<button class="chip" type="button" data-pick="${id}">${short(id)}</button>`).join("")}${more}</span>`;
}

function renderCard() {
  const n = selected;
  if (!n) {
    card.innerHTML = `<p class="card-none dim">${esc(say("js-network-select"))}</p>`;
    return;
  }
  const d = n.data;
  const roles = [];
  if (n.founder) roles.push(say("js-network-role-founder"));
  if (n.site) roles.push(say("js-network-role-site"));
  if (n.id === mine) roles.push(say("js-network-role-yours"));
  const asked = links.filter((l) => l.kind !== "handover" && l.target === n);
  const asking = links.filter((l) => l.kind !== "handover" && l.source === n);
  const reach = esc(d.reach === "direct" ? say("js-network-reach-direct") : d.reach === "onion" ? say("js-network-reach-tor") : say("js-network-reach-unknown"));
  const given = d.sponsor
    ? say("js-network-given-by", { epoch: esc(d.admitted), sponsor: chips([d.sponsor]) })
    : esc(n.founder ? say("js-network-given-founder") : say("js-network-given-none"));
  const rows = [
    [say("js-network-col-state"), stateLine(n)],
    [say("js-network-col-given"), given],
    [say("js-network-col-counted"), typeof d.counts_from === "number" ? esc(say("js-network-epoch", { epoch: String(d.counts_from) })) : "—"],
    [say("js-network-col-answered"), epochsAgo(lastAnswer(d))],
    [say("js-network-row-said"), typeof d.signal === "number" ? String(d.signal) : esc(say("js-network-nothing"))],
    [say("js-network-col-reached"), reach],
    [say("js-network-row-handed"), chips(n.children)],
    [say("js-network-row-testimony"), esc(say("js-network-testimony", { asked: asked.length, asking: asking.length }))],
  ];
  card.innerHTML = `
    <h3>${short(n.id)}</h3>
    <p class="role">${esc(roles.join(" · ") || say("js-network-role-none"))}</p>
    <p class="full">${n.id}</p>
    <dl>${rows.map(([k, v]) => `<dt>${esc(k)}</dt><dd>${v}</dd>`).join("")}</dl>
    <div class="row">
      <button class="button" type="button" data-copy-name>${esc(say("js-network-copy-name"))}</button>
      ${n.id === mine ? "" : `<button class="button" type="button" data-mine>${esc(say("js-network-mine"))}</button>`}
    </div>`;
  card.querySelector("[data-copy-name]").addEventListener("click", async (e) => {
    try {
      await navigator.clipboard.writeText(n.id);
      e.target.textContent = say("js-copied");
    } catch {
      e.target.textContent = say("js-network-select-name");
    }
  });
  const markMine = card.querySelector("[data-mine]");
  if (markMine) markMine.addEventListener("click", () => {
    remember(n.id);
    renderCard();
    renderTable();
    schedule();
  });
}

card.addEventListener("click", (e) => {
  const id = e.target.getAttribute && e.target.getAttribute("data-pick");
  if (id && byId.has(id)) select(byId.get(id), true);
});

// The table and the report ------------------------------------------------------------------

function renderTable() {
  const body = document.querySelector(".nodes tbody");
  const order = [...nodes].sort((a, b) => (b.founder - a.founder) || ((a.data.admitted ?? 1e12) - (b.data.admitted ?? 1e12)));
  body.innerHTML = order.map((n) => {
    const d = n.data;
    const tags = [n.founder ? say("js-network-tag-founder") : "", n.site ? say("js-network-tag-site") : "", n.id === mine ? say("js-network-tag-yours") : ""].filter(Boolean).map(esc).join(" · ");
    return `<tr>
      <td><button class="name" type="button" data-pick="${n.id}">${short(n.id)}</button>${tags ? `<span class="tag">${tags}</span>` : ""}</td>
      <td>${stateLine(n)}</td>
      <td>${d.sponsor ? inEpoch(d.admitted) : "—"}</td>
      <td>${typeof d.counts_from === "number" ? inEpoch(d.counts_from) : "—"}</td>
      <td>${typeof lastAnswer(d) === "number" ? inEpoch(lastAnswer(d)) : "—"}</td>
      <td>${typeof d.signal === "number" ? d.signal : "—"}</td>
      <td>${d.reach === "direct" ? esc(say("js-network-reach-direct")) : d.reach === "onion" ? esc(say("js-network-reach-tor-short")) : "—"}</td>
    </tr>`;
  }).join("");
}

document.querySelector(".nodes").addEventListener("click", (e) => {
  const id = e.target.getAttribute && e.target.getAttribute("data-pick");
  if (id && byId.has(id)) {
    select(byId.get(id), true);
    frame.scrollIntoView({ behavior: reduced.matches ? "auto" : "smooth", block: "center" });
  }
});

const words = (key) => key.replace(/_/g, " ").replace(/^./, (ch) => ch.toUpperCase());

function valueOf(v) {
  if (v === null || v === undefined) return "—";
  if (typeof v === "boolean") return esc(v ? say("js-network-yes") : say("js-network-no"));
  if (typeof v === "number") return v.toLocaleString(document.documentElement.lang || "en");
  return esc(v);
}

function rowsOf(obj) {
  return Object.entries(obj).map(([k, v]) => {
    if (v && typeof v === "object" && !Array.isArray(v)) return `<dt>${esc(words(k))}</dt><dd></dd><div class="nested"><dl>${rowsOf(v)}</dl></div>`;
    if (Array.isArray(v)) {
      if (!v.length) return `<dt>${esc(words(k))}</dt><dd>${esc(say("js-network-none"))}</dd>`;
      return `<dt>${esc(words(k))}</dt><dd>${v.length}</dd><div class="nested">${v.map((item) => (item && typeof item === "object" ? `<dl>${rowsOf(item)}</dl>` : `<dl><dt></dt><dd>${valueOf(item)}</dd></dl>`)).join("")}</div>`;
    }
    return `<dt>${esc(words(k))}</dt><dd>${valueOf(v)}</dd>`;
  }).join("");
}

function renderReport() {
  const status = (net && net.status) || {};
  const plain = {};
  const groups = [];
  for (const [k, v] of Object.entries(status)) {
    if (v && typeof v === "object") groups.push([k, v]);
    else plain[k] = v;
  }
  const sections = [[null, plain], ...groups];
  document.querySelector(".report").innerHTML = sections
    .map(([k, v]) => `<section><h3>${esc(k === null ? say("js-network-this-node") : words(k))}</h3><dl>${Array.isArray(v) ? rowsOf({ entries: v }) : rowsOf(v)}</dl></section>`)
    .join("");
  const asof = document.querySelector("[data-asof]");
  if (asof && net && net.as_of) {
    asof.textContent = `${new Date(net.as_of).toISOString().slice(0, 19).replace("T", " ")} UTC`;
    asof.dateTime = new Date(net.as_of).toISOString();
  }
}

// Wiring ----------------------------------------------------------------------------------------

function apply(next) {
  build(next);
  const f = figures(next);
  const strip = document.querySelector('[data-live="figures"]');
  if (strip) {
    const answering = strip.querySelector('[data-figure="answering"]');
    const roll = strip.querySelector('[data-figure="roll"]');
    if (answering) answering.textContent = shown(f.answering);
    if (roll) roll.textContent = shown(f.roll);
  }
  empty.hidden = nodes.length > 1;
  empty.textContent = nodes.length <= 1 ? say("js-network-empty") : "";
  if (selected) selected = byId.get(selected.id) || null;
  if (firstFit) {
    firstFit = false;
    const wanted = new URLSearchParams(location.search).get("node");
    const pick = (wanted && nodes.find((n) => n.id.startsWith(wanted.toLowerCase()))) || (mine && byId.get(mine)) || nodes.find((n) => n.founder) || null;
    fit();
    if (pick) select(pick, false);
  }
  renderCard();
  renderTable();
  renderReport();
}

matchMedia("(prefers-color-scheme: light)").addEventListener("change", () => {
  colours = tokens();
  schedule();
});
new ResizeObserver(resize).observe(canvas);
resize();
start(apply);
