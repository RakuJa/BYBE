# setup.py
import os
import pathlib
import getopt, sys
import shutil

# Remove 1st argument from the
# list of command line arguments
argumentList = sys.argv[1:]

# Options
options = "hn:"

# Long options
long_options = ["help", "no_clean"]

def handle_command_line_arguments() -> bool:
    no_cleanup: bool = False
    try:
        # Parsing argument
        arguments, values = getopt.getopt(argumentList, options, long_options)

        # checking each argument
        for currentArgument, _currentValue in arguments:

            if currentArgument in ("-h", "--help"):
                print("This script is used as a post build cleanup script for BYBE. \n"
                      "Should be executed every time a Persistent build (app) needs to be mad as it ensure a correct DB is used. \n"
                      "Pass the --no-clean or -n argument to avoid deleting files (db used for builds)")
            elif currentArgument in ("-n", "--no_clean"):
                no_cleanup = True
    except getopt.error:
        pass
    return no_cleanup

def main():
    # Check if the file already exists or needs downloading
    print("Executing post build script...")
    no_cleanup: bool = handle_command_line_arguments() is True
    clean_db: str = "database.db_copy"
    dirty_db: str = "database.db"
    if os.path.exists(clean_db):
        print("Restoring clean DB file...")
        pathlib.Path(dirty_db).unlink(missing_ok=True)
        shutil.copyfile(clean_db, dirty_db)
    else:
        print("Cannot restore clean DB, will use the dirty DB used for build...")
    if not no_cleanup:
        print("Cleaning up DB copy...")
        pathlib.Path(clean_db).unlink(missing_ok=True)


if __name__ == '__main__':
    main()
