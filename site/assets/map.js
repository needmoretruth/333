// Where we are: one read when the page opens, and more only while the reader asks for them.
// Off until asked. One fetch when the page opens, and nothing after that unless the
// person turns it on, because a page that quietly polls for ever is a page that costs
// somebody money on a metered connection.
var EVERY = 30000;
var timer = null;
var naming = null;
try { naming = new Intl.DisplayNames(["en"], { type: "region" }); } catch (e) { naming = null; }

function nameOf(code) {
  if (code === "TOR") return "Tor";
  if (!naming) return code;
  try { return naming.of(code) || code; } catch (e) { return code; }
}

function draw(data) {
  var dots = document.getElementById("dots");
  dots.textContent = "";
  var seen = {};
  for (var i = 0; i < data.dots.length; i++) {
    var lon = data.dots[i][0], lat = data.dots[i][1];
    var key = lon + "," + lat;
    seen[key] = (seen[key] || 0) + 1;
  }
  for (var at in seen) {
    if (!Object.prototype.hasOwnProperty.call(seen, at)) continue;
    var parts = at.split(",");
    var x = (Number(parts[0]) + 180) / 360 * 1440;
    var y = (90 - Number(parts[1])) / 180 * 720;
    var many = seen[at];
    var ring = document.createElementNS("http://www.w3.org/2000/svg", "circle");
    ring.setAttribute("class", "ring");
    ring.setAttribute("cx", x); ring.setAttribute("cy", y);
    ring.setAttribute("r", 9 + Math.min(many, 9) * 3);
    dots.appendChild(ring);
    var dot = document.createElementNS("http://www.w3.org/2000/svg", "circle");
    dot.setAttribute("class", "dot");
    dot.setAttribute("cx", x); dot.setAttribute("cy", y);
    dot.setAttribute("r", 5 + Math.min(many, 9));
    var title = document.createElementNS("http://www.w3.org/2000/svg", "title");
    title.textContent = many + (many === 1 ? " node" : " nodes");
    dot.appendChild(title);
    dots.appendChild(dot);
  }

  var rows = document.getElementById("rows");
  rows.textContent = "";
  function row(where, count, klass) {
    var tr = document.createElement("tr");
    if (klass) tr.className = klass;
    var a = document.createElement("td"); a.textContent = where;
    var b = document.createElement("td"); b.className = "n"; b.textContent = String(count);
    tr.appendChild(a); tr.appendChild(b); rows.appendChild(tr);
  }
  for (var j = 0; j < data.countries.length; j++) {
    row(nameOf(data.countries[j].c), data.countries[j].n, null);
  }
  if (data.tor) row("Tor", data.tor, null);
  if (data.unplaced) row("Nowhere the edge could place", data.unplaced, null);
  if (!data.countries.length && !data.tor && !data.unplaced) {
    var tr = document.createElement("tr");
    var td = document.createElement("td");
    td.className = "dim"; td.colSpan = 2;
    td.textContent = "Nobody is saying where they are.";
    tr.appendChild(td); rows.appendChild(tr);
  } else {
    row("All of us saying", data.saying, "sum");
  }
  document.getElementById("said").textContent =
    "Read at " + new Date(data.as_of).toISOString().replace("T", " ").slice(0, 19) + " UTC.";
}

function look() {
  fetch("/333/where-we-are", { cache: "no-store" })
    .then(function (r) { return r.ok ? r.json() : Promise.reject(r.status); })
    .then(draw)
    .catch(function () {
      document.getElementById("said").textContent = "The board could not be read just now.";
    });
}

document.getElementById("live").addEventListener("click", function () {
  var on = timer !== null;
  if (on) {
    clearInterval(timer); timer = null;
  } else {
    timer = setInterval(look, EVERY);
    look();
  }
  this.setAttribute("aria-pressed", String(!on));
  this.textContent = on ? "Watch it live" : "Stop watching";
  document.getElementById("lamp").className = on ? "dot" : "dot ok";
});

look();
