// The board for people: every verified statement as an invitation to copy, newest first.
// The server already wrote the list into the page; this only keeps it current.
const { start, say, base } = await import(`./live.js${new URL(import.meta.url).search}`);

const list = document.querySelector("[data-board]");
const empty = document.querySelector("[data-board-empty]");
const esc = (v) => String(v).replace(/[&<>"]/g, (m) => `&#${m.charCodeAt(0)};`);

function read(id) {
  try {
    return JSON.parse(document.getElementById(id).textContent);
  } catch {
    return null;
  }
}

function render(lines, site) {
  const rows = Array.isArray(lines) ? [...lines].sort((a, b) => b.epoch - a.epoch) : [];
  empty.hidden = rows.length > 0;
  list.innerHTML = rows.map((l) => {
    const short = esc(l.node.slice(0, 12));
    const node = `<a href="${esc(base)}/network?node=${short}">${short}</a>`;
    let line = say("js-board-said", { epoch: String(l.epoch), node });
    if (l.node === site) line += ` · ${esc(say("js-board-site"))}`;
    if (l.tor) line += ` · ${esc(say("js-board-tor"))}`;
    else if (l.country) line += ` · ${esc(l.country)}`;
    return `
    <li>
      <div class="command"><code>333 join 333:${esc(l.address)}</code><button type="button" data-copy>${esc(say("js-copy"))}</button></div>
      <p class="dim">${line}</p>
    </li>`;
  }).join("");
}

const net = read("network-data");
start();
setInterval(async () => {
  try {
    const answer = await fetch("/api/board", { cache: "no-store" });
    if (answer.ok) render(await answer.json(), net && net.site_node);
  } catch {
    // The list on screen stays as it was.
  }
}, 30000);
