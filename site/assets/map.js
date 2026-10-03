// Where we are: the server drew the map and its table into the page; this names each
// country in the reader's language and, only while the reader asks for it, keeps both current.
// Off until asked, because a page that quietly polls for ever is a page that costs
// somebody money on a metered connection.
const { say, lang } = await import(`./live.js${new URL(import.meta.url).search}`);

const EVERY = 30000;
const SVG = "http://www.w3.org/2000/svg";
let timer = null;
let naming = null;
try { naming = new Intl.DisplayNames([lang], { type: "region" }); } catch { naming = null; }

function nameOf(code) {
  if (!naming) return code;
  try { return naming.of(code) || code; } catch { return code; }
}

/** Name every country the page shows by its code. */
function nameCountries() {
  for (const cell of document.querySelectorAll("#rows [data-c]")) {
    cell.textContent = nameOf(cell.getAttribute("data-c"));
  }
}

function mark(group, x, y, many) {
  const ring = document.createElementNS(SVG, "circle");
  ring.setAttribute("class", "ring");
  ring.setAttribute("cx", x); ring.setAttribute("cy", y);
  ring.setAttribute("r", 9 + Math.min(many, 9) * 3);
  group.appendChild(ring);
  const dot = document.createElementNS(SVG, "circle");
  dot.setAttribute("class", "dot");
  dot.setAttribute("cx", x); dot.setAttribute("cy", y);
  dot.setAttribute("r", 5 + Math.min(many, 9));
  const title = document.createElementNS(SVG, "title");
  title.textContent = say("js-map-dot", { count: many });
  dot.appendChild(title);
  group.appendChild(dot);
}

function draw(data) {
  const dots = document.getElementById("dots");
  dots.textContent = "";
  const seen = new Map();
  for (const [lon, lat] of data.dots) seen.set(`${lon},${lat}`, (seen.get(`${lon},${lat}`) || 0) + 1);
  for (const [at, many] of seen) {
    const [lon, lat] = at.split(",").map(Number);
    mark(dots, (lon + 180) * 4, (90 - lat) * 4, many);
  }
  const tor = document.getElementById("tor-dots");
  tor.textContent = "";
  for (const [x, y] of data.tor_dots || []) mark(tor, x, y, 1);

  const rows = document.getElementById("rows");
  rows.textContent = "";
  function row(where, count, klass) {
    const tr = document.createElement("tr");
    if (klass) tr.className = klass;
    const a = document.createElement("td"); a.textContent = where;
    const b = document.createElement("td"); b.className = "n"; b.textContent = String(count);
    tr.appendChild(a); tr.appendChild(b); rows.appendChild(tr);
  }
  for (const country of data.countries) row(nameOf(country.c), country.n, null);
  if (data.tor) row(say("js-map-tor"), data.tor, null);
  if (data.unplaced) row(say("js-map-nowhere"), data.unplaced, null);
  if (!data.saying) {
    const tr = document.createElement("tr");
    const td = document.createElement("td");
    td.className = "dim"; td.colSpan = 2;
    td.textContent = say("js-map-nobody");
    tr.appendChild(td); rows.appendChild(tr);
  } else {
    row(say("js-map-all"), data.saying, "sum");
  }
  if (typeof data.unsaid === "number") row(say("js-map-unsaid"), data.unsaid, "unsaid");
  document.getElementById("said").textContent =
    say("js-map-read-at", { read_at: new Date(data.as_of).toISOString().replace("T", " ").slice(0, 19) });
}

function look() {
  fetch("/333/where-we-are", { cache: "no-store" })
    .then((r) => (r.ok ? r.json() : Promise.reject(r.status)))
    .then(draw)
    .catch(() => {
      document.getElementById("said").textContent = say("js-map-unreadable");
    });
}

document.getElementById("live").addEventListener("click", function () {
  const on = timer !== null;
  if (on) {
    clearInterval(timer); timer = null;
  } else {
    timer = setInterval(look, EVERY);
    look();
  }
  this.setAttribute("aria-pressed", String(!on));
  this.textContent = on ? say("js-map-watch") : say("js-map-stop");
  document.getElementById("lamp").className = on ? "dot" : "dot ok";
});

nameCountries();
