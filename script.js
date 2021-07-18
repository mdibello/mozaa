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
grid[32][32].type = "tile";
grid[32][32].tile = start_tile.id;
grid[32][32].color = start_tile.color;

let current_tile = tiles.pop();

window.addEventListener('load', function(event) {
    let divs = "";
    let gtas = "";
    for (i = subgrid_start.y; i <= subgrid_end.y; i++) {
        let gta = '"';
        for (j = subgrid_start.x; j <= subgrid_end.x; j++) {
            divs += `<div class="${grid[j][i].type} tile-${grid[j][i].id}" style="background-color:${grid[j][i].color}"></div>`;
            gta += `tile-${grid[j][i].id} `;
        }
        gta += '" ';
        gtas += gta;
    }
    this.document.getElementById("tiles-div").style.gridTemplateAreas = gtas;
    this.document.getElementById("tiles-div").innerHTML = divs;
    this.document.getElementById("tile-new").style.backgroundColor = current_tile.color;
});

function getRandomColor() {
    var letters = '0123456789ABCDEF';
    var color = '#';
    for (var i = 0; i < 6; i++) {
        color += letters[Math.floor(Math.random() * 16)];
    }
    return color;
}
