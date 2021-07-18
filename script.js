let numtiles = 1;
let subgrid_start = { x: 31, y: 31 };
let subgrid_end = { x: 33, y: 33 };
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

function render() {
    let divs = "";
    let gtas = "";
    for (i = subgrid_start.y; i <= subgrid_end.y; i++) {
        let gta = '"';
        for (j = subgrid_start.x; j <= subgrid_end.x; j++) {
            if (is_valid_square(i, j)) {
                if (grid[j][i].type == "space") {
                    divs += `<div class="${grid[j][i].type}" id="${grid[j][i].id}" x="${j}" y="${i}">${create_svg([4, 4, 4, 4])}</div>`;
                    gta += `${grid[j][i].id} `;
                } else {
                    divs += `<div class="${grid[j][i].type}" id="${grid[j][i].id}" x="${j}" y="${i}">${create_svg(grid[j][i].tile)}</div>`;
                    gta += `${grid[j][i].id} `;
                }
            } else {
                divs += `<div class="blank ${grid[j][i].id}">${create_svg([5, 5, 5, 5])}</div>`;
                gta += `${grid[j][i].id} `;
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
            console.log(event);
            this.innerHTML = create_svg([4, 4, 4, 4]);
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

function getRandomColor() {
    var letters = '0123456789ABCDEF';
    var color = '#';
    for (var i = 0; i < 6; i++) {
        color += letters[Math.floor(Math.random() * 16)];
    }
    return color;
}
