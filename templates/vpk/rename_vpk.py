import os

vpk_name = "mp_map"

target = "pak000_dir"
result = f"englishclient_{vpk_name}.bsp.pak000_dir"
path = os.getcwd() # relative path

for file in os.listdir( path ):
    if file.startswith( target ):
        renamed_file = file.replace( target, result )
        os.rename( os.path.join( path, file), os.path.join( path, renamed_file) )
        print( f" renamed {file} to {renamed_file}" )

target = "pak000_000"
result = f"client_{vpk_name}.bsp.pak000_000"

for file in os.listdir( path ):
    if file.startswith( target ):
        renamed_file = file.replace( target, result )
        os.rename( os.path.join( path, file), os.path.join( path, renamed_file) )
        print( f" renamed {file} to {renamed_file}" )
