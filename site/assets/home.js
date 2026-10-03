// The home page: the 3D first screen, the story, and the numbers that follow the network.

// Every module is named with this page's version, as the page names this one, so a
// release never pairs this file with another release's copy of the ones it uses.
const v = new URL(import.meta.url).search;
const { start, figures, shown, embedded, say } = await import(`./live.js${v}`);
const { story } = await import(`./story.js${v}`);

const hero = document.querySelector(".hero");
let first = null;

function numbers(net) {
  const f = figures(net);
  const heroLine = document.querySelector('[data-live="hero"]');
  if (heroLine) {
    // By name, not by place: a language may say the epoch before the count.
    const answering = heroLine.querySelector('[data-figure="answering"]');
    const epoch = heroLine.querySelector('[data-figure="epoch"]');
    if (answering) answering.textContent = shown(f.answering);
    if (epoch) epoch.textContent = shown(f.epoch);
  }
  const strip = document.querySelector('[data-live="figures"]');
  if (strip) {
    const answering = strip.querySelector('[data-figure="answering"]');
    const roll = strip.querySelector('[data-figure="roll"]');
    if (answering) answering.textContent = shown(f.answering);
    if (roll) roll.textContent = shown(f.roll);
  }
  if (first) first.update(net);
}

start(numbers);
story(document.querySelector(".story"), say);

// Three.js loads after the text is already on screen; if it cannot, the heading stays as text.
import(`./vigil.js${v}`)
  .then(({ vigil }) => vigil(hero.querySelector(".vigil"), hero, embedded()))
  .then((drawn) => {
    if (drawn) first = drawn;
  })
  .catch(() => {
    // No WebGL, or the module failed to load: the heading stays as text.
  });
