// The home page: the 3D first screen, the story, and the numbers that follow the network.

import { start, figures, shown, embedded } from "./live.js";
import { story } from "./story.js";

const hero = document.querySelector(".hero");
let first = null;

function numbers(net) {
  const f = figures(net);
  const heroLine = document.querySelector('[data-live="hero"]');
  if (heroLine) {
    const [answering, epoch] = heroLine.querySelectorAll("span");
    answering.textContent = shown(f.answering);
    epoch.textContent = shown(f.epoch);
  }
  const dds = document.querySelectorAll('[data-live="figures"] dd');
  if (dds.length >= 3) {
    dds[0].textContent = shown(f.answering);
    dds[1].textContent = shown(f.roll);
    dds[2].textContent = shown(f.epoch);
  }
  if (first) first.update(net);
}

start(numbers);
story(document.querySelector(".story"));

// Three.js loads after the text is already on screen; if it cannot, the heading stays as text.
import("./vigil.js")
  .then(({ vigil }) => vigil(hero.querySelector(".vigil"), hero, embedded()))
  .then((drawn) => {
    if (drawn) first = drawn;
  })
  .catch(() => {
    // No WebGL, or the module failed to load: the heading stays as text.
  });
