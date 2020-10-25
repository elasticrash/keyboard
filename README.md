This project is just an idea to automate plate and sandwich case prototyping for 3d printing

How to run

cargo run {keyboard}

i.e cargo run generic60

D: exports plate to dxf

P: exports plate in ply (default thickness 30mm)


Instructions how to make your own layout.

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