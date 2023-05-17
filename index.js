import { Universe } from "./pkg/etka_life";
import { memory } from "./pkg/etka_life_bg.wasm";

const CELL_SIZE = 5;
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const DEAD = 0;
const ALIVE = 1;

//TODO initUniverse

const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

let animationId = null;

const renderLoop = () => {
  universe.tick();

  drawCells();
  drawGrid();

  animationId = requestAnimationFrame(renderLoop);
};

const drawGrid = () => {
  //TODO drawGridBody
};

const drawCells = () => {
  //TODO drawCellsBody
};

const drawCellsOfType = (cells, condition, style) => {
  //TODO drawCellsOfTypeBody
};

const getIndex = (row, column) => {
  return row * width + column;
};

const playPauseButton = document.getElementById("play-pause");

const isPaused = () => {
  return animationId === null;
};

const play = () => {
  playPauseButton.textContent = "⏸";
  renderLoop();
};

const pause = () => {
  playPauseButton.textContent = "▶";
  cancelAnimationFrame(animationId);
  animationId = null;
};

playPauseButton.addEventListener("click", event => {
  if (isPaused()) {
    play();
  } else {
    pause();
  }
});

play();
