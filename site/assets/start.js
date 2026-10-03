// Take the program, narrowed to the machine this browser says it is on. Everything this shows is
// already on the page without it: it only picks one of the two installers, fills in the file
// name, and marks one row of the table. It sends nothing anywhere and keeps nothing.
(function () {
  var pick = document.getElementById("pick");
  var machine = document.getElementById("machine");
  var form = document.getElementById("form");
  var guessed = document.getElementById("guessed");
  if (!pick || !machine || !form || !guessed) return;

  // The names the release attaches, and nothing else.
  var MACHINES = {
    "linux-x86_64": { said: "Linux on x86-64", part: "x86_64-linux", family: "unix" },
    "linux-aarch64": { said: "Linux on 64-bit ARM", part: "aarch64-linux", family: "unix" },
    "linux-armv6": { said: "Linux on 32-bit ARM", part: "armv6-linux", family: "unix" },
    "macos-aarch64": { said: "a Mac with Apple silicon", part: "aarch64-macos", family: "unix" },
    "macos-x86_64": { said: "a Mac with an Intel chip", part: "x86_64-macos", family: "unix" },
    "windows-x86_64": { said: "Windows", part: "x86_64-windows.exe", family: "windows" }
  };

  function fileName(key, light) {
    return "333-" + (light ? "light-" : "") + MACHINES[key].part;
  }
  function each(selector, act) {
    var found = document.querySelectorAll(selector);
    for (var i = 0; i < found.length; i += 1) act(found[i]);
  }

  // What the user-agent string says. `sure` is false where it names the system and not
  // the processor: Safari on every Mac says Intel, and many Linux browsers say nothing.
  function fromAgent() {
    var ua = navigator.userAgent || "";
    var touch = navigator.maxTouchPoints || 0;
    if (/iPhone|iPad|iPod|Android/.test(ua) || (/Macintosh/.test(ua) && touch > 1)) return { phone: true };
    if (/Windows/.test(ua)) return { key: "windows-x86_64", sure: true };
    if (/Macintosh|Mac OS X/.test(ua)) return { key: "macos-aarch64", sure: false };
    if (/aarch64|arm64/i.test(ua)) return { key: "linux-aarch64", sure: true };
    if (/armv[67]/i.test(ua)) return { key: "linux-armv6", sure: true };
    if (/x86_64|amd64/i.test(ua)) return { key: "linux-x86_64", sure: true };
    if (/Linux|X11|CrOS/.test(ua)) return { key: "linux-x86_64", sure: false };
    return {};
  }

  // What User-Agent Client Hints say, where the browser offers them. Hints that name the
  // system and not the processor leave the user-agent string's guess standing.
  function fromHints(h) {
    var p = h.platform, a = h.architecture, bits = h.bitness;
    if (p === "Android" || p === "iOS") return { phone: true };
    if (p === "Windows") return { key: "windows-x86_64", sure: true };
    if (a !== "arm" && a !== "x86") return null;
    if (p === "macOS") return { key: a === "arm" ? "macos-aarch64" : "macos-x86_64", sure: true };
    if (p === "Linux" || p === "Chrome OS") {
      if (a === "arm") return { key: bits === "32" ? "linux-armv6" : "linux-aarch64", sure: true };
      return { key: "linux-x86_64", sure: true };
    }
    return null;
  }

  function tell(seen) {
    var text;
    if (seen.phone) {
      text = "This looks like a phone or a tablet, and the program is for a computer that " +
        "stays on. Choose that computer here.";
    } else if (!seen.key) {
      text = "This browser does not say what it is running on. Choose your machine here.";
    } else if (seen.sure) {
      text = "This browser says it is on " + MACHINES[seen.key].said + ", so that is chosen here.";
    } else if (seen.key.indexOf("macos") === 0) {
      text = "This browser says it is on a Mac and not which chip, so Apple silicon is chosen " +
        "here. The installer asks the machine itself.";
    } else {
      text = "This browser says it is on Linux and not which processor, so x86-64 is chosen " +
        "here. The installer asks the machine itself.";
    }
    guessed.textContent = text;
    if (seen.key && !touched) machine.value = seen.key;
    show();
  }

  function show() {
    var key = machine.value;
    var light = form.value === "light";
    var family = MACHINES[key].family;
    var chosen = fileName(key, light);
    each('[data-name="standard"]', function (el) { el.textContent = fileName(key, false); });
    each('[data-name="light"]', function (el) { el.textContent = fileName(key, true); });
    each('[data-name="unix"], [data-name="windows"]', function (el) { el.textContent = chosen; });
    each("[data-sum]", function (el) {
      el.textContent = key.indexOf("macos") === 0 ? "shasum -a 256" : "sha256sum";
    });
    each("[data-sh]", function (el) { el.textContent = light ? "sh -s -- --light" : "sh"; });
    each("[data-ps]", function (el) {
      el.textContent = light
        ? "& ([scriptblock]::Create((irm https://the333.dev/install.ps1))) -Light"
        : "irm https://the333.dev/install.ps1 | iex";
    });
    each("[data-for]", function (el) { el.hidden = el.getAttribute("data-for") !== family; });
    each("[data-hint-light], [data-hint-mac]", function (el) { el.hidden = true; });
    each("tr[data-row] .guess", function (el) { el.parentNode.removeChild(el); });
    each('tr[data-row="' + key + '"] th', function (el) {
      var mark = document.createElement("span");
      mark.className = "guess";
      mark.textContent = "Chosen above";
      el.appendChild(mark);
    });
  }

  var touched = false;
  machine.addEventListener("change", function () { touched = true; show(); });
  form.addEventListener("change", show);
  pick.hidden = false;
  tell(fromAgent());

  var hints = navigator.userAgentData;
  if (hints && typeof hints.getHighEntropyValues === "function") {
    hints.getHighEntropyValues(["architecture", "bitness"]).then(function (h) {
      var seen = fromHints({ platform: hints.platform, architecture: h.architecture, bitness: h.bitness });
      if (seen) tell(seen);
    }, function () {});
  }
})();
