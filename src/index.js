'use strict';

const canvas = window.document.getElementById("canvas");
canvas.style.height = (window.innerHeight - 200) + "px";
window.document.addEventListener("DOMContentLoaded", function () {
  const request = new XMLHttpRequest();
  request.open("GET", "/grid", true);
  request.onreadystatechange = function () {
    if (request.readyState !== 4 || request.status !== 200) {
      return;
    }
    const grid = JSON.parse(request.responseText);
    displayGrid(grid);
  };
  request.send();
});

function displayGrid(grid) {
  const canvas = window.document.getElementById("canvas");
  const dimensions = {
    width: canvas.offsetWidth,
    height: window.innerHeight - 200,
    offset: 2
  };
  const stage = buildStage(dimensions, canvas.id);
  stage.add(buildGridLayer(dimensions, grid));
  stage.add(buildCellsLayer(dimensions, grid));
}

function buildStage(dimensions, containerId) {
  return new Konva.Stage({
    container: containerId,
    width: dimensions.width,
    height: dimensions.height
  });
}

function buildGridLayer(dimensions, grid) {
  const offset = dimensions.offset;
  const rows = grid.width;
  const columns = grid.height;

  const gridWidth = dimensions.width - 2 * offset;
  const gridHeight = dimensions.height - 2 * offset;
  const columnHeight = gridHeight / columns;
  const rowWidth = gridWidth / rows;

  const gridLayer = new Konva.Layer();
  for (let column = 0; column <= columns; ++column) {
    gridLayer.add(
      new Konva.Line({
        points: [
          offset,
          column * columnHeight + offset,
          gridWidth + offset,
          column * columnHeight + offset
        ],
        stroke: "black",
        strokeWidth: 1
      })
    );
  }
  for (let row = 0; row <= rows; ++row) {
    gridLayer.add(
      new Konva.Line({
        points: [
          row * rowWidth + offset,
          offset,
          row * rowWidth + offset,
          gridHeight + offset
        ],
        stroke: "black",
        strokeWidth: 1
      })
    );
  }
  return gridLayer;
}

function buildCellsLayer(dimensions, grid) {
  const offset = dimensions.offset;
  const rows = grid.width;
  const columns = grid.height;
  const cells = grid.cells;

  const gridWidth = dimensions.width - 2 * offset;
  const gridHeight = dimensions.height - 2 * offset;
  const columnHeight = gridHeight / columns;
  const rowWidth = gridWidth / rows;
  const circleSize = Math.min(rowWidth, columnHeight);

  const cellsLayer = new Konva.Layer();
  for (let column = 0; column < columns; ++column) {
    for (let row = 0; row < rows; ++row) {
      if (cells[column][row]) {
        cellsLayer.add(
          new Konva.Circle({
            x: row * rowWidth + offset + rowWidth / 2,
            y: column * columnHeight + offset + columnHeight / 2,
            radius: circleSize / 2,
            fill: "red",
            stroke: "black",
            strokeWidth: 1
          })
        );
      }
    }
  }
  return cellsLayer;
}
