import os
import glob
import shutil

def main():
    file_list = []
    file_list.extend( glob.glob( "./src/**/*.rs" , recursive=True) )
    print( file_list )

    out_file = "Cargo.toml"
    shutil.copy("Cargo_template.toml" , out_file)

    with open( out_file ,"a") as file:


        for i in file_list:
            path = i
            if not "lib" in path:
                name = i.split("/")[-2]+ "_" + i.split("/")[-1].split(".")[0]
                file.write( "\n\n")
                file.write( "[[bin]]\n")
                file.write( "name = \"{}\"\n".format( name ) )
                file.write( "path = \"{}\"\n".format( path ) )


if __name__ == '__main__':
    main()