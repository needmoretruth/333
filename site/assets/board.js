// The board for people: every verified statement as an invitation to copy, newest first.
import { start } from "./live.js";

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
  list.innerHTML = rows.map((l) => `
    <li>
      <div class="command"><code>333 join 333:${esc(l.address)}</code><button type="button" data-copy>Copy</button></div>
      <p class="dim">Said in epoch ${esc(l.epoch)} by <a href="/network?node=${esc(l.node.slice(0, 12))}">${esc(l.node.slice(0, 12))}</a>${l.node === site ? " · this site's node" : ""}${l.tor ? " · through Tor" : l.country ? ` · ${esc(l.country)}` : ""}</p>
    </li>`).join("");
}

const net = read("network-data");
render(read("board-data"), net && net.site_node);
start();
setInterval(async () => {
  try {
    const answer = await fetch("/api/board", { cache: "no-store" });
    if (answer.ok) render(await answer.json(), net && net.site_node);
  } catch {
    // The list on screen stays as it was.
  }
}, 30000);
