# Keyboard plate prototyping
The idea behind this project is to automate keyboard plate prototyping for i.e. 3d printing

## How to run
### Through CLI
`cargo run --bin kpt_cli {keyboard}`

i.e `cargo run --bin kpt_cli generic60`

You will get then the following options

1. Export Dxf
2. Export Ply
3. Exit


### Through UI
`cargo run --bin kpt_ui {keyboard}`

i.e `cargo run --bin kpt_ui generic60`

D: exports plate to dxf

P: exports plate in ply (default thickness 30mm)


## How to make your own layout

Layouts are in json format 

basic structure is as follow

```json
{
    "layout": [
        // rows
        [
            //keys
             {
                "size": float //size in units
                "char": string //optional for UI module
                "k_type": bit // 0: hidden (spacer) 1: visible (key)
            },
        ],
        [
        ],
        [
        ],
        [
        ]
    ],
    "options": {
        "plate_height": f32 //how thick the plate needs to be, defaults to 20mm
        "screw_holes": bool // adds m2 sized holes at the edge of the plate
                            // not that useful at the moment, needs to be
                            // moved more in to allow heat inserts to be 
                            // used
        "row": [ // options for the rows (not supported yet)
             
        ],
        "column": [ //options for the colums
            {
                "index": integer // column index (zero based)
                "offset": float // vertical offset (used to achieve vertical stagger)
                                // column needs to be the same key size
                                // rows need to have the same amount keys/spacers
            },
        ]
    }
}
```
