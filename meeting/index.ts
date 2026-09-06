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

/** One node's statement about where it is. */
interface Line {
  /** The public key the statement is signed with, in hex — a slot name, not a claim. A liar
   *  can write into somebody else's slot and gains nothing by it: the bytes underneath are
   *  signed, so the worst that is achieved is one wasted slot. */
  k: string;
  /** The frame itself, base64 of the exact bytes the node signed. */
  b: string;
  /** When this server received it, in milliseconds. Used only to forget. */
  t: number;
}

/** The parts of the Workers runtime this file uses, and no more. */
interface Store {
  get(key: string, type: "text"): Promise<string | null>;
  put(key: string, value: string): Promise<void>;
}
interface Limiter {
  limit(of: { key: string }): Promise<{ success: boolean }>;
}
interface Env {
  BOARD: Store;
  SPEAKING: Limiter;
}

const NAMED = /^[0-9a-f]{64}$/;

export default {
  async fetch(request: Request, env: Env): Promise<Response> {
    const path = new URL(request.url).pathname;

    if (path === "/meet/where") return whereYouAre(request);
    if (path === "/meet") {
      return request.method === "GET" ? readTheBoard(request, env) : plain(405, "GET /meet\n");
    }
    if (path.startsWith("/meet/")) {
      return request.method === "PUT"
        ? speak(request, env, path.slice("/meet/".length))
        : plain(405, "PUT /meet/<key in hex>\n");
    }
    return plain(404, "Nothing is kept at this address.\n");
  },
};

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
 *  The body is the same shape as the logs the client already reads: a four-byte length in
 *  network order, then that many bytes, repeated. Nothing here says which of them verify. */
async function readTheBoard(request: Request, env: Env): Promise<Response> {
  const lines = alive(await held(env));

  if ((request.headers.get("accept") ?? "").includes("text/html")) {
    return plain(
      200,
      `${lines.length} node${lines.length === 1 ? " is" : "s are"} saying where they can be ` +
        `reached.\n\nThis address answers a program, not a browser. The program is at\n` +
        `https://github.com/needmoretruth/333\n`,
    );
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

/** Take one statement and put it in the slot its key names. */
async function speak(request: Request, env: Env, key: string): Promise<Response> {
  if (!NAMED.test(key)) return plain(400, "The slot is a public key in lower-case hex.\n");

  const from = request.headers.get("cf-connecting-ip") ?? "";
  const { success } = await env.SPEAKING.limit({ key: from });
  if (!success) return plain(429, "Once an epoch is enough. Nothing here changes faster.\n");

  const frame = new Uint8Array(await request.arrayBuffer());
  if (frame.length === 0 || frame.length > LONGEST) {
    return plain(413, `A statement is between 1 and ${LONGEST} bytes.\n`);
  }

  const said = base64(frame);
  const lines = alive(await held(env));
  const standing = lines.find((line) => line.k === key);
  // Byte for byte what is already there: the node is repeating itself, which costs nothing
  // and should stay costing nothing.
  if (standing?.b === said) return plain(200, "Already said.\n");

  const kept = lines.filter((line) => line.k !== key);
  kept.push({ k: key, b: said, t: Date.now() });
  await env.BOARD.put(BOARD, JSON.stringify({ v: 1, e: kept.slice(-MOST) }));
  return plain(200, "Said.\n");
}

/** Everything on the board, or nothing if it cannot be read as a board. */
async function held(env: Env): Promise<Line[]> {
  const raw = await env.BOARD.get(BOARD, "text");
  if (raw === null) return [];
  try {
    const board: unknown = JSON.parse(raw);
    const lines = (board as { e?: unknown }).e;
    return Array.isArray(lines) ? (lines as Line[]) : [];
  } catch {
    return [];
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
