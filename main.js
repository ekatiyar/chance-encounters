// Chance Encounters — front-end UI controller.
const $ = (id) => document.getElementById(id);

const els = {
  uploadView: $("upload-view"),
  resultsView: $("results-view"),
  fileA: $("file-a"),
  fileB: $("file-b"),
  analyzeBtn: $("analyze-btn"),
  progress: $("progress"),
  progressFill: $("progress-fill"),
  progressLabel: $("progress-label"),
  error: $("error"),
  items: $("encounter-items"),
  stats: $("stats"),
  timeline: $("timeline"),
  modeTabs: [...document.querySelectorAll(".mode-tab")],
  modePanels: [...document.querySelectorAll(".mode-panel")],
  speedPresets: $("speed-presets"),
  winDistance: $("win-distance"),
  winTime: $("win-time"),
  winTimeUnit: $("win-time-unit"),
  groupDist: $("group-dist"),
  groupTime: $("group-time"),
};

const MAP_ZOOM_THRESHOLD = 12; // at/above this, map shows colored pairs
const TIMELINE_DETAIL_SPAN_MS = 3 * 24 * 3600 * 1000; // visible span under this => colored

const state = {
  encounters: [],
  colors: [],
  activeIndex: null,
  map: null,
  collapsedLayer: null,
  expandedLayer: null,
  mapMarkers: [], // { line, a, b } per encounter
  tl: null, // timeline view state
  mode: "window", // "window" | "speed"
  speedKmh: 100, // active speed preset (driving)
};

// ---------- Upload flow ----------
function updateButton() {
  els.analyzeBtn.disabled = !(els.fileA.files[0] && els.fileB.files[0]);
}
els.fileA.addEventListener("change", updateButton);
els.fileB.addEventListener("change", updateButton);

// ---------- Search config ----------
els.modeTabs.forEach((tab) => {
  tab.addEventListener("click", () => {
    state.mode = tab.dataset.mode;
    els.modeTabs.forEach((t) => t.classList.toggle("active", t === tab));
    els.modePanels.forEach((p) => p.classList.toggle("hidden", p.dataset.panel !== state.mode));
  });
});

els.speedPresets.addEventListener("click", (e) => {
  const btn = e.target.closest("button[data-kmh]");
  if (!btn) return;
  state.speedKmh = Number(btn.dataset.kmh);
  [...els.speedPresets.children].forEach((b) => b.classList.toggle("active", b === btn));
});

// Read the active controls into the options object sent to the wasm core.
function buildOptions() {
  const opts = {
    mode: state.mode,
    spaceEpsKm: Number(els.groupDist.value) || 0,
    timeEpsSecs: (Number(els.groupTime.value) || 0) * 3600,
  };
  if (state.mode === "speed") {
    opts.speedKmh = state.speedKmh;
  } else {
    opts.spatialCapKm = Number(els.winDistance.value) || 0;
    opts.timeWindowSecs = (Number(els.winTime.value) || 0) * Number(els.winTimeUnit.value);
  }
  return opts;
}

function setProgress(pct, label) {
  els.progressFill.style.width = `${Math.max(0, Math.min(100, pct))}%`;
  if (label) {
    els.progressLabel.textContent = label;
  } else if (pct >= 100) {
    els.progressLabel.textContent = "Rendering…";
  } else if (pct >= 65) {
    els.progressLabel.textContent = "Searching for encounters…";
  } else if (pct >= 30) {
    els.progressLabel.textContent = "Parsing location histories…";
  } else {
    els.progressLabel.textContent = "Starting…";
  }
}

function showError(message) {
  els.progress.classList.add("hidden");
  els.error.textContent = message;
  els.error.classList.remove("hidden");
  els.analyzeBtn.disabled = false;
}

els.analyzeBtn.addEventListener("click", async () => {
  els.error.classList.add("hidden");
  els.progress.classList.remove("hidden");
  els.analyzeBtn.disabled = true;
  setProgress(0, "Reading files…");

  let f1, f2;
  try {
    [f1, f2] = await Promise.all([els.fileA.files[0].text(), els.fileB.files[0].text()]);
  } catch (e) {
    return showError(`Could not read files: ${e}`);
  }

  const worker = new Worker("./worker.js", { type: "module" });
  worker.onmessage = (event) => {
    const msg = event.data;
    if (msg.type === "progress") {
      setProgress(msg.pct);
    } else if (msg.type === "result") {
      worker.terminate();
      onResult(msg.result);
    } else if (msg.type === "error") {
      worker.terminate();
      showError(msg.message);
    }
  };
  worker.onerror = (e) => {
    worker.terminate();
    showError(e.message || "Worker failed to start");
  };
  worker.postMessage({ file1: f1, file2: f2, opts: buildOptions() });
});

// ---------- Result handling ----------
function onResult(result) {
  const encounters = result.encounters || [];
  const n = Math.max(encounters.length, 1);
  state.encounters = encounters;
  state.colors = encounters.map((_, i) => `hsl(${Math.round((i * 360) / n)}, 75%, 60%)`);
  state.stats = result.stats;
  transitionToResults();
}

function transitionToResults() {
  els.uploadView.classList.add("fade-out");
  setTimeout(() => {
    els.uploadView.classList.add("hidden");
    els.resultsView.classList.remove("hidden");
    els.resultsView.classList.add("fade-in");

    renderStats();
    renderList();
    initMap();
    buildMapLayers();
    updateMapView();
    initTimeline();
    renderTimeline();

    // Leaflet must recompute size now that the container is visible.
    setTimeout(() => state.map && state.map.invalidateSize(), 60);
  }, 400);
}

// ---------- Formatting ----------
function fmtDistance(km) {
  return km < 1 ? `${Math.round(km * 1000)} m` : `${km.toFixed(2)} km`;
}

function fmtGap(secs) {
  const s = Math.round(Math.abs(secs));
  if (s === 0) return "overlapping";
  if (s < 60) return `${s}s apart`;
  if (s < 3600) return `${Math.round(s / 60)}m apart`;
  if (s < 86400) return `${(s / 3600).toFixed(1)}h apart`;
  return `${(s / 86400).toFixed(1)}d apart`;
}

function fmtDate(ms) {
  return new Date(ms).toLocaleString(undefined, {
    year: "numeric",
    month: "short",
    day: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  });
}

function midpoint(enc) {
  return [(enc.a.lat + enc.b.lat) / 2, (enc.a.lon + enc.b.lon) / 2];
}

// ---------- List ----------
function renderStats() {
  const s = state.stats || {};
  els.stats.textContent = `${s.clusters ?? 0} events · ${(s.points_a ?? 0).toLocaleString()} + ${(
    s.points_b ?? 0
  ).toLocaleString()} points`;
}

function renderList() {
  els.items.innerHTML = "";
  if (state.encounters.length === 0) {
    const li = document.createElement("li");
    li.className = "empty";
    li.textContent = "No encounters found within the search window.";
    els.items.appendChild(li);
    return;
  }

  state.encounters.forEach((enc, i) => {
    const li = document.createElement("li");
    li.className = "encounter-item";
    li.style.setProperty("--color", state.colors[i]);
    li.dataset.index = i;
    li.innerHTML = `
      <span class="encounter-rank">${enc.rank}</span>
      <span class="encounter-date">${fmtDate(enc.a.start)}</span>
      ${enc.cluster_size > 1 ? `<span class="badge">×${enc.cluster_size}</span>` : ""}
      <span class="encounter-metrics"><b>${fmtDistance(enc.distance_km)}</b> · ${fmtGap(enc.time_gap_secs)}</span>
    `;
    li.addEventListener("mouseenter", () => setActive(i));
    li.addEventListener("click", () => setActive(i));
    els.items.appendChild(li);
  });

  els.items.addEventListener("mouseleave", () => setActive(null));
}

function setActive(index) {
  state.activeIndex = index;
  [...els.items.children].forEach((li) => {
    li.classList.toggle("active", Number(li.dataset.index) === index);
  });
  updateMapView();
  if (index != null) {
    focusMap(index);
    focusTimeline(index);
  }
  renderTimeline();
}

// ---------- Map ----------
function initMap() {
  if (state.map) return;
  state.map = L.map("map", { zoomControl: true, worldCopyJump: true }).setView([20, 0], 2);
  L.tileLayer("https://tile.openstreetmap.org/{z}/{x}/{y}.png", {
    maxZoom: 19,
    attribution: "© OpenStreetMap contributors",
  }).addTo(state.map);
  state.map.on("zoomend", updateMapView);
}

function buildMapLayers() {
  state.collapsedLayer = L.layerGroup();
  state.expandedLayer = L.layerGroup();
  state.mapMarkers = [];

  const bounds = [];
  state.encounters.forEach((enc, i) => {
    const color = state.colors[i];
    const a = [enc.a.lat, enc.a.lon];
    const b = [enc.b.lat, enc.b.lon];
    bounds.push(a, b);

    // Collapsed: a single neutral dot per encounter.
    L.circleMarker(midpoint(enc), {
      radius: 5,
      color: "#8b98a5",
      weight: 1,
      fillColor: "#8b98a5",
      fillOpacity: 0.85,
    })
      .bindTooltip(`#${enc.rank}`, { direction: "top" })
      .on("mouseover", () => setActive(i))
      .addTo(state.collapsedLayer);

    // Expanded: colored A + B markers joined by a line.
    const line = L.polyline([a, b], { color, weight: 2, opacity: 0.6, dashArray: "4 4" });
    const ma = L.circleMarker(a, { radius: 6, color, weight: 2, fillColor: color, fillOpacity: 0.95 }).bindTooltip(
      `#${enc.rank} · A`,
      { direction: "top" }
    );
    const mb = L.circleMarker(b, { radius: 6, color, weight: 2, fillColor: color, fillOpacity: 0.4 }).bindTooltip(
      `#${enc.rank} · B`,
      { direction: "top" }
    );
    [line, ma, mb].forEach((layer) => {
      layer.on("mouseover", () => setActive(i));
      layer.addTo(state.expandedLayer);
    });
    state.mapMarkers.push({ line, a: ma, b: mb });
  });

  if (bounds.length) {
    state.map.fitBounds(L.latLngBounds(bounds).pad(0.2));
  }
}

function updateMapView() {
  if (!state.map) return;
  const zoomedIn = state.map.getZoom() >= MAP_ZOOM_THRESHOLD;
  const showExpanded = zoomedIn || state.activeIndex != null;

  if (showExpanded) {
    if (!state.map.hasLayer(state.expandedLayer)) state.expandedLayer.addTo(state.map);
    if (state.map.hasLayer(state.collapsedLayer)) state.map.removeLayer(state.collapsedLayer);
  } else {
    if (!state.map.hasLayer(state.collapsedLayer)) state.collapsedLayer.addTo(state.map);
    if (state.map.hasLayer(state.expandedLayer)) state.map.removeLayer(state.expandedLayer);
  }

  // Emphasize the active pair.
  state.mapMarkers.forEach((m, i) => {
    const active = i === state.activeIndex;
    const r = active ? 9 : 6;
    m.a.setRadius(r);
    m.b.setRadius(r);
    m.line.setStyle({ weight: active ? 4 : 2, opacity: active ? 0.9 : 0.6 });
    if (active) {
      m.line.bringToFront();
      m.a.bringToFront();
      m.b.bringToFront();
    }
  });
}

function focusMap(index) {
  const enc = state.encounters[index];
  const b = L.latLngBounds([
    [enc.a.lat, enc.a.lon],
    [enc.b.lat, enc.b.lon],
  ]).pad(0.6);
  state.map.flyToBounds(b, { maxZoom: 15, duration: 0.5 });
}

// ---------- Timeline (custom canvas) ----------
function timelineDomain() {
  let min = Infinity;
  let max = -Infinity;
  for (const enc of state.encounters) {
    min = Math.min(min, enc.a.start, enc.b.start);
    max = Math.max(max, enc.a.end, enc.b.end);
  }
  if (!isFinite(min)) {
    min = Date.now();
    max = min + 86400000;
  }
  const pad = Math.max((max - min) * 0.05, 3600000);
  return { min: min - pad, max: max + pad };
}

function initTimeline() {
  const canvas = els.timeline;
  const domain = timelineDomain();
  // scale in px per ms so the whole domain fits the current width.
  const width = canvas.clientWidth || 800;
  state.tl = {
    domain,
    offset: domain.min, // time at left edge
    scale: width / (domain.max - domain.min),
    dragging: false,
    lastX: 0,
  };

  if (!canvas.dataset.wired) {
    canvas.dataset.wired = "1";
    canvas.addEventListener("wheel", onTimelineWheel, { passive: false });
    canvas.addEventListener("mousedown", (e) => {
      state.tl.dragging = true;
      state.tl.lastX = e.clientX;
    });
    window.addEventListener("mouseup", () => (state.tl.dragging = false));
    window.addEventListener("mousemove", (e) => {
      if (!state.tl.dragging) return;
      const dx = e.clientX - state.tl.lastX;
      state.tl.lastX = e.clientX;
      state.tl.offset -= dx / state.tl.scale;
      renderTimeline();
    });
    canvas.addEventListener("mousemove", onTimelineHover);
    const ro = new ResizeObserver(() => renderTimeline());
    ro.observe(canvas);
  }
}

function onTimelineWheel(e) {
  e.preventDefault();
  const rect = els.timeline.getBoundingClientRect();
  const cursorX = e.clientX - rect.left;
  const cursorTime = state.tl.offset + cursorX / state.tl.scale;
  const factor = e.deltaY < 0 ? 1.15 : 1 / 1.15;
  state.tl.scale *= factor;
  // keep the time under the cursor fixed
  state.tl.offset = cursorTime - cursorX / state.tl.scale;
  renderTimeline();
}

function onTimelineHover(e) {
  const rect = els.timeline.getBoundingClientRect();
  const x = e.clientX - rect.left;
  const y = e.clientY - rect.top;
  const hit = timelineHitTest(x, y);
  if (hit != null && hit !== state.activeIndex) setActive(hit);
}

function timeToX(t) {
  return (t - state.tl.offset) * state.tl.scale;
}

function laneY(canvasH, lane) {
  return lane === 0 ? canvasH * 0.38 : canvasH * 0.72;
}

function timelineHitTest(x, y) {
  const h = els.timeline.clientHeight;
  for (let i = 0; i < state.encounters.length; i++) {
    const enc = state.encounters[i];
    const points = [
      { t: (enc.a.start + enc.a.end) / 2, lane: 0 },
      { t: (enc.b.start + enc.b.end) / 2, lane: 1 },
    ];
    for (const p of points) {
      const px = timeToX(p.t);
      const py = laneY(h, p.lane);
      if (Math.hypot(px - x, py - y) < 9) return i;
    }
  }
  return null;
}

function focusTimeline(index) {
  const enc = state.encounters[index];
  const mid = (enc.a.start + enc.b.end) / 2;
  const span = Math.max(enc.b.end - enc.a.start, 3600000) * 6;
  const width = els.timeline.clientWidth || 800;
  state.tl.scale = width / span;
  state.tl.offset = mid - width / state.tl.scale / 2;
}

function renderTimeline() {
  const canvas = els.timeline;
  const tl = state.tl;
  if (!tl) return;
  const dpr = window.devicePixelRatio || 1;
  const w = canvas.clientWidth;
  const h = canvas.clientHeight;
  if (canvas.width !== w * dpr || canvas.height !== h * dpr) {
    canvas.width = w * dpr;
    canvas.height = h * dpr;
  }
  const ctx = canvas.getContext("2d");
  ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
  ctx.clearRect(0, 0, w, h);

  // lane guides + labels
  ctx.strokeStyle = "#2a333d";
  ctx.lineWidth = 1;
  ctx.fillStyle = "#8b98a5";
  ctx.font = "11px system-ui, sans-serif";
  [0, 1].forEach((lane) => {
    const y = laneY(h, lane);
    ctx.beginPath();
    ctx.moveTo(0, y);
    ctx.lineTo(w, y);
    ctx.stroke();
    ctx.fillText(lane === 0 ? "Person A" : "Person B", 8, y - 8);
  });

  drawTimeAxis(ctx, w, h);

  const visibleSpan = w / tl.scale;
  const detailed = visibleSpan < TIMELINE_DETAIL_SPAN_MS;

  state.encounters.forEach((enc, i) => {
    const active = i === state.activeIndex;
    const colored = detailed || active;
    const color = colored ? state.colors[i] : "#8b98a5";
    const aMid = (enc.a.start + enc.a.end) / 2;
    const bMid = (enc.b.start + enc.b.end) / 2;
    const ax = timeToX(aMid);
    const bx = timeToX(bMid);
    const ay = laneY(h, 0);
    const by = laneY(h, 1);

    if (ax < -20 && bx < -20) return;
    if (ax > w + 20 && bx > w + 20) return;

    // connector between the paired points
    ctx.strokeStyle = color;
    ctx.globalAlpha = active ? 0.9 : 0.35;
    ctx.lineWidth = active ? 2 : 1;
    ctx.beginPath();
    ctx.moveTo(ax, ay);
    ctx.lineTo(bx, by);
    ctx.stroke();
    ctx.globalAlpha = 1;

    // duration bars (start..end) on each lane
    if (detailed) {
      ctx.strokeStyle = color;
      ctx.lineWidth = active ? 5 : 3;
      ctx.beginPath();
      ctx.moveTo(timeToX(enc.a.start), ay);
      ctx.lineTo(timeToX(enc.a.end), ay);
      ctx.moveTo(timeToX(enc.b.start), by);
      ctx.lineTo(timeToX(enc.b.end), by);
      ctx.stroke();
    }

    // dots
    const r = active ? 7 : 5;
    drawDot(ctx, ax, ay, r, color, active);
    drawDot(ctx, bx, by, r, color, active);
  });
}

function drawDot(ctx, x, y, r, color, ring) {
  ctx.beginPath();
  ctx.fillStyle = color;
  ctx.arc(x, y, r, 0, Math.PI * 2);
  ctx.fill();
  if (ring) {
    ctx.strokeStyle = "#fff";
    ctx.lineWidth = 2;
    ctx.stroke();
  }
}

function drawTimeAxis(ctx, w, h) {
  ctx.fillStyle = "#8b98a5";
  ctx.font = "10px system-ui, sans-serif";
  const tl = state.tl;
  const spanMs = w / tl.scale;
  const target = 6; // approx label count
  const niceSteps = [
    3600000, 6 * 3600000, 12 * 3600000, 86400000, 7 * 86400000, 30 * 86400000, 90 * 86400000, 365 * 86400000,
  ];
  let step = niceSteps[niceSteps.length - 1];
  for (const s of niceSteps) {
    if (spanMs / s <= target) {
      step = s;
      break;
    }
  }
  const start = Math.ceil(tl.offset / step) * step;
  for (let t = start; timeToX(t) < w; t += step) {
    const x = timeToX(t);
    ctx.strokeStyle = "#20272f";
    ctx.beginPath();
    ctx.moveTo(x, 0);
    ctx.lineTo(x, h - 14);
    ctx.stroke();
    const label = step >= 86400000 ? new Date(t).toLocaleDateString() : new Date(t).toLocaleString([], { hour: "2-digit", minute: "2-digit", month: "short", day: "numeric" });
    ctx.fillText(label, x + 4, h - 4);
  }
}
