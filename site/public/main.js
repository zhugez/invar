const scenarios = {
  match: {
    heading: 'Behavior matches.',
    description: 'All configured examples match the accepted baseline.',
    output: 'alice\nbob\ncarol',
    channel: 'stdout unchanged',
    exit: '0',
  },
  change: {
    heading: 'Behavior changed.',
    description: 'The names still match. Their order doesn’t. Review the stdout difference.',
    output: 'carol\nbob\nalice',
    channel: 'stdout changed · first byte differs',
    exit: '1',
  },
};

for (const button of document.querySelectorAll('[data-scenario]')) {
  button.addEventListener('click', () => {
    const selected = button.dataset.scenario;
    const scenario = scenarios[selected];
    document.querySelector('.terminal').classList.toggle('changed', selected === 'change');
    for (const option of document.querySelectorAll('[data-scenario]')) {
      const active = option === button;
      option.classList.toggle('active', active);
      option.setAttribute('aria-pressed', String(active));
    }
    const heading = document.getElementById('result-heading');
    heading.replaceChildren();
    const dot = document.createElement('span');
    dot.className = 'status-dot';
    heading.append(dot, document.createTextNode(scenario.heading));
    document.getElementById('result-description').textContent = scenario.description;
    document.getElementById('current-output').textContent = scenario.output;
    document.getElementById('channel-status').textContent = scenario.channel;
    document.getElementById('exit-code').textContent = scenario.exit;
  });
}

const copyButton = document.getElementById('copy-command');
copyButton.addEventListener('click', async () => {
  const feedback = document.getElementById('copy-feedback');
  try {
    await navigator.clipboard.writeText(document.getElementById('install-command').textContent);
    feedback.textContent = 'Install command copied.';
    copyButton.querySelector('.copy-label').textContent = 'Copied';
  } catch {
    feedback.textContent = 'Could not copy. Select the command above and copy it manually.';
  }
});

const canvas = document.getElementById('spine');
const context = canvas.getContext('2d');
if (context) {
  const reducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)');
  const particles = [];
  let width = 0;
  let height = 0;
  let frame = 0;
  let visible = true;
  let tick = 0;
  let seed = 37;
  const random = () => {
    seed = (seed * 16807) % 2147483647;
    return (seed - 1) / 2147483646;
  };
  const grid = 26;
  const occupied = (x, y) => {
    const top = y >= 2 && y <= 6 && x >= 5 && x <= 21;
    const stem = y >= 6 && y <= 19 && x >= 11 && x <= 15;
    const bottom = y >= 19 && y <= 23 && x >= 3 && x <= 19;
    return top || stem || bottom;
  };
  for (let i = 0; i < 1700; i++) {
    const x = random() * grid;
    const y = random() * grid;
    if (occupied(x, y)) {
      particles.push({ x, y, z: random() * 2 - 1, size: .35 + random() * .9, phase: random() * Math.PI * 2, speed: .2 + random() * .7 });
    }
  }
  const project = (x, y, z = 0) => {
    const size = Math.min(width * .77, height * .81);
    return { x: width * .52 + ((x - 13) + z * 1.8) / 26 * size, y: height * .46 + ((y - 13) + z * -1.2) / 26 * size };
  };
  const outline = [[5,2],[21,2],[21,6],[15,6],[15,19],[19,19],[19,23],[3,23],[3,19],[11,19],[11,6],[5,6]];
  function trace(depth) {
    context.beginPath();
    outline.forEach(([x, y], index) => {
      const p = project(x, y, depth);
      if (index === 0) context.moveTo(p.x, p.y);
      else context.lineTo(p.x, p.y);
    });
    context.closePath();
    context.stroke();
  }
  function draw() {
    frame = 0;
    context.clearRect(0, 0, width, height);
    const time = reducedMotion.matches ? 0 : tick / 60;
    context.strokeStyle = 'rgba(105,137,255,.14)';
    context.lineWidth = .6;
    for (let row = 0; row <= 26; row += 2) {
      const a = project(0, row);
      const b = project(26, row);
      context.beginPath(); context.moveTo(a.x, a.y); context.lineTo(b.x, b.y); context.stroke();
    }
    for (let column = 0; column <= 26; column += 2) {
      const a = project(column, 0);
      const b = project(column, 26);
      context.beginPath(); context.moveTo(a.x, a.y); context.lineTo(b.x, b.y); context.stroke();
    }
    context.strokeStyle = 'rgba(128,156,255,.46)';
    context.lineWidth = .8;
    trace(-1);
    context.strokeStyle = 'rgba(168,188,255,.75)';
    trace(1);
    for (const [x, y] of outline) {
      const a = project(x, y, -1);
      const b = project(x, y, 1);
      context.strokeStyle = 'rgba(138,166,255,.42)';
      context.beginPath(); context.moveTo(a.x, a.y); context.lineTo(b.x, b.y); context.stroke();
      context.fillStyle = '#c8d5ff';
      context.fillRect(b.x - 1.1, b.y - 1.1, 2.2, 2.2);
    }
    for (const particle of particles) {
      const inStem = particle.y > 6 && particle.y < 19;
      const drift = inStem ? 0 : Math.sin(time * particle.speed + particle.phase) * .42;
      const p = project(particle.x + drift, particle.y, particle.z);
      const alpha = .25 + (Math.sin(time * .8 + particle.phase) + 1) * .28;
      context.fillStyle = `rgba(${inStem ? '198,215,255' : '118,151,255'},${alpha})`;
      context.fillRect(p.x, p.y, particle.size, particle.size);
    }
    const top = project(13, -1);
    const bottom = project(13, 26);
    context.setLineDash([2, 5]);
    context.strokeStyle = 'rgba(171,188,224,.33)';
    context.beginPath(); context.moveTo(top.x, top.y); context.lineTo(bottom.x, bottom.y); context.stroke();
    context.setLineDash([]);
    if (!reducedMotion.matches && visible && !document.hidden) {
      tick++;
      frame = requestAnimationFrame(draw);
    }
  }
  function restart() {
    cancelAnimationFrame(frame);
    draw();
  }
  new ResizeObserver(() => {
    const bounds = canvas.getBoundingClientRect();
    width = bounds.width;
    height = bounds.height;
    const scale = Math.min(window.devicePixelRatio || 1, 2);
    canvas.width = Math.round(width * scale);
    canvas.height = Math.round(height * scale);
    context.setTransform(scale, 0, 0, scale, 0, 0);
    restart();
  }).observe(canvas);
  new IntersectionObserver(([entry]) => {
    visible = entry.isIntersecting;
    if (visible) restart();
    else cancelAnimationFrame(frame);
  }).observe(canvas);
  document.addEventListener('visibilitychange', restart);
  reducedMotion.addEventListener('change', restart);
}
