// The story under the first screen: one sentence over one picture, both driven by the scroll
// position (never by a timer), so the picture is always where the reader is. The pictures are
// diagrams of the rules, not of the live network; the live network has its own page.

// The page's words; home.js hands over the real ones before anything is drawn.
let say = (key) => key;
const reduced = matchMedia("(prefers-reduced-motion: reduce)");
const BEAT_VH = 0.85;

function tokens() {
  const css = getComputedStyle(document.documentElement);
  const get = (name) => css.getPropertyValue(name).trim();
  return { ink: get("--ink"), dim: get("--dim"), faint: get("--faint"), edge: get("--edge-strong"), ok: get("--ok"), ground: get("--surface-flat") };
}

const ease = (x) => (x <= 0 ? 0 : x >= 1 ? 1 : x * x * (3 - 2 * x));
const span = (t, a, b) => ease((t - a) / (b - a));

function node(c, x, y, r, fill, stroke, alpha = 1) {
  c.globalAlpha = alpha;
  c.beginPath();
  c.arc(x, y, r, 0, Math.PI * 2);
  if (fill) {
    c.fillStyle = fill;
    c.fill();
  }
  if (stroke) {
    c.lineWidth = 1.5;
    c.strokeStyle = stroke;
    c.stroke();
  }
  c.globalAlpha = 1;
}

function label(c, text, x, y, colour, size, face = "mono", align = "center") {
  c.fillStyle = colour;
  c.textAlign = align;
  c.textBaseline = "middle";
  c.font = face === "mono" ? `500 ${size}px "JetBrains Mono", monospace` : `600 ${size}px "Fraunces", Georgia, serif`;
  c.fillText(text, x, y);
}

function line(c, x1, y1, x2, y2, colour, width = 1.5, dash = []) {
  c.setLineDash(dash);
  c.strokeStyle = colour;
  c.lineWidth = width;
  c.beginPath();
  c.moveTo(x1, y1);
  c.lineTo(x2, y2);
  c.stroke();
  c.setLineDash([]);
}

/** 0 — three bytes, out of the one light the first screen gathered into. */
function bytes(c, k, t) {
  const { cx, cy, u } = k;
  const size = 1.7 * u;
  for (let i = 0; i < 3; i++) {
    const p = span(t, 0.05 + i * 0.12, 0.35 + i * 0.12);
    const x = cx + (i - 1) * (size + 0.45 * u);
    const s = size * (0.2 + 0.8 * p);
    k.c.globalAlpha = 0.2 + 0.8 * p;
    k.c.fillStyle = k.ground;
    k.c.strokeStyle = k.edge;
    k.c.lineWidth = 1.5;
    k.c.beginPath();
    k.c.roundRect(x - s / 2, cy - s / 2, s, s, 0.18 * u);
    k.c.fill();
    k.c.stroke();
    k.c.globalAlpha = p;
    label(c, "3", x, cy + 0.04 * u, k.ink, 1.15 * u * (0.4 + 0.6 * p), "serif");
    label(c, "0x33", x, cy + size / 2 + 0.45 * u, k.faint, 0.3 * u);
    k.c.globalAlpha = 1;
  }
  const f = span(t, 0.62, 0.85);
  c.globalAlpha = f;
  label(c, say("js-story-file"), cx, cy - size / 2 - 0.7 * u, k.dim, 0.34 * u);
  c.globalAlpha = 1;
}

/** 1 — a handover, signed twice. */
function handover(c, k, t) {
  const { cx, cy, u } = k;
  const ax = cx - 3 * u;
  const bx = cx + 3 * u;
  line(c, ax, cy, bx, cy, k.edge);
  const got = span(t, 0.52, 0.62);
  node(c, ax, cy, 0.5 * u, k.ink);
  node(c, bx, cy, 0.5 * u, got > 0 ? k.ink : null, k.ink, 1);
  if (got < 1) node(c, bx, cy, 0.5 * u, k.ground, k.ink, 1 - got);
  const m = span(t, 0.12, 0.52);
  if (m > 0 && m < 1) {
    const x = ax + (bx - ax) * m;
    c.fillStyle = k.ink;
    c.beginPath();
    c.roundRect(x - 0.28 * u, cy - 0.95 * u, 0.56 * u, 0.4 * u, 4);
    c.fill();
    label(c, "333", x, cy - 0.75 * u, k.ground, 0.22 * u);
  }
  const g = span(t, 0.62, 0.74);
  const r = span(t, 0.74, 0.86);
  c.globalAlpha = g;
  label(c, say("js-story-gave"), ax, cy + 1.1 * u, k.dim, 0.3 * u);
  label(c, say("js-story-signed"), ax, cy + 1.6 * u, k.faint, 0.26 * u);
  c.globalAlpha = r;
  label(c, say("js-story-received"), bx, cy + 1.1 * u, k.dim, 0.3 * u);
  label(c, say("js-story-signed"), bx, cy + 1.6 * u, k.faint, 0.26 * u);
  c.globalAlpha = 1;
}

const ASKERS = [-Math.PI / 2, Math.PI / 6, (5 * Math.PI) / 6];

/** 2 — an epoch of 333 minutes, and three drawn to ask. */
function asking(c, k, t) {
  const { cx, cy, u } = k;
  const ring = 2.1 * u;
  c.strokeStyle = k.edge;
  c.lineWidth = 1.5;
  c.beginPath();
  c.arc(cx, cy, ring, 0, Math.PI * 2);
  c.stroke();
  c.strokeStyle = k.ink;
  c.lineWidth = 2.5;
  c.beginPath();
  c.arc(cx, cy, ring, -Math.PI / 2, -Math.PI / 2 + Math.PI * 2 * span(t, 0, 0.9));
  c.stroke();
  label(c, say("js-story-minutes"), cx, cy + ring + 0.5 * u, k.faint, 0.28 * u);
  const ask = span(t, 0.18, 0.5);
  const answer = span(t, 0.55, 0.82);
  for (const a of ASKERS) {
    const x = cx + Math.cos(a) * 3.6 * u;
    const y = cy + Math.sin(a) * 3.6 * u * 0.74;
    node(c, x, y, 0.32 * u, k.dim);
    if (ask > 0) line(c, x, y, x + (cx - x) * ask * 0.88, y + (cy - y) * ask * 0.88, k.dim, 1.5, [5, 5]);
    if (answer > 0) line(c, cx, cy, cx + (x - cx) * answer * 0.9, cy + (y - cy) * answer * 0.9, k.ink, 2);
  }
  const pulse = answer > 0 && answer < 1 ? 1 + 0.25 * Math.sin(answer * Math.PI) : 1;
  node(c, cx, cy, 0.48 * u * pulse, k.ink);
}

/** 3 — what they sign goes onto this node's record: one cell per epoch, the last 333. */
function record(c, k, t) {
  const { cx, cy, u } = k;
  const cols = 37;
  const rows = 9;
  const cell = Math.min(0.24 * u, (k.w - 2 * u) / cols / 1.25);
  const pitch = cell * 1.25;
  const left = cx - (cols * pitch) / 2;
  const top = cy - (rows * pitch) / 2 - 0.3 * u;
  const filled = Math.floor(333 * (0.5 + 0.5 * span(t, 0.05, 0.7)));
  for (let i = 0; i < 333; i++) {
    const x = left + (i % cols) * pitch;
    const y = top + Math.floor(i / cols) * pitch;
    const absent = (i * 7919) % 23 === 0;
    c.fillStyle = i < filled && !absent ? k.ink : k.edge;
    c.globalAlpha = i < filled ? 1 : 0.5;
    c.fillRect(x, y, cell, cell);
  }
  c.globalAlpha = 1;
  const bottom = top + rows * pitch + 0.45 * u;
  label(c, say("js-story-epochs"), left, bottom, k.faint, 0.28 * u, "mono", "left");
  label(c, say("js-story-now"), left + cols * pitch - (pitch - cell), bottom, k.faint, 0.28 * u, "mono", "right");
  const drop = span(t, 0.72, 0.95);
  if (drop > 0) {
    const last = left + ((filled - 1) % cols) * pitch + cell / 2;
    const lastY = top + Math.floor((filled - 1) / cols) * pitch + cell / 2;
    for (let j = 0; j < 3; j++) {
      const x = last + (j - 1) * 0.5 * u * (1 - drop);
      const y = top - 1.2 * u + (lastY - top + 1.2 * u) * drop;
      c.globalAlpha = 1 - drop * 0.7;
      label(c, "✓", x, y, k.ink, 0.38 * u);
    }
    c.globalAlpha = 1;
  }
}

const GRID = [9, 4];
const LIT = new Set([1, 4, 6, 9, 13, 16, 20, 23, 27, 31, 34]);

/** 4 — names on the roll, and the few answering. */
function counted(c, k, t, dark = 0, keep = -1) {
  const { cx, cy, u } = k;
  const gap = 0.95 * u;
  let on = 0;
  for (let row = 0; row < GRID[1]; row++) {
    for (let col = 0; col < GRID[0]; col++) {
      const i = row * GRID[0] + col;
      const x = cx + (col - (GRID[0] - 1) / 2) * gap;
      const y = cy - 0.7 * u + (row - (GRID[1] - 1) / 2) * gap;
      const order = [...LIT].indexOf(i);
      const lit = i === keep || (order >= 0 && span(t, 0.1 + order * 0.04, 0.2 + order * 0.04) > 0.5 && order >= Math.round(dark * LIT.size));
      if (lit) on += 1;
      node(c, x, y, 0.2 * u, lit ? k.ink : null, lit ? null : k.edge);
    }
  }
  const y = cy + 2.3 * u;
  label(c, String(on), cx - 1.6 * u, y, k.ink, 0.8 * u);
  label(c, say("js-story-answering"), cx - 1.6 * u, y + 0.7 * u, k.dim, 0.28 * u);
  label(c, String(GRID[0] * GRID[1]), cx + 1.6 * u, y, k.faint, 0.8 * u);
  label(c, say("js-story-roll"), cx + 1.6 * u, y + 0.7 * u, k.faint, 0.28 * u);
  return on;
}

/** 5 — the lights go out; the count of 19,683 years; one answer ends it. */
function silence(c, k, t) {
  const { cx, cy, u } = k;
  const back = t > 0.84;
  const on = counted(c, k, 1, back ? 1 : span(t, 0, 0.42), back ? 13 : -1);
  const count = span(t, 0.46, 0.56) * (back ? 1 - span(t, 0.84, 0.9) : 1);
  if (on === 0 && count > 0) {
    c.globalAlpha = count;
    const years = 19683 - Math.floor(span(t, 0.56, 0.84) * 2);
    c.fillStyle = k.ground;
    c.fillRect(cx - 3.6 * u, cy + 1.6 * u, 7.2 * u, 1.6 * u);
    label(c, say("js-story-years", { years }), cx, cy + 2.35 * u, k.ink, 0.62 * u);
    c.globalAlpha = 1;
  }
}

const SCENES = [bytes, handover, asking, record, (c, k, t) => counted(c, k, t), silence];

/** Start the story; returns nothing, and leaves the plain list when there is no canvas. */
export function story(section, speak) {
  if (speak) say = speak;
  const canvas = section.querySelector(".stage");
  const items = [...section.querySelectorAll(".captions li")];
  const c = canvas.getContext("2d");
  if (!c) return;
  section.classList.add("live");
  section.style.setProperty("--beats", String(items.length));
  const marks = document.createElement("div");
  marks.className = "progress";
  marks.innerHTML = items.map(() => "<i></i>").join("");
  section.querySelector(".story-pin").append(marks);
  let colours = tokens();
  matchMedia("(prefers-color-scheme: light)").addEventListener("change", () => {
    colours = tokens();
    draw();
  });
  let size = { w: 0, h: 0 };
  const resize = () => {
    const r = canvas.getBoundingClientRect();
    const ratio = Math.min(devicePixelRatio || 1, 2);
    canvas.width = Math.round(r.width * ratio);
    canvas.height = Math.round(r.height * ratio);
    c.setTransform(ratio, 0, 0, ratio, 0, 0);
    size = { w: r.width, h: r.height };
    draw();
  };
  let shown = -1;
  function draw() {
    const top = section.getBoundingClientRect().top;
    const beat = innerHeight * BEAT_VH;
    const p = Math.max(0, Math.min(items.length - 0.001, -top / beat));
    const i = Math.floor(p);
    const t = reduced.matches ? 1 : p - i;
    if (i !== shown) {
      items.forEach((li, j) => li.classList.toggle("now", j === i));
      [...marks.children].forEach((m, j) => m.classList.toggle("on", j <= i));
      shown = i;
    }
    const { w, h } = size;
    c.clearRect(0, 0, w, h);
    const u = Math.min(w / 11, h / 6.2);
    SCENES[i](c, { c, cx: w / 2, cy: h / 2, u, w, h, ...colours }, t);
  }
  let queued = false;
  addEventListener("scroll", () => {
    if (queued) return;
    queued = true;
    requestAnimationFrame(() => {
      queued = false;
      draw();
    });
  }, { passive: true });
  addEventListener("resize", resize);
  document.fonts.ready.then(resize);
  resize();
}
