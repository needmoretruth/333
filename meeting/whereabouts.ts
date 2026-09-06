// Reading what a node signed about where it is.
//
// The server does not have to understand any of this. It could hold the bytes and hand
// them back, and the client would be no worse off, because the client checks every
// signature itself and believes nothing that arrives from here. Understanding them buys
// two things and no authority: a person with a browser can see an invitation instead of
// a length-prefixed blob, and a statement that does not verify never takes up a slot
// somebody real could have used.
//
// FROZEN, because it reads a frozen format. A whereabouts frame is a 64-byte signature
// followed by the postcard encoding of four fields, and what is signed is a 16-byte
// domain tag followed by that encoding. Change either and this stops reading anything.

/** The domain a whereabouts statement is signed under. Must match the client. */
const DOMAIN = new TextEncoder().encode("333.v1.where.iam");

/** Length of the signature at the front of every frame. */
const SIGNATURE = 64;

/** The wire version this understands. */
const PROTOCOL = 1;

/** The longest address that will be read out of a statement. */
const LONGEST_ADDRESS = 300;

/** What one node said about itself. */
export interface Said {
  /** The name the protocol derives from the key, in hex. */
  node: string;
  /** Where it said to look. */
  address: string;
  /** The epoch it said that in. */
  epoch: number;
}

/** Read one number in the encoding postcard writes integers in. */
function varint(bytes: Uint8Array, from: number): [number, number] | null {
  let value = 0;
  let shift = 0;
  let at = from;
  while (at < bytes.length && shift <= 56) {
    const byte = bytes[at];
    at += 1;
    value += (byte & 0x7f) * 2 ** shift;
    if ((byte & 0x80) === 0) return [value, at];
    shift += 7;
  }
  return null;
}

/** Take a frame apart without checking whether anybody stands behind it. */
function unpack(frame: Uint8Array): { key: Uint8Array; body: Uint8Array; said: Said } | null {
  if (frame.length <= SIGNATURE) return null;
  const body = frame.subarray(SIGNATURE);

  const protocol = varint(body, 0);
  if (protocol === null || protocol[0] !== PROTOCOL) return null;
  let at = protocol[1];

  if (at + 32 > body.length) return null;
  const key = body.subarray(at, at + 32);
  at += 32;

  const length = varint(body, at);
  if (length === null || length[0] > LONGEST_ADDRESS) return null;
  at = length[1];
  if (at + length[0] > body.length) return null;
  const address = new TextDecoder().decode(body.subarray(at, at + length[0]));
  at += length[0];

  const epoch = varint(body, at);
  // Trailing bytes are a different message wearing this one as a hat. Refused, the same
  // way the client refuses them.
  if (epoch === null || epoch[1] !== body.length) return null;

  return { key, body, said: { node: "", address, epoch: epoch[0] } };
}

/** The name the protocol derives from a public key. */
async function nameOf(key: Uint8Array): Promise<string> {
  const digest = await crypto.subtle.digest("SHA-256", key);
  return [...new Uint8Array(digest)].map((byte) => byte.toString(16).padStart(2, "0")).join("");
}

/** Read a frame, and hand it back only if the key it names really signed it. */
export async function opened(frame: Uint8Array): Promise<Said | null> {
  const parts = unpack(frame);
  if (parts === null) return null;
  const signed = new Uint8Array(DOMAIN.length + parts.body.length);
  signed.set(DOMAIN);
  signed.set(parts.body, DOMAIN.length);
  try {
    const key = await crypto.subtle.importKey("raw", parts.key, { name: "Ed25519" }, false, [
      "verify",
    ]);
    const stands = await crypto.subtle.verify(
      { name: "Ed25519" },
      key,
      frame.subarray(0, SIGNATURE),
      signed,
    );
    if (!stands) return null;
  } catch {
    // A key that is not a point on the curve, or a runtime without Ed25519. Either way
    // this cannot be shown to be true, so it is not passed on as though it were.
    return null;
  }
  return { ...parts.said, node: await nameOf(parts.key) };
}
