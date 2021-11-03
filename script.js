let numtiles = 1;
let subgrid_start = { x: 30, y: 30 };
let subgrid_end = { x: 34, y: 34 };
let colors = ["purple", "dodgerblue", "slategray", "crimson", "lightgray", "white"];

let score = 0;

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
let starting_tile = [[0, 1, 2, 3], [1, 2, 3, 0], [2, 3, 0, 1], [3, 0, 1, 2]];
let n = 0;
let num_colors = 4;
for (let i = 0; i < num_colors; i++) {
    for (let j = 0; j < num_colors; j++) {
        for (let k = 0; k < num_colors; k++) {
            for (let l = 0; l < num_colors; l++) {
                let new_tile = [i, j, k, l];
                if (!is_duplicate(starting_tile, new_tile) && !is_duplicate(tiles, new_tile) && !is_dichromatic_bowtie(new_tile)) {
                    tiles[n] = new_tile;
                    n++;
                }
            }
        }
    }
}

let unique_tileset = tiles;

tiles.unshift([4, 4, 4, 4]);

shuffle_tiles();

let max_tiles = tiles.length;

place_tile(32, 32, [0, 1, 2, 3]);

let current_tile = tiles.pop();

let current_hover = null;

window.addEventListener('load', render);

window.addEventListener('wheel', function(event) {
    event.preventDefault();
    if (event.deltaY < 0) {
        // Rotate counter-clockwise
        current_tile = rotate_tile(current_tile, 1);
    } else {
        // Rotate clockwise
        current_tile = rotate_tile(current_tile, 3);
    }
    this.document.getElementById("tile-new").innerHTML = create_svg(current_tile, 1.0);
    if (current_hover != null) {
        current_hover.innerHTML = create_svg(current_tile, 0.4);
    }
});

function render() {
    let divs = "";
    let gtas = "";
    for (i = subgrid_start.y; i <= subgrid_end.y; i++) {
        let gta = '"';
        for (j = subgrid_start.x; j <= subgrid_end.x; j++) {
            if (is_valid_square(j, i)) {
                if (grid[i][j].type == "space") {
                    divs += `<div class="${grid[i][j].type}" id="${grid[i][j].id}" x="${j}" y="${i}">${create_svg([4, 4, 4, 4], 1.0)}</div>`;
                    gta += `${grid[i][j].id} `;
                } else {
                    divs += `<div class="${grid[i][j].type}" id="${grid[i][j].id}" x="${j}" y="${i}">${create_svg(grid[i][j].tile, 1.0)}</div>`;
                    gta += `${grid[i][j].id} `;
                }
            } else {
                divs += `<div class="blank ${grid[i][j].id}">${create_svg([5, 5, 5, 5], 1.0)}</div>`;
                gta += `${grid[i][j].id} `;
            }
        }
        gta += '" ';
        gtas += gta;
    }
    this.document.getElementById("tiles-div").style.gridTemplateAreas = gtas;
    this.document.getElementById("tiles-div").innerHTML = divs;
    this.document.getElementById("tile-new").innerHTML = create_svg(current_tile, 1.0);
    this.document.getElementById("info").innerHTML = create_info_table();

    this.document.getElementById("new-tile").addEventListener("click", function(event) {
        tiles.push(current_tile);
        shuffle_tiles();
        current_tile = tiles.pop();
        render();
    });

    this.document.getElementById("add-tiles").addEventListener("click", function(event) {
        let final_tile = tiles.shift();
        tiles = tiles.concat(unique_tileset);
        tiles.unshift(final_tile);
        shuffle_tiles();
        render();
    });

    if (tiles.length > 0) {
        let spaces = this.document.getElementsByClassName("space");
        for (let i = 0; i < spaces.length; i++) {
            spaces[i].addEventListener('mouseover', function(event) {
                current_hover = this;
                this.innerHTML = create_svg(current_tile, 0.5);
                event.target.style.cursor = "pointer";
            });
            spaces[i].addEventListener('mouseout', function(event) {
                current_hover = null;
                this.innerHTML = create_svg([4, 4, 4, 4], 1.0);
                event.target.style.cursor = "default";
            });
            spaces[i].addEventListener('click', function(event) {
                let tile_x = parseInt(this.attributes.x.value);
                let tile_y = parseInt(this.attributes.y.value);
                if (is_valid_placement(tile_x, tile_y)) {
                    place_tile(tile_x, tile_y, current_tile);
                    score_tile(tile_x, tile_y);
                    recalculate_subgrid(tile_x, tile_y);
                    current_tile = tiles.pop();
                    render();
                } else {
                    alert("Cannot place this tile here!");
                }
            });
        }
        this.document.getElementById("tile-new").removeEventListener('click', rotate_on_click);
        this.document.getElementById("tile-new").addEventListener('click', rotate_on_click);
        this.document.getElementById("tile-new").addEventListener('mouseover', function(event) {
            event.target.style.cursor = "pointer";
        });
        this.document.getElementById("tile-new").addEventListener('mouseout', function(event) {
            event.target.style.cursor = "default";
        });
    }
}

function shuffle_tiles() {
    let final_tile = tiles.shift();
    let counter = tiles.length;
    while (counter > 0) {
        let index = Math.floor(Math.random() * counter);
        counter--;
        let temp = tiles[counter];
        tiles[counter] = tiles[index];
        tiles[index] = temp;
    }
    tiles.unshift(final_tile);
}

function rotate_on_click() {
    console.log(current_tile)
    current_tile = rotate_tile(current_tile, 3);
    this.innerHTML = create_svg(current_tile, 1.0);
    if (current_hover != null) {
        current_hover.innerHTML = create_svg(current_tile, 0.4);
    }
}

function is_duplicate(tile_array, tile) {
    return contains_tile(tile_array, tile) ||
           contains_tile(tile_array, rotate_tile(tile, 1)) ||
           contains_tile(tile_array, rotate_tile(tile, 2)) ||
           contains_tile(tile_array, rotate_tile(tile, 3));
}

function is_dichromatic_bowtie(tile) {
    return tile[0] == tile[2] && tile[1] == tile[3] && tile[0] != tile[1];
}

function contains_tile(tile_array, tile) {
    for (let i = 0; i < tile_array.length; i++) {
        if (tile_array[i][0] == tile[0] &&
            tile_array[i][1] == tile[1] &&
            tile_array[i][2] == tile[2] &&
            tile_array[i][3] == tile[3]) {
                return true;
        }
    }
}

function rotate_tile(tile, n) {
    for (let i = 0; i < n; i++) {
        let temp =  tile[0];
        tile[0] =  tile[1];
        tile[1] =  tile[2];
        tile[2] =  tile[3];
        tile[3] = temp;
    }
    return tile;
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

function score_tile(x, y) {
    let left_color = grid[y][x].tile[0];
    let top_color = grid[y][x].tile[1];
    let right_color = grid[y][x].tile[2];
    let bottom_color = grid[y][x].tile[3];
    let scores = [{
        color: left_color,
        score: score_color(left_color, x-1, y, 2),
    }, {
        color: top_color,
        score: score_color(top_color, x, y-1, 3),
    }, {
        color: right_color,
        score: score_color(right_color, x+1, y, 0),
    }, {
        color: bottom_color,
        score: score_color(bottom_color, x, y+1, 1),
    }];
    let i = 0;
    while (i < scores.length) {
        if (scores[i].color == scores[(i+1)%scores.length].color) {
            if (scores[i].score == null || scores[ (i+1)%scores.length].score == null) {
                scores[i].score = null;
                scores[(i+1)%scores.length].score = null;
                i++
            } else {
                scores[i].score += scores[ (i+1)%scores.length].score;
                scores.splice((i+1)%scores.length, 1);
            }
        } else {
            i++;
        }
    }
    let final_score = 0;
    for (let i = 0; i < scores.length; i++) {
        if (scores[i].score != null) {
            if (scores[i].score < 4) {
                final_score += (scores[i].score + 1);
            } else {
                final_score += (scores[i].score + 1) * 2;
            }
        }
    }
    if (final_score != 0) {
        score += final_score;
    }
}

function score_color(color, x, y, origin) {
    let tile = grid[y][x];
    if (tile.type == "space") {
        return null;
    }
    let tally = 1;
    let next_tiles = [];
    if (tile[(origin+1)%4] == color) {
        next_tiles.push((origin+1)%4);
    }
    if (tile[(origin+3)%4] == color) {
        next_tiles.push((origin+3)%4);
    }
    if ((tile[(origin+1)%4] == color || tile[(origin+3)%4] == color) && tile[(origin+2)%4] == color) {
        next_tiles.push((origin+2)%4);
    }
    for (let i = 0; i < next_tiles.length; i++) {
        if (next_tile[i] == 0) {
            let s = score_color(color, x-1, y, 2);
            if (s == null) {
                return null;
            }
            tally += s;
        } else if (next_tile[i] == 1) {
            let s = score_color(color, x, y-1, 3);
            if (s == null) {
                return null;
            }
            tally += s;
        } else if (next_tile[i] == 2) {
            let s = score_color(color, x+1, y, 0);
            if (s == null) {
                return null;
            }
            tally += s;
        } else {
            let s = score_color(color, x, y+1, 1);
            if (s == null) {
                return null;
            }
            tally += s;
        }
    }
    return tally;
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

function create_svg([left, top, right, bottom], opacity) {
    let svg =
        `<svg viewbox="0 0 500 500" style="pointer-events:none" version="1.1" xmlns="http://www.w3.org/2000/svg">
            <polyline points="0 0 250 250 0 500 0 0" stroke="${colors[left]}" fill="${colors[left]}" fill-opacity="${opacity}" stroke-width="1" stroke-opacity="${opacity/2}"/>
            <polyline points="0 0 500 0 250 250 0 0" stroke="${colors[top]}" fill="${colors[top]}" fill-opacity="${opacity}" stroke-width="1" stroke-opacity="${opacity/2}"/>
            <polyline points="500 0 500 500 250 250 500 0" stroke="${colors[right]}" fill="${colors[right]}" fill-opacity="${opacity}" stroke-width="1" stroke-opacity="${opacity/2}"/>
            <polyline points="0 500 500 500 250 250 0 500" stroke="${colors[bottom]}" fill="${colors[bottom]}" fill-opacity="${opacity}" stroke-width="1" stroke-opacity="${opacity/2}"/>
        </svg>`
    return svg;
}

function create_info_table() {
    let table =
        `<div>
            <table>
                <tr><th>Score:</th><td>${score}</td>
                <tr><th>Tiles Remaining:</th><td>${tiles.length}</td>
            </table>
            <div class="spacer"></div>
            <ul>
                <li>Scroll with your mouse wheel or click the new tile to rotate</li>
                <li>Earn points for completing shapes of a given color</li>
            </ul>
            <div class="spacer"></div>
            <div style="width:100%;text-align:center">
                <button id="new-tile">Draw New Tile</button>
                <button id="add-tiles">Add More Tiles</button>
            </div>
        </div>`;
    return table;
}
