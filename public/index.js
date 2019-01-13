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

const renderLoop = () => {
    frame.tick();

    drawGrid();
    drawCells();

    animationId = requestAnimationFrame(renderLoop);
};

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


// renderLoop()
frame.tick();

drawGrid();
drawCells();
