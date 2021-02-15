# Keyboard plate prototyping (lib)

* For usage look at either the [cli](http://github.com/elasticrash/keyboard/tree/master/cli) and/or the [ui](http://github.com/elasticrash/keyboard/tree/master/ui) examples
* [output examples](http://github.com/elasticrash/keyboard/tree/master/samples)
* configuration examples 
    * [1. 60%](http://github.com/elasticrash/keyboard/blob/master/generic60.json)
    * [2. Ortho](https://github.com/elasticrash/keyboard/blob/master/ortho40.json)
    * [3. example design](https://github.com/elasticrash/keyboard/blob/master/tougo.json)
    * [4. example design/column stagger](https://github.com/elasticrash/keyboard/blob/master/tougo_cs.json)

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

## Supported exports

* Dxf
* Ply