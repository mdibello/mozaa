import { default as init } from './pkg/mozaa.js'

let colors = ["purple", "dodgerblue", "slategray", "crimson", "lightgray", "white"];

let shapes = {};
let score = 0;

place_tile(32, 32, [0, 1, 2, 3]);
shapes[gen_key(32, 32, 0)] = gen_edges(1, [grid[32][32].id]);
shapes[gen_key(32, 32, 1)] = gen_edges(1, [grid[32][32].id]);
shapes[gen_key(32, 32, 2)] = gen_edges(1, [grid[32][32].id]);
shapes[gen_key(32, 32, 3)] = gen_edges(1, [grid[32][32].id]);

let current_tile = get_current_tile();
let current_hover = null;

window.addEventListener('load', initialize);

window.addEventListener('wheel', function(event) {
    event.preventDefault();
    if (event.deltaY < 0) {
        // Rotate counter-clockwise
        current_tile = rotate_tile(current_tile, 1);
    } else {
        // Rotate clockwise
        current_tile = rotate_tile(current_tile, 3);
    }
    document.getElementById("tile-new").innerHTML = create_svg(current_tile, 1.0);
    if (current_hover != null) {
        current_hover.innerHTML = create_svg(current_tile, 0.4);
    }
});

async function initialize() {
    await init('./pkg/mozaa_bg.wasm');
    render();
}

function render() {
    let divs = "";
    let gtas = "";
    let subgrid = get_subgrid();
    for (var i = subgrid_start.y; i <= subgrid_end.y; i++) {
        let gta = '"';
        for (var j = subgrid_start.x; j <= subgrid_end.x; j++) {
            if (should_be_displayed(coordinate(j, i))) {
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
    document.getElementById("tiles-div").style.gridTemplateAreas = gtas;
    document.getElementById("tiles-div").innerHTML = divs;
    document.getElementById("tile-new").innerHTML = create_svg(current_tile, 1.0);
    document.getElementById("info").innerHTML = create_info_table();

    document.getElementById("new-tile").addEventListener("click", function(event) {
        select_new_tile();
        current_tile = get_current_tile();
        render();
    });

    document.getElementById("add-tiles").addEventListener("click", function(event) {
        add_more_tiles();
        render();
    });

    if (tiles.length > 0) {
        let spaces = document.getElementsByClassName("space");
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
                if (is_valid_placement(coordinate(tile_x, tile_y))) {
                    place_tile(coordinate(tile_x, tile_y), current_tile);
                    score_tile(tile_x, tile_y);
                    recalculate_subgrid(tile_x, tile_y);
                    if (tiles_remaining() > 0) {
                        current_tile = draw_new_tile();
                    } else {
                        current_tile = null;
                    }
                    render();
                } else {
                    alert("Cannot place this tile here!");
                }
            });
        }
        document.getElementById("tile-new").removeEventListener('click', rotate_on_click);
        document.getElementById("tile-new").addEventListener('click', rotate_on_click);
        document.getElementById("tile-new").addEventListener('mouseover', function(event) {
            event.target.style.cursor = "pointer";
        });
        document.getElementById("tile-new").addEventListener('mouseout', function(event) {
            event.target.style.cursor = "default";
        });
    }
}

function score_tile(x, y) {

    console.log('x: ' + x);
    console.log('y: ' + y);

    let tile = grid[y][x].tile;
    let local_shapes = [];

    if (tile[0] == tile[1] && tile[0] == tile[2] && tile[0] == tile[3]) {
        local_shapes = [[0, 1, 2, 3]];
    } else if (tile[0] == tile[1] && tile[0] == tile[2]) {
        local_shapes = [[0, 1, 2], [3]];
    } else if (tile[0] == tile[1] && tile[0] == tile[3]) {
        local_shapes = [[0, 1, 3], [2]];
    } else if (tile[0] == tile[2] && tile[0] == tile[3]) {
        local_shapes = [[0, 2, 3], [1]];
    } else if (tile[0] == tile[1]) {
        if (tile[2] == tile[3]) {
            local_shapes = [[0, 1], [2, 3]];
        } else {
            local_shapes = [[0, 1], [2], [3]];
        }
    } else if (tile[0] == tile[3]) {
        if (tile[1] == tile[2]) {
            local_shapes = [[0, 3], [1, 2]];
        } else {
            local_shapes = [[0, 3], [1], [2]];
        }
    } else if (tile[1] == tile[2] && tile[1] == tile[3]) {
        local_shapes = [[0], [1, 2, 3]];
    } else if (tile[1] == tile[2]) {
        local_shapes = [[0], [1, 2], [3]];
    } else if (tile[2] == tile[3]) {
        local_shapes = [[0], [2, 3], [1]];
    } else {
        local_shapes = [[0], [1], [2], [3]];
    }

    var local_shape_index_to_shape_key = [];
    for (var i = 0; i < local_shapes.length; i++) {
        local_shape_index_to_shape_key.push([]);
    }

    local_shape_index_to_shape_key = check_edge(x-1, y, 0, local_shapes, local_shape_index_to_shape_key);
    local_shape_index_to_shape_key = check_edge(x, y-1, 1, local_shapes, local_shape_index_to_shape_key);
    local_shape_index_to_shape_key = check_edge(x+1, y, 2, local_shapes, local_shape_index_to_shape_key);
    local_shape_index_to_shape_key = check_edge(x, y+1, 3, local_shapes, local_shape_index_to_shape_key);

    // Merge shapes
    local_shape_index_to_shape_key.forEach(function(keys, index, array) {
        if (keys.length > 1) {
            let unified_shape = shapes[keys[0]];
            for (var i = 1; i < keys.length; i++) {
                unified_shape.open_edges += shapes[keys[i]].open_edges;
                unified_shape.open_edges -= 2;
                shapes[keys[i]].tiles.forEach(function(id, index, array) {
                    if (!unified_shape.tiles.includes(id)) {
                        unified_shape.tiles.push(id);
                    }
                });
            }
            keys.forEach(function(key, index, array) {
                shapes[key] = unified_shape;
            });
            score_shape(unified_shape);
        } else if (keys.length == 1) {
            score_shape(shapes[keys[0]]);
        }
    });
}

function check_edge(x, y, edge, local_shapes, local_shape_index_to_shape_key) {
    console.log('x: ' + x);
    console.log('y: ' + y);
    console.log('e: ' + edge);
    console.log('o: ' + (edge+2)%4);
    if (grid[y][x].type == "tile") {
        let key = gen_key(x, y, (edge+2)%4);
        let idx = grid[y][x].id;
        if (!shapes[key].tiles.includes(idx)) {
            local_shapes.forEach(function(value, index, array) {
                if (value.includes(edge)) {
                    shapes[key].open_edges += value.length;
                    local_shape_index_to_shape_key[index].push(key);
                }
            });
            shapes[key].open_edges -= 2;
            shapes[key].tiles.push(idx);
        } else {
            shapes[key].open_edges -= 1;
            score_shape(shapes[key]);
        }
    } else {
        shapes[gen_key(x, y, (edge+2)%4)] = gen_edges(edge, [grid[y][x].id]);
    }
    return local_shape_index_to_shape_key;
}

function gen_key(x, y, side) {
    return {'index':grid[y][x].id, 'side':side};
}

function gen_edges(edges, tiles) {
    return {'open_edges':edges, 'tiles':tiles};
}

function score_shape(shape) {
    console.log(shape);
    if (shape.open_edges == 0) {
        if (shape.tiles.length > 3) {
            score += (shape.tiles.length * 2);
        } else {
            score += shape.tiles.length;
        }
    }
}

function rotate_on_click() {
    console.log(current_tile)
    current_tile = rotate_tile(current_tile, 3);
    this.innerHTML = create_svg(current_tile, 1.0);
    if (current_hover != null) {
        current_hover.innerHTML = create_svg(current_tile, 0.4);
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
