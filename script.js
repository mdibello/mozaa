let numtiles = 1;
let subgrid_start = { x: 31, y: 31 };
let subgrid_end = { x: 33, y: 33 };

let grid = [];
for (let y = 0; y < 65; y++) {
    let row = [];
    for (let x = 0; x < 65; x++) {
        row[x] = {
            type: "space",
            id: y * 64 + x,
            tile: -1,
            color: "gray",
        }
    }
    grid[y] = row;
}

let tiles = [];
for (let i = 0; i < 64; i++) {
    tiles[i] = {
        color: getRandomColor(),
        id: i,
    }
}

let counter = tiles.length;
while (counter > 0) {
    let index = Math.floor(Math.random() * counter);
    counter--;
    let temp = tiles[counter];
    tiles[counter] = tiles[index];
    tiles[index] = temp;
}

let start_tile = tiles.pop();
place_tile(32, 32, start_tile);

let current_tile = tiles.pop();

window.addEventListener('load', render);

function render() {
    let divs = "";
    let gtas = "";
    for (i = subgrid_start.y; i <= subgrid_end.y; i++) {
        let gta = '"';
        for (j = subgrid_start.x; j <= subgrid_end.x; j++) {
            if (is_valid_square(i, j)) {
                divs += `<div class="${grid[j][i].type}" id="${grid[j][i].id}" x="${j}" y="${i}" style="background-color:${grid[j][i].color}"></div>`;
                gta += `${grid[j][i].id} `;
            } else {
                divs += `<div class="blank ${grid[j][i].id}" style="background-color:white"></div>`;
                gta += `${grid[j][i].id} `;
            }
        }
        gta += '" ';
        gtas += gta;
    }
    this.document.getElementById("tiles-div").style.gridTemplateAreas = gtas;
    this.document.getElementById("tiles-div").innerHTML = divs;
    this.document.getElementById("tile-new").style.backgroundColor = current_tile.color;

    let spaces = this.document.getElementsByClassName("space");
    for (let i = 0; i < spaces.length; i++) {
        spaces[i].addEventListener('mouseover', function(event) {
            event.target.style.backgroundColor = "lightgray";
            event.target.style.cursor = "pointer";
        });
        spaces[i].addEventListener('mouseout', function(event) {
            event.target.style.backgroundColor = "gray";
            event.target.style.cursor = "default";
        });
        spaces[i].addEventListener('click', function(event) {
            let tile_x = parseInt(this.attributes.x.value);
            let tile_y = parseInt(this.attributes.y.value);
            place_tile(tile_y, tile_x, current_tile);
            recalculate_subgrid(tile_x, tile_y);
            current_tile = tiles.pop();
            render();
        });
    }
}

function is_valid_square(x, y) {
    return grid[y-1][x].type == "tile" ||
           grid[y+1][x].type == "tile" ||
           grid[y][x-1].type == "tile" ||
           grid[y][x+1].type == "tile" ||
           grid[y][x].type == "tile";
}

function place_tile(x, y, tile) {
    grid[y][x].type = "tile";
    grid[y][x].tile = tile.id;
    grid[y][x].color = tile.color;
}

function recalculate_subgrid(x, y) {
    if (x-1 < subgrid_start.x || y-1 < subgrid_start.y) {
        subgrid_start.x -= 1;
        subgrid_start.y -= 1;
    }
    if (x+1 > subgrid_end.x || y+1 > subgrid_end.y) {
        subgrid_end.x += 1;
        subgrid_end.y += 1;
    }
}

function getRandomColor() {
    var letters = '0123456789ABCDEF';
    var color = '#';
    for (var i = 0; i < 6; i++) {
        color += letters[Math.floor(Math.random() * 16)];
    }
    return color;
}
