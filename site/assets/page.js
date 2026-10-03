// Reading pages: only the header's state and the copy buttons follow the site's node.
// live.js is named with this page's version, as the page names this file.
const { start } = await import(`./live.js${new URL(import.meta.url).search}`);

start();
