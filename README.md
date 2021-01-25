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


## How to make your own layout.

Layouts are in json format the idea is that you have

an object with a single property

 "layout": [] which is an array of arrays 
 "options": [] which is list of overridable values

 options at the moment include
 * plate_height: how thick the plate needs to be, defaults to 20mm

each array element is a row and each object within it is a key

keys have 3 properties

* size in units (1 = 1U)
* char (just for the preview)
* k_type 1 for visible and 0 for hidden