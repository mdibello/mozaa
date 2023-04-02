import {
    initialize,
    is_valid_placement,
    get_tile,
    place_tile,
    rotate_tile,
    should_be_displayed,
    contains_tile,
    get_current_tile,
    draw_new_tile,
    recalculate_subgrid,
    get_subgrid,
    tiles_remaining,
    coordinate,
    select_new_tile,
    add_more_tiles,
    total_score,
    score,
    default as init
} from './pkg/mozaa.js';

async function initialize_game() {
    await init('./pkg/mozaa_bg.wasm');
    initialize();
    current_tile = get_current_tile();
    render();
}

window.addEventListener('load', initialize_game);

let colors = ["purple", "dodgerblue", "slategray", "crimson", "lightgray", "white"];

let current_tile = null;
let current_hover = null;
let final_tile = false;

window.addEventListener('wheel', function(event) {
    if (current_tile != null) {
        event.preventDefault();
        if (event.deltaY < 0) {
            // Rotate counter-clockwise
            current_tile = rotate_tile(current_tile, 1);
        } else {
            // Rotate clockwise
            current_tile = rotate_tile(current_tile, 3);
        }
        document.getElementById("tile-new").innerHTML = create_svg(tile_to_colors(current_tile), 1.0);
        if (current_hover != null) {
            current_hover.innerHTML = create_svg(tile_to_colors(current_tile), 0.4);
        }
    }
});

function render() {
    let divs = "";
    let gtas = "";
    let subgrid = get_subgrid();
    for (var i = subgrid.start.y; i <= subgrid.end.y; i++) {
        let gta = '"';
        for (var j = subgrid.start.x; j <= subgrid.end.x; j++) {
            if (should_be_displayed(coordinate(j, i))) {
                if (!contains_tile(coordinate(j, i))) {
                    divs += `<div class="space" id="${generate_id(j, i)}" x="${j}" y="${i}">${create_svg([4, 4, 4, 4], 1.0)}</div>`;
                    gta += `${generate_id(j, i)} `;
                } else {
                    let tile_colors = tile_to_colors(get_tile(coordinate(j, i)));
                    divs += `<div class="tile" id="${generate_id(j, i)}" x="${j}" y="${i}">${create_svg(tile_colors, 1.0)}</div>`;
                    gta += `${generate_id(j, i)} `;
                }
            } else {
                divs += `<div class="blank ${generate_id(j, i)}">${create_svg([5, 5, 5, 5], 1.0)}</div>`;
                gta += `${generate_id(j, i)} `;
            }
        }
        gta += '" ';
        gtas += gta;
    }
    document.getElementById("tiles-div").style.gridTemplateAreas = gtas;
    document.getElementById("tiles-div").innerHTML = divs;
    document.getElementById("info").innerHTML = create_info_table();

    if (!final_tile) {
        document.getElementById("tile-new").innerHTML = create_svg(tile_to_colors(current_tile), 1.0);
    } else {
        document.getElementById("tile-new").innerHTML = create_svg([4, 4, 4, 4], 1.0);
    }

    document.getElementById("new-tile").addEventListener("click", function(_) {
        select_new_tile();
        current_tile = get_current_tile();
        render();
    });

    document.getElementById("add-tiles").addEventListener("click", function(_) {
        final_tile = false;
        add_more_tiles();
        current_tile = get_current_tile();
        render();
    });

    if (!final_tile) {
        let spaces = document.getElementsByClassName("space");
        for (let i = 0; i < spaces.length; i++) {
            spaces[i].addEventListener('mouseover', function(event) {
                current_hover = this;
                this.innerHTML = create_svg(tile_to_colors(current_tile), 0.5);
                event.target.style.cursor = "pointer";
            });
            spaces[i].addEventListener('mouseout', function(event) {
                current_hover = null;
                this.innerHTML = create_svg([4, 4, 4, 4], 1.0);
                event.target.style.cursor = "default";
            });
            spaces[i].addEventListener('click', function(_) {
                let tile_x = parseInt(this.attributes.x.value);
                let tile_y = parseInt(this.attributes.y.value);
                if (is_valid_placement(current_tile, coordinate(tile_x, tile_y))) {
                    place_tile(coordinate(tile_x, tile_y), current_tile);
                    recalculate_subgrid(coordinate(tile_x, tile_y));
                    if (tiles_remaining() > 0) {
                        draw_new_tile();
                        current_tile = get_current_tile();
                    } else {
                        final_tile = true;
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

function rotate_on_click() {
    current_tile = rotate_tile(current_tile, 3);
    this.innerHTML = create_svg(tile_to_colors(current_tile), 1.0);
    if (current_hover != null) {
        current_hover.innerHTML = create_svg(tile_to_colors(current_tile), 0.4);
    }
}

function create_svg([left, top, right, bottom], opacity) {
    let svg =
        `<svg viewbox="0 0 500 500" style="pointer-events:none" version="1.1" xmlns="http://www.w3.org/2000/svg">
            <polyline points="0 0 250 250 0 500 0 0" stroke="${colors[left]}" fill="${colors[left]}" fill-opacity="${opacity}" stroke-width="1" stroke-opacity="${opacity/2}"/>
            <polyline points="0 0 500 0 250 250 0 0" stroke="${colors[top]}" fill="${colors[top]}" fill-opacity="${opacity}" stroke-width="1" stroke-opacity="${opacity/2}"/>
            <polyline points="500 0 500 500 250 250 500 0" stroke="${colors[right]}" fill="${colors[right]}" fill-opacity="${opacity}" stroke-width="1" stroke-opacity="${opacity/2}"/>
            <polyline points="0 500 500 500 250 250 0 500" stroke="${colors[bottom]}" fill="${colors[bottom]}" fill-opacity="${opacity}" stroke-width="1" stroke-opacity="${opacity/2}"/>
        </svg>`;
    return svg;
}

function create_info_table() {
    let table =
        `<div>
            <table>
                <tr><th>Score:</th><td>${total_score()}</td>
                <tr><th>Tiles Remaining:</th><td>${tiles_remaining()}</td>
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

function generate_id(x, y) {
    return (y * 64) + x;
}

function tile_to_colors(tile) {
    return [tile.left, tile.top, tile.right, tile.bottom];
}
