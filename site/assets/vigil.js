// The first screen: the numerals 333 made of points, and around them the nodes this site's
// node has actually seen, as lights on an orbit. Nothing here is invented: an empty network
// draws an empty orbit. Scrolling gathers every point into one light, which the story below
// picks up as its first picture.

import * as THREE from "/assets/vendor/three-0.186.1/three.module.js";

const VERTEX = /* glsl */ `
  attribute float seed;
  attribute float size;
  attribute float bright;
  uniform float time;
  uniform float gather;
  uniform float ratio;
  uniform float still;
  varying float vBright;
  void main() {
    float delay = seed * 0.35;
    float g = clamp((gather - delay) / (1.0 - delay * 0.9), 0.0, 1.0);
    g = g * g * (3.0 - 2.0 * g);
    vec3 drift = vec3(sin(time * 0.55 + seed * 40.0), cos(time * 0.43 + seed * 31.0), sin(time * 0.37 + seed * 17.0)) * 0.018 * (1.0 - still);
    vec3 p = mix(position + drift, vec3(0.0), g);
    vec4 mv = modelViewMatrix * vec4(p, 1.0);
    gl_Position = projectionMatrix * mv;
    gl_PointSize = size * ratio * (9.0 / -mv.z) * (1.0 + g * 1.5);
    vBright = bright;
  }
`;

const FRAGMENT = /* glsl */ `
  uniform vec3 colour;
  uniform float opacity;
  varying float vBright;
  void main() {
    float d = length(gl_PointCoord - 0.5);
    float a = smoothstep(0.5, 0.0, d);
    gl_FragColor = vec4(colour, a * a * opacity * vBright);
  }
`;

const reduced = matchMedia("(prefers-reduced-motion: reduce)");
const light = matchMedia("(prefers-color-scheme: light)");

/** Sample the filled pixels of "333" set in the display face. */
async function numeralPoints(count) {
  try {
    await document.fonts.load('800 260px "Fraunces"');
  } catch {
    // Without the face the system serif draws the same three digits.
  }
  const w = 1400;
  const h = 560;
  const canvas = document.createElement("canvas");
  canvas.width = w;
  canvas.height = h;
  const ctx = canvas.getContext("2d", { willReadFrequently: true });
  ctx.fillStyle = "#fff";
  ctx.textAlign = "center";
  ctx.textBaseline = "middle";
  ctx.font = '800 400px "Fraunces", Georgia, serif';
  ctx.fillText("333", w / 2, h / 2 + 10);
  const data = ctx.getImageData(0, 0, w, h).data;
  const filled = [];
  for (let y = 0; y < h; y += 1) {
    for (let x = 0; x < w; x += 1) {
      if (data[(y * w + x) * 4 + 3] > 140) filled.push(x, y);
    }
  }
  const pairs = filled.length / 2;
  // Scale by the drawn digits themselves, so the numerals are WIDTH units wide wherever they are set.
  let x0 = w, x1 = 0, y0 = h, y1 = 0;
  for (let k = 0; k < filled.length; k += 2) {
    x0 = Math.min(x0, filled[k]);
    x1 = Math.max(x1, filled[k]);
    y0 = Math.min(y0, filled[k + 1]);
    y1 = Math.max(y1, filled[k + 1]);
  }
  const scale = WIDTH / Math.max(1, x1 - x0);
  const mx = (x0 + x1) / 2;
  const my = (y0 + y1) / 2;
  const out = new Float32Array(count * 3);
  for (let i = 0; i < count; i++) {
    const k = Math.floor(Math.random() * pairs) * 2;
    out[i * 3] = (filled[k] - mx + Math.random()) * scale;
    out[i * 3 + 1] = -(filled[k + 1] - my + Math.random()) * scale;
    out[i * 3 + 2] = (Math.random() - 0.5) * 0.22;
  }
  return { points: out, height: (y1 - y0) * scale };
}

function material(ratio) {
  return new THREE.ShaderMaterial({
    vertexShader: VERTEX,
    fragmentShader: FRAGMENT,
    transparent: true,
    depthWrite: false,
    blending: light.matches ? THREE.NormalBlending : THREE.AdditiveBlending,
    uniforms: {
      time: { value: 0 },
      gather: { value: 0 },
      ratio: { value: ratio },
      still: { value: reduced.matches ? 1 : 0 },
      opacity: { value: 1 },
      colour: { value: new THREE.Color(light.matches ? 0x121316 : 0xf3f4f6) },
    },
  });
}

function cloud(positions, sizeOf, brightOf, mat) {
  const n = positions.length / 3;
  const seed = new Float32Array(n);
  const size = new Float32Array(n);
  const bright = new Float32Array(n);
  for (let i = 0; i < n; i++) {
    seed[i] = Math.random();
    size[i] = sizeOf(i);
    bright[i] = brightOf(i);
  }
  const geometry = new THREE.BufferGeometry();
  geometry.setAttribute("position", new THREE.BufferAttribute(positions, 3));
  geometry.setAttribute("seed", new THREE.BufferAttribute(seed, 1));
  geometry.setAttribute("size", new THREE.BufferAttribute(size, 1));
  geometry.setAttribute("bright", new THREE.BufferAttribute(bright, 1));
  return new THREE.Points(geometry, mat);
}

const ORBIT = 4.3;
/** How wide the numerals are, in scene units. */
const WIDTH = 4.6;

/** Lights for the nodes this site's node has seen, on one tilted orbit. */
function nodeLights(net, mat) {
  const nodes = (net && Array.isArray(net.nodes) ? net.nodes : []).slice(0, 333);
  const n = nodes.length;
  const positions = new Float32Array(Math.max(n, 1) * 3);
  nodes.forEach((node, i) => {
    const a = (i / Math.max(n, 1)) * Math.PI * 2 + 0.6;
    positions[i * 3] = Math.cos(a) * ORBIT;
    positions[i * 3 + 1] = Math.sin(a) * ORBIT * 0.32;
    positions[i * 3 + 2] = Math.sin(a) * ORBIT * 0.5;
  });
  const points = cloud(
    positions.subarray(0, n * 3),
    (i) => (nodes[i].founder ? 30 : 18),
    (i) => (nodes[i].answered_now || nodes[i].founder ? 1 : 0.45),
    mat,
  );
  points.userData.count = n;
  return points;
}

function orbitLine() {
  const curve = [];
  for (let i = 0; i <= 160; i++) {
    const a = (i / 160) * Math.PI * 2;
    curve.push(new THREE.Vector3(Math.cos(a) * ORBIT, Math.sin(a) * ORBIT * 0.32, Math.sin(a) * ORBIT * 0.5));
  }
  const geometry = new THREE.BufferGeometry().setFromPoints(curve);
  const lineMat = new THREE.LineBasicMaterial({
    color: light.matches ? 0x121316 : 0xffffff,
    transparent: true,
    opacity: light.matches ? 0.14 : 0.12,
  });
  return new THREE.Line(geometry, lineMat);
}

/** Start the first screen. Returns false when this device cannot draw it. */
export async function vigil(canvas, hero, net) {
  let renderer;
  try {
    renderer = new THREE.WebGLRenderer({ canvas, antialias: true, alpha: true, powerPreference: "low-power" });
  } catch {
    return false;
  }
  const small = matchMedia("(max-width: 760px)").matches;
  const ratio = Math.min(devicePixelRatio || 1, 2);
  renderer.setPixelRatio(ratio);
  const scene = new THREE.Scene();
  const camera = new THREE.PerspectiveCamera(32, 1, 0.1, 100);
  camera.position.set(0, 0, small ? 15.5 : 12);

  // The numerals only lean towards the pointer; the orbit and its lights turn around them.
  const world = new THREE.Group();
  const orbit = new THREE.Group();
  orbit.rotation.x = 0.08;
  world.add(orbit);
  scene.add(world);
  const digits = await numeralPoints(small ? 12000 : 24000);
  const digitMat = material(ratio);
  world.add(cloud(digits.points, () => 1.3 + Math.random() * 1.5, () => 0.5 + Math.random() * 0.5, digitMat));
  const ring = orbitLine();
  orbit.add(ring);
  const nodeMat = material(ratio);
  let lights = nodeLights(net, nodeMat);
  orbit.add(lights);
  document.documentElement.classList.add("has-3d");

  const mark = hero.querySelector(".hero-mark");
  const resize = () => {
    const w = canvas.clientWidth;
    const h = canvas.clientHeight;
    renderer.setSize(w, h, false);
    camera.aspect = w / h;
    camera.updateProjectionMatrix();
    // Put the numerals where the heading's box is, so they never sit on the sentence below.
    const box = mark.getBoundingClientRect();
    const own = canvas.getBoundingClientRect();
    const perPixel = (2 * Math.tan((camera.fov * Math.PI) / 360) * camera.position.z) / h;
    world.position.y = (own.top + h / 2 - (box.top + box.height / 2)) * perPixel;
    const fit = Math.min(1.25, (Math.min(w * 0.62, 560) * perPixel) / WIDTH, (box.height * 0.82 * perPixel) / digits.height);
    world.scale.setScalar(fit);
  };
  resize();
  addEventListener("resize", resize);

  let aimX = 0;
  let aimY = 0;
  let spin = 0;
  let pull = 0;
  let dragging = null;
  canvas.addEventListener("pointermove", (e) => {
    const r = canvas.getBoundingClientRect();
    aimX = ((e.clientX - r.left) / r.width - 0.5) * 2;
    aimY = ((e.clientY - r.top) / r.height - 0.5) * 2;
    if (dragging !== null) {
      pull = (e.clientX - dragging) * 0.004;
      dragging = e.clientX;
    }
  });
  canvas.addEventListener("pointerdown", (e) => {
    if (e.pointerType === "mouse") dragging = e.clientX;
  });
  addEventListener("pointerup", () => {
    dragging = null;
  });

  const text = hero.querySelector(".hero-text");
  const foot = hero.querySelector(".hero-foot");
  let gathered = 0;
  let visible = true;
  const onScroll = () => {
    const span = hero.offsetHeight * 0.85;
    gathered = Math.min(1, Math.max(0, scrollY / span));
    const fade = String(Math.max(0, 1 - gathered * 1.6));
    text.style.opacity = fade;
    foot.style.opacity = fade;
    if (reduced.matches && visible) requestAnimationFrame(frame);
  };
  addEventListener("scroll", onScroll, { passive: true });
  onScroll();

  new IntersectionObserver(([entry]) => {
    visible = entry.isIntersecting;
    if (visible) requestAnimationFrame(frame);
  }).observe(hero);

  const clock = new THREE.Timer();
  function frame() {
    clock.update();
    const t = clock.getElapsed();
    const still = reduced.matches;
    if (!still) {
      spin += 0.0016 + pull;
      pull *= 0.92;
    }
    world.rotation.y += ((still ? 0 : aimX * 0.22) - world.rotation.y) * 0.06;
    world.rotation.x += ((still ? 0 : aimY * 0.12) - world.rotation.x) * 0.06;
    orbit.rotation.y = spin;
    const gone = 1 - Math.min(1, Math.max(0, (gathered - 0.5) / 0.25));
    for (const m of [digitMat, nodeMat]) {
      m.uniforms.time.value = still ? 0 : t;
      m.uniforms.gather.value = gathered;
    }
    digitMat.uniforms.opacity.value = gone;
    nodeMat.uniforms.opacity.value = gone * (still ? 1 : 0.82 + Math.sin(t * 3.1) * 0.18);
    ring.material.opacity = (light.matches ? 0.14 : 0.12) * (1 - gathered);
    renderer.render(scene, camera);
    if (visible && !still) requestAnimationFrame(frame);
  }
  requestAnimationFrame(frame);
  reduced.addEventListener("change", () => requestAnimationFrame(frame));

  return {
    update(next) {
      orbit.remove(lights);
      lights.geometry.dispose();
      lights = nodeLights(next, nodeMat);
      orbit.add(lights);
      if (reduced.matches) requestAnimationFrame(frame);
    },
  };
}
