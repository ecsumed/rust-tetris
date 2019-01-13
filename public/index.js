import * as wasm from "tetris";
import { Canvas } from "tetris";
import { memory } from "tetris/tetris_bg";

const GRID_COLOR = "#CCCCCC";
const EMPTY_COLOR = "#FFFFFF";
const BLOCK_COLOR = "#5CB3FF";
const CELL_SIZE = 25; // px
const WIDTH = '10'
const HEIGHT = '21'

const canvas = document.getElementById("tetris-canvas");
canvas.height = (CELL_SIZE + 1) * HEIGHT + 1;
canvas.width = (CELL_SIZE + 1) * WIDTH + 1; 

const ctx = canvas.getContext('2d');

const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // Vertical lines.
  for (let i = 0; i <= WIDTH; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * HEIGHT + 1);
  }

  // Horizontal lines.
  for (let j = 0; j <= HEIGHT; j++) {
    ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * WIDTH + 1, j * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();
};

const drawCells = () => {
  const cellsPtr = frame.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, WIDTH * HEIGHT);

  ctx.beginPath();

  for (let row = 0; row < HEIGHT; row++) {
    for (let col = 0; col < WIDTH; col++) {
      const idx = getIndex(row, col);

      ctx.fillStyle = cells[idx] === 0
        ? EMPTY_COLOR
        : BLOCK_COLOR;

      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }

  ctx.stroke();
};

const getIndex = (row, column) => {
    return row * WIDTH + column;
};

const frame = Canvas.new(WIDTH, HEIGHT);

document.addEventListener('keydown', function(event) {
    switch (event.key) {
        // Left pressed
        case "ArrowLeft":
            frame.piece_left();

            drawGrid();
            drawCells();
            break;

        // Right pressed
        case "ArrowRight":
            frame.piece_right();

            drawGrid();
            drawCells();
            break;

        // Up pressed
        case "ArrowUp":
            frame.piece_rotate_clockwise();

            drawGrid();
            drawCells();
            break;

        // Down pressed
        case "ArrowDown":
            frame.tick();

            drawGrid();
            drawCells();
            break;
    }
});

function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

const fps = new class {
  constructor() {
    this.fps = document.getElementById("fps");
    this.frames = [];
    this.lastFrameTimeStamp = performance.now();
  }

  render() {
    // Convert the delta time since the last frame render into a measure
    // of frames per second.
    const now = performance.now();
    const delta = now - this.lastFrameTimeStamp;
    this.lastFrameTimeStamp = now;
    const fps = 1 / delta * 1000;

    // Save only the latest 100 timings.
    this.frames.push(fps);
    if (this.frames.length > 100) {
      this.frames.shift();
    }

    // Find the max, min, and mean of our 100 latest timings.
    let min = Infinity;
    let max = -Infinity;
    let sum = 0;
    for (let i = 0; i < this.frames.length; i++) {
      sum += this.frames[i];
      min = Math.min(this.frames[i], min);
      max = Math.max(this.frames[i], max);
    }
    let mean = sum / this.frames.length;

    // Render the statistics.
    this.fps.textContent = `
Frames per Second:
         latest = ${Math.round(fps)}
avg of last 100 = ${Math.round(mean)}
min of last 100 = ${Math.round(min)}
max of last 100 = ${Math.round(max)}
`.trim();
  }
};

const renderLoop = () => {
	fps.render();
    frame.tick();

    drawGrid();
    drawCells();

    animationId = requestAnimationFrame(renderLoop);
};

fps.render();
frame.tick();

drawGrid();
drawCells();
