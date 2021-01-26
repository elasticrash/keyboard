# Keyboard plate prototyping (lib)

* For usage look at either the [cli](http://github.com/elasticrash/keyboard/tree/master/cli) and/or the [ui](http://github.com/elasticrash/keyboard/tree/master/ui) examples
* [output examples](http://github.com/elasticrash/keyboard/tree/master/samples)
* configuration examples 
    * [1. 60%](http://github.com/elasticrash/keyboard/blob/master/generic60.json)
    * [2. Ortho](https://github.com/elasticrash/keyboard/blob/master/ortho40.json)
    * [3. my keyboard design](https://github.com/elasticrash/keyboard/blob/master/tougo.json)


## How to make your own layout

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

## Supported exports

* Dxf
* Ply