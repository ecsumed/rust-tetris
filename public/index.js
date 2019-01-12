import * as wasm from "tetris";
import { Canvas } from "tetris";
import { memory } from "tetris/tetris_bg";

const GRID_COLOR = "#CCCCCC";
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

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);

      ctx.fillStyle = cells[idx] === 0
        ? DEAD_COLOR
        : ALIVE_COLOR;

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

drawGrid();
    
const frame = Canvas.new(WIDTH, HEIGHT);

frame.tick();

drawCells();
