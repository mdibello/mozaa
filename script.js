let numtiles = 1;
let subgrid_start = { x: 30, y: 30 };
let subgrid_end = { x: 34, y: 34 };
let colors = ["purple", "dodgerblue", "slategray", "crimson", "lightgray", "white"];

let grid = [];
for (let y = 0; y < 65; y++) {
    let row = [];
    for (let x = 0; x < 65; x++) {
        row[x] = {
            type: "space",
            id: y * 64 + x,
            tile: [-1, -1, -1, -1],
        }
    }
    grid[y] = row;
}

let tiles = [];
let n = 0;
for (let i = 0; i < 4; i++) {
    for (let j = 0; j < 4; j++) {
        for (let k = 0; k < 4; k++) {
            for (let l = 0; l < 4; l++) {
                let new_tile = [i, j, k, l];
                if (new_tile != [0, 1, 2, 3]) {
                    tiles[n] = new_tile;
                    n++;
                }
            }
        }
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

place_tile(32, 32, [0, 1, 2, 3]);

let current_tile = tiles.pop();

window.addEventListener('load', render);

window.addEventListener('wheel', function(event) {
    event.preventDefault();
    if (event.deltaY < 0) {
        // Rotate counter-clockwise
        let temp = current_tile[0];
        current_tile[0] = current_tile[1];
        current_tile[1] = current_tile[2];
        current_tile[2] = current_tile[3];
        current_tile[3] = temp;
    } else {
        // Rotate clockwise
        let temp = current_tile[3];
        current_tile[3] = current_tile[2];
        current_tile[2] = current_tile[1];
        current_tile[1] = current_tile[0];
        current_tile[0] = temp;
    }
    this.document.getElementById("tile-new").innerHTML = create_svg(current_tile);
});

function render() {
    let divs = "";
    let gtas = "";
    for (i = subgrid_start.y; i <= subgrid_end.y; i++) {
        let gta = '"';
        for (j = subgrid_start.x; j <= subgrid_end.x; j++) {
            if (is_valid_square(j, i)) {
                if (grid[i][j].type == "space") {
                    divs += `<div class="${grid[i][j].type}" id="${grid[i][j].id}" x="${j}" y="${i}">${create_svg([4, 4, 4, 4])}</div>`;
                    gta += `${grid[i][j].id} `;
                } else {
                    divs += `<div class="${grid[i][j].type}" id="${grid[i][j].id}" x="${j}" y="${i}">${create_svg(grid[i][j].tile)}</div>`;
                    gta += `${grid[i][j].id} `;
                }
            } else {
                divs += `<div class="blank ${grid[i][j].id}">${create_svg([5, 5, 5, 5])}</div>`;
                gta += `${grid[i][j].id} `;
            }
        }
        gta += '" ';
        gtas += gta;
    }
    this.document.getElementById("tiles-div").style.gridTemplateAreas = gtas;
    this.document.getElementById("tiles-div").innerHTML = divs;
    this.document.getElementById("tile-new").innerHTML = create_svg(current_tile);

    let spaces = this.document.getElementsByClassName("space");
    for (let i = 0; i < spaces.length; i++) {
        spaces[i].addEventListener('mouseover', function(event) {
            // this.innerHTML = create_svg(current_tile);
            event.target.style.cursor = "pointer";
        });
        spaces[i].addEventListener('mouseout', function(event) {
            // this.innerHTML = create_svg([4, 4, 4, 4]);
            event.target.style.cursor = "default";
        });
        spaces[i].addEventListener('click', function(event) {
            let tile_x = parseInt(this.attributes.x.value);
            let tile_y = parseInt(this.attributes.y.value);
            if (is_valid_placement(tile_x, tile_y)) {
                place_tile(tile_x, tile_y, current_tile);
                recalculate_subgrid(tile_x, tile_y);
                current_tile = tiles.pop();
                render();
            } else {
                alert("Cannot place this tile here!");
            }
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

function is_valid_placement(x, y) {
    if (grid[y-1][x].type == "tile" && grid[y-1][x].tile[3] != current_tile[1]) {
        return false;
    }
    if (grid[y+1][x].type == "tile" && grid[y+1][x].tile[1] != current_tile[3]) {
        return false;
    }
    if (grid[y][x-1].type == "tile" && grid[y][x-1].tile[2] != current_tile[0]) {
        return false;
    }
    if (grid[y][x+1].type == "tile" && grid[y][x+1].tile[0] != current_tile[2]) {
        return false;
    }
    return true;
}

function place_tile(x, y, tile) {
    grid[y][x].type = "tile";
    grid[y][x].tile = tile;
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

function create_svg([left, top, right, bottom]) {
    let svg =
    `<svg viewbox="0 0 500 500" version="1.1" xmlns="http://www.w3.org/2000/svg">
        <polyline points="0 0 250 250 0 500 0 0" stroke="${colors[left]}" fill="${colors[left]}" stroke-width="1"/>
        <polyline points="0 0 500 0 250 250 0 0" stroke="${colors[top]}" fill="${colors[top]}" stroke-width="1"/>
        <polyline points="500 0 500 500 250 250 500 0" stroke="${colors[right]}" fill="${colors[right]}" stroke-width="1"/>
        <polyline points="0 500 500 500 250 250 0 500" stroke="${colors[bottom]}" fill="${colors[bottom]}" stroke-width="1"/>
    </svg>`
    return svg;
}
