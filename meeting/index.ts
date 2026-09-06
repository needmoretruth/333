import { opened, type Said } from "./whereabouts";

// The meeting point.
//
// It is one fixed address in a design that wanted none. A node with an invitation never
// touches it; two nodes on one network find each other without it. It exists for the third
// case — two machines on two networks, nobody to make the introduction — and it does the
// least that case allows: it holds signed statements about where nodes say they are, hands
// them back to whoever asks, and forgets them after two epochs.
//
// IT IS NOT TRUSTED BY ANYTHING THAT READS IT. Every line on the board is signed by the node
// it names, and the client verifies before it believes. This server cannot invent a member,
// cannot forge an address, cannot vouch for one, and cannot tell which of the statements it
// is holding are true. What it can do is disappear, which is why nothing depends on it twice.
//
// Written in TypeScript rather than Rust, which the Recommendations permit for a thing this
// size: one file, no build step of its own, and small enough that a reader can hold all of it
// at once. The client that talks to it is Rust like everything else.

/** How long a statement is held before it is dropped: two epochs of 333 minutes. */
const KEPT_FOR_MS = 2 * 333 * 60 * 1000;

/** The most statements the board holds. The oldest go first. */
const MOST = 333;

/** The largest statement accepted, in bytes. A whereabouts frame is a signature, a key, a
 *  timestamp and an address; an onion address is the longest of those and this is far above
 *  it. It is here so that a stranger cannot fill the board with one request. */
const LONGEST = 512;

/** The key the whole board lives under. One key rather than one per node: reading the board
 *  is then one read instead of three hundred, and the cost of that is that two nodes speaking
 *  in the same instant can lose one of the two. A node that is not on the board announces
 *  again next epoch, so a lost write costs an epoch and nothing else. */
const BOARD = "board";

/** How long one address waits before it may leave another statement, in seconds.
 *
 *  A node has one thing to say and 333 minutes in which to say it, so a minute between words
 *  is generous by three hundred times over and still stops one machine from spending the
 *  whole day of writing in a second. */
const BETWEEN_WORDS = 60;

/** The most times the board may be written in one day.
 *
 *  The store this runs on allows a thousand writes a day and refuses everything after that,
 *  which would take the board down until midnight and take the rest of the account with it.
 *  This is under that on purpose. Reading is never refused, so a board that has stopped
 *  taking statements still hands out every one it already holds. */
const MOST_WRITES_A_DAY = 900;

/** One node's statement about where it is. */
interface Line {
  /** The name of the node the statement is about, in hex — a slot name, not a claim. A liar
   *  can write into somebody else's slot and gains nothing by it: the bytes underneath are
   *  signed, so the worst that is achieved is one wasted slot. */
  k: string;
  /** The frame itself, base64 of the exact bytes the node signed. */
  b: string;
  /** When this server received it, in milliseconds. Used only to forget. */
  t: number;
  /** Where the node was when it said this, as far as the edge could tell.
   *
   *  ABSENT ON PURPOSE FOR A HIDDEN NODE. A node publishing an onion address reaches this
   *  server over the ordinary internet, so the edge does know which country the request came
   *  from — and that is exactly the fact the onion address exists to withhold. It is dropped
   *  here rather than stored and hidden later: `"tor"` is all that is kept, and there is
   *  nothing in the record to leak afterwards.
   *
   *  Missing on lines written before this existed, and on any request the edge could not
   *  place. Both are counted as unplaced and neither is guessed at. */
  p?: Place;
}

/** Where a node was when it wrote, or the fact that it will not say. */
type Place = "tor" | { c: string; y: number; x: number };

/** The board, as it is stored. */
interface Board {
  /** Which day the count below belongs to, in whole days since the epoch of the clock. */
  d: number;
  /** How many times the board has been written on that day. */
  w: number;
  /** The statements. */
  e: Line[];
}

/** The parts of the edge's own account of a request that this file reads, and no more. */
interface Edge {
  /** Two-letter country code, or null where the edge could not place the request. */
  country?: string | null;
  /** Degrees north, as a string, where the edge has it. */
  latitude?: string | null;
  /** Degrees east, as a string, where the edge has it. */
  longitude?: string | null;
}

/** A request as it actually arrives here: the standard one, plus what the edge knows. */
type Arriving = Request & { readonly cf?: Edge };

/** The parts of the Workers runtime this file uses, and no more. */
interface Store {
  get(key: string, type: "text"): Promise<string | null>;
  put(key: string, value: string): Promise<void>;
}
/** The static pages, fetched from the edge rather than from anywhere this code can see. */
interface Pages {
  fetch(request: Request): Promise<Response>;
}
interface Env {
  BOARD: Store;
  ASSETS: Pages;
}

/** The runtime's streaming HTML editor, declared to the extent this file uses it. */
declare const HTMLRewriter: new () => {
  on(
    selector: string,
    handlers: { element(element: { setInnerContent(text: string): void }): void },
  ): { transform(response: Response): Response };
};

const NAMED = /^[0-9a-f]{64}$/;

/** The hexadecimal a name has to begin with before the protocol will count it.
 *
 *  A key is searched for until its name starts this way, at roughly four thousand tries.
 *  Checking it here does not make this server an authority on anything. It means a slot on
 *  the board costs what a name costs, which is the same thing it costs everybody else. */
const ELIGIBLE = "333";

export default {
  async fetch(request: Arriving, env: Env): Promise<Response> {
    const path = new URL(request.url).pathname;

    if (path === "/" || path === "/index.html") return theFrontPage(request, env);
    if (path === "/333/where") return whereYouAre(request);
    if (path === "/333/where-we-are") {
      return request.method === "GET" ? whereWeAre(env) : plain(405, "GET /333/where-we-are\n");
    }
    if (path === "/333") {
      return request.method === "GET" ? readTheBoard(request, env) : plain(405, "GET /333\n");
    }
    if (path.startsWith("/333/")) {
      return request.method === "PUT"
        ? speak(request, env, path.slice("/333/".length))
        : plain(405, "PUT /333/<node name in hex>\n");
    }
    return plain(404, "Nothing is kept at this address.\n");
  },
};

/** The one page that is not quite static.
 *
 *  Everything on it is written by hand except one number: how many statements are being held
 *  right now. Nobody edits that in, and nothing generates the page — the count is read out of
 *  the board on the way past, by the same code that serves the board to programs. A node
 *  arriving at the board changes what the next visitor reads, and no person is in the loop.
 *
 *  IT IS NOT A COUNT OF ANYBODY. It is how many signed statements this address is holding, and
 *  the page says so where it says the number. What that is worth is exactly what the reader can
 *  check, which from a browser is nothing, and the honest place to ask is a node of your own.
 *
 *  A board that cannot be read leaves the page as it was written, with a dash in it. A number
 *  nobody can produce is better missing than invented. */
async function theFrontPage(request: Request, env: Env): Promise<Response> {
  const page = await env.ASSETS.fetch(request);
  let saying: number;
  try {
    saying = alive((await held(env)).e).length;
  } catch {
    return page;
  }
  // Rebuilt rather than edited: the response the edge hands over has headers that cannot be
  // changed in place, and this one must not be cached with a number in it.
  const fresh = new Response(page.body, page);
  fresh.headers.set("cache-control", "no-store");
  return new HTMLRewriter()
    .on("[data-saying]", {
      element(element) {
        element.setInnerContent(String(saying));
      },
    })
    .transform(fresh);
}

/** Tell a caller the address this server saw it come from.
 *
 *  A node behind a household router knows the port it is listening on and has no way to learn
 *  the address the rest of the world would have to use to reach it. Everything else it needs
 *  in order to say where it is, it already has. */
function whereYouAre(request: Request): Response {
  return plain(200, `${request.headers.get("cf-connecting-ip") ?? ""}\n`);
}

/** Hand back every statement being held, newest last.
 *
 *  To a program, the body is the same shape as the logs the client already reads: a
 *  four-byte length in network order, then that many bytes, repeated. Everything is
 *  handed over, including anything that does not verify, because the client checks for
 *  itself and a server that filters is a server that can filter somebody out.
 *
 *  To a browser, the same board with the signatures checked here and the ones that fail
 *  left out, written as invitations a person can copy. Checking here proves nothing to
 *  anybody and is not meant to: it is so that a page for people is a page of things that
 *  are at least real statements by real keys. */
async function readTheBoard(request: Request, env: Env): Promise<Response> {
  const lines = alive((await held(env)).e);

  if ((request.headers.get("accept") ?? "").includes("text/html")) {
    const said: Said[] = [];
    for (const line of lines) {
      const one = await opened(unbase64(line.b));
      if (one !== null) said.push(one);
    }
    return page(said);
  }

  let total = 0;
  const frames = lines.map((line) => {
    const bytes = unbase64(line.b);
    total += 4 + bytes.length;
    return bytes;
  });
  const body = new Uint8Array(total);
  const header = new DataView(body.buffer);
  let at = 0;
  for (const frame of frames) {
    header.setUint32(at, frame.length);
    body.set(frame, at + 4);
    at += 4 + frame.length;
  }
  return new Response(body, {
    headers: { "content-type": "application/octet-stream", "cache-control": "no-store" },
  });
}

/** Nothing here is written by us, so nothing here goes into a page unescaped. */
function safe(text: string): string {
  return text.replace(/[&<>"]/g, (mark) => `&#${mark.charCodeAt(0)};`);
}

/** The board, for somebody with a browser rather than a client. */
function page(said: Said[]): Response {
  const rows =
    said.length === 0
      ? "<p>Nobody has left an address here in the last two epochs.</p>"
      : `<ul>${said
          .map(
            (one) =>
              `<li><code>333:${safe(one.address)}</code><span> said in epoch ${one.epoch}` +
              ` by ${safe(one.node.slice(0, 12))}</span></li>`,
          )
          .join("")}</ul>`;
  const body = `<!doctype html><html lang="en"><head><meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>333, where people are</title>
<link rel="icon" href="/favicon.svg" type="image/svg+xml">
<style>
:root{color-scheme:light dark;--g:#fff;--i:#141414;--d:#575757;--l:#e3e3e3;--r:#f5f5f5}
@media(prefers-color-scheme:dark){:root{--g:#0b0b0b;--i:#ededed;--d:#a6a6a6;--l:#262626;--r:#151515}}
body{margin:0;background:var(--g);color:var(--i);padding:2.5rem 1.25rem 5rem;
font:1rem/1.7 system-ui,-apple-system,"Segoe UI",Roboto,Arial,sans-serif;overflow-wrap:anywhere}
main{max-width:40rem;margin:0 auto}h1{font-size:2rem;font-weight:600;margin:0 0 1.5rem}
p{margin:0 0 1.1rem}ul{list-style:none;padding:0;margin:0 0 1.5rem}
li{border:1px solid var(--l);border-radius:14px;padding:.9rem 1.1rem;margin-bottom:.6rem;background:var(--r)}
code{font-family:ui-monospace,SFMono-Regular,Menlo,Consolas,monospace;font-size:.95em}
li span{display:block;color:var(--d);font-size:.85rem;margin-top:.35rem}
a{color:inherit;text-underline-offset:4px}.d{color:var(--d)}
</style></head><body><main>
<h1>Where people are</h1>
<p class="d">These are the addresses nodes have signed for themselves in the last two epochs.
Each signature was checked here, and your client checks it again before it believes any of
it. Nothing on this page hands over the file. The node at the far end decides that, and it
will want you to have been introduced.</p>
${rows}
<p class="d">To knock on one of them, run <code>333 join 333:address:port</code>. If you do
not have the client yet, it is at
<a href="https://github.com/needmoretruth/333">github.com/needmoretruth/333</a>.</p>
</main></body></html>`;
  return new Response(body, {
    headers: { "content-type": "text/html; charset=utf-8", "cache-control": "no-store" },
  });
}

/** Take one statement and put it in the slot its own key names.
 *
 *  THIS IS WHERE SPAM IS TURNED AWAY, and it is turned away by the protocol rather than by
 *  anybody deciding. Three things have to be true. The statement has to open and verify,
 *  so the bytes cannot be arbitrary. The key that signed it has to be the key the slot is
 *  named after, so nobody can write over somebody else, and nobody can hold slots they
 *  have no key for. And the name has to be one the protocol would count, which means it
 *  was searched for rather than generated, at about four thousand tries each.
 *
 *  None of that stops somebody determined. It stops a loop, which is what actually
 *  happens, and it costs a real node nothing it was not already paying. */
async function speak(request: Arriving, env: Env, key: string): Promise<Response> {
  if (!NAMED.test(key)) return plain(400, "The slot is a node name in lower-case hex.\n");

  const frame = new Uint8Array(await request.arrayBuffer());
  if (frame.length === 0 || frame.length > LONGEST) {
    return plain(413, `A statement is between 1 and ${LONGEST} bytes.\n`);
  }

  const standsUp = await opened(frame);
  if (standsUp === null) {
    return plain(400, "That is not a signed statement about where a node is.\n");
  }
  if (standsUp.node !== key) {
    return plain(403, "A statement goes in the slot named after the key that signed it.\n");
  }
  if (!key.startsWith(ELIGIBLE)) {
    return plain(403, `A name that 333 counts begins with ${ELIGIBLE}. This one does not.\n`);
  }

  const said = base64(frame);
  const board = await held(env);
  const lines = alive(board.e);
  const standing = lines.find((line) => line.k === key);
  // Byte for byte what is already there: the node is repeating itself, which costs nothing
  // and should stay costing nothing. Checked before the waiting, so that a node retrying
  // after a lost answer is never told to come back later for a write that already happened.
  if (standing?.b === said) return plain(200, "Already said.\n");

  if (await tooSoon(request)) {
    return plain(429, "Once an epoch is enough. Nothing here changes faster.\n");
  }

  const today = Math.floor(Date.now() / 86_400_000);
  const written = board.d === today ? board.w : 0;
  if (written >= MOST_WRITES_A_DAY) {
    return plain(503, "The board is full for today. It is still readable, and it empties at midnight.\n");
  }

  const kept = lines.filter((line) => line.k !== key);
  kept.push({ k: key, b: said, t: Date.now(), p: placeOf(request, standsUp.address) });
  const next: Board = { d: today, w: written + 1, e: kept.slice(-MOST) };
  await env.BOARD.put(BOARD, JSON.stringify({ v: 1, ...next }));
  return plain(200, "Said.\n");
}

/** Where the writer was, or `"tor"`, or nothing.
 *
 *  A statement naming an onion address is placed nowhere, whatever the edge says about the
 *  connection that carried it. That connection is the node's own address and this is the one
 *  place in the whole design where it would be written down.
 *
 *  Everything else is placed as coarsely as the map needs: whole degrees, which is roughly a
 *  hundred kilometres, and which is less than the address on the board already gives away to
 *  anybody who cares to look it up. */
function placeOf(request: Arriving, address: string): Place | undefined {
  if (hostOf(address).endsWith(".onion")) return "tor";
  const edge = request.cf;
  const country = edge?.country;
  if (typeof country !== "string" || !/^[A-Z]{2}$/.test(country)) return undefined;
  const y = Number(edge?.latitude);
  const x = Number(edge?.longitude);
  if (!Number.isFinite(y) || !Number.isFinite(x)) return undefined;
  if (y < -90 || y > 90 || x < -180 || x > 180) return undefined;
  return { c: country, y: Math.round(y), x: Math.round(x) };
}

/** The host out of a `host:port` address, leaving a bare host alone.
 *
 *  Split on the last colon so that an address written with a bracketed IPv6 host keeps its
 *  brackets and everything before them. Nothing here needs the port. */
function hostOf(address: string): string {
  const cut = address.lastIndexOf(":");
  const host = cut === -1 ? address : address.slice(0, cut);
  return host.toLowerCase();
}

/** How many of us are where, for the map. Names nobody: counts only.
 *
 *  It is served from the same board everything else is served from, so it is exactly as
 *  fresh, exactly as incomplete, and exactly as untrusted. A node that met somebody through
 *  an invitation and never came here is not in it, and neither is one that told this server
 *  nothing. It is what this one place saw, said as that and not as a census. */
async function whereWeAre(env: Env): Promise<Response> {
  const lines = alive((await held(env)).e);
  const counted = new Map<string, number>();
  const dots: Array<[number, number]> = [];
  let tor = 0;
  let unplaced = 0;
  for (const line of lines) {
    if (line.p === "tor") {
      tor += 1;
    } else if (line.p === undefined) {
      unplaced += 1;
    } else {
      counted.set(line.p.c, (counted.get(line.p.c) ?? 0) + 1);
      dots.push([line.p.x, line.p.y]);
    }
  }
  const countries = [...counted]
    .map(([c, n]) => ({ c, n }))
    .sort((one, two) => two.n - one.n || one.c.localeCompare(two.c));
  return json(200, { as_of: Date.now(), saying: lines.length, tor, unplaced, countries, dots });
}

/** Hand back a value as JSON, uncached: the board changes and a stale map is a wrong map. */
function json(status: number, body: unknown): Response {
  return new Response(JSON.stringify(body), {
    status,
    headers: {
      "content-type": "application/json; charset=utf-8",
      "cache-control": "no-store",
    },
  });
}

/** Whether this address has left a statement too recently to leave another.
 *
 *  WHY THIS AND NOT THE RATE LIMITER THE PLATFORM OFFERS. The binding was tried first and did
 *  not refuse anything, at eighteen writes in a minute against a limit of three, and a guard
 *  that cannot be shown to guard is not one. This can be shown from outside, which is the
 *  whole of the argument for it.
 *
 *  It is a doorstop rather than a lock. The memory is per location and can be swept away
 *  early, so a caller determined to get past it will. What it stops is the ordinary case of
 *  one machine writing in a loop, and behind it stands the day of writing, which is the thing
 *  actually worth defending. */
async function tooSoon(request: Request): Promise<boolean> {
  const from = request.headers.get("cf-connecting-ip");
  if (from === null) return false;
  // Keyed on this site so the memory stays where this Worker can reach it. Nothing answers at
  // that path: a request for it arrives here and is told there is nothing at this address.
  const gate = new Request(new URL(`/gate/${encodeURIComponent(from)}`, request.url).toString());
  const seen = await caches.default.match(gate);
  if (seen !== undefined) return true;
  await caches.default.put(
    gate,
    new Response("", { headers: { "cache-control": `max-age=${BETWEEN_WORDS}` } }),
  );
  return false;
}

/** The board as it stands, or an empty one if what is stored cannot be read as a board. */
async function held(env: Env): Promise<Board> {
  const empty: Board = { d: 0, w: 0, e: [] };
  const raw = await env.BOARD.get(BOARD, "text");
  if (raw === null) return empty;
  try {
    const stored = JSON.parse(raw) as Partial<Board>;
    return {
      d: typeof stored.d === "number" ? stored.d : 0,
      w: typeof stored.w === "number" ? stored.w : 0,
      e: Array.isArray(stored.e) ? stored.e : [],
    };
  } catch {
    return empty;
  }
}

/** The statements young enough to still be worth handing out. */
function alive(lines: Line[]): Line[] {
  const oldest = Date.now() - KEPT_FOR_MS;
  return lines.filter((line) => typeof line.t === "number" && line.t >= oldest);
}

function base64(bytes: Uint8Array): string {
  let out = "";
  for (const byte of bytes) out += String.fromCharCode(byte);
  return btoa(out);
}

function unbase64(text: string): Uint8Array {
  const raw = atob(text);
  const bytes = new Uint8Array(raw.length);
  for (let at = 0; at < raw.length; at += 1) bytes[at] = raw.charCodeAt(at);
  return bytes;
}

function plain(status: number, body: string): Response {
  return new Response(body, {
    status,
    headers: { "content-type": "text/plain; charset=utf-8", "cache-control": "no-store" },
  });
}
