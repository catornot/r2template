# r2template
basic templating tool for titanfall 2 mods

## Usage
1. setup author : `r2template -a author`
2. create boiler plate: `r2template -n mod_name`
3. you can specify the template by adding `-t` parameter

### Templates
currently by default the following templates are included
- server-side -> server side boilerplate
- weapon-keyvalues -> just weapon keyvalues; yeah; probably broken too
- client-side -> client side boilerplate
- shared -> client side and server side in one file (shared) boilerplate
- full -> everything in one neat package

## Building
the executable needs to be included with the templates folder and it should cross platform

### Windows
1. get rustup
2. build it in release
3. put the exe with templates folder
4. done
5. add it to path

### Linux
idk help me