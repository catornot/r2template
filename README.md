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
- maps -> basic folder structure and files for new maps
- icon -> adds a icon to the folder for packing into thunderstore mods
- maps-icon -> maps and icon in one

## Compiling from source
the executable needs to be included with the templates folder and it should cross platform

### Windows
```powershell
>> winget install rustup 
# install if rust isn't installed
>> git clone https://github.com/catornot/r2template.git
>> cd r2template/
>> cargo install --path .
```

### Linux
```bash
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# add ~/.cargo/bin to path if using fish
$ git clone https://github.com/catornot/r2template.git
$ cd r2template/
$ cargo install --path .
$ cp ./templates ~/.cargo/bin/
```
