# setup.py
import pathlib
import shutil
import urllib.request
import getopt, sys
from typing import Tuple

# Remove 1st argument from the
# list of command line arguments
argumentList = sys.argv[1:]

# Options
options = "hdco:"

# Long options
long_options = ["help", "db_version=", "copy_db", "overwrite"]

def handle_command_line_arguments() -> Tuple[str, bool, bool]:
    db_version: str = "2.3.0" # Oldest DB version publicly available
    copy_db: bool = False
    overwrite: bool = False
    try:
        # Parsing argument
        arguments, values = getopt.getopt(argumentList, options, long_options)
        # checking each argument
        for currentArgument, currentValue in arguments:

            if currentArgument in ("-h", "--help"):
                print("This script downloads or creates necessary file to build BYBE. \n"
                      "Should be executed the first time the project is built in the machine or when resetting the database \n"
                      "Pass the --db_version or -d argument to input a specific BYBE-DB version to download (>= 2.3.0) \n"
                      "Pass the --overwrite or -c argument to download even if a BYBE-DB is already present \n"
                      "Pass the --copy or -c argument to temporary copy the database (both downloaded or already present). \n"
                      "This is used to enable Persistent startup (build with db copy and then delete the dirty db used to build)")
            elif currentArgument in ("-d", "--db_version"):
                db_version = currentValue
            elif currentArgument in ("-c", "--copy_db"):
                copy_db = True
            elif currentArgument in ("-o", "--overwrite"):
                overwrite = True
    except getopt.error:
        pass
    return db_version, copy_db, overwrite

def main():
    # Check if the file already exists or needs downloading
    x = handle_command_line_arguments()
    db_version: str = x[0]
    copy_enabled: bool = x[1]
    overwrite: bool = x[2]
    print(f"Using DB version: {db_version}")
    remote_url: str = f"https://github.com/RakuJa/BYBE-DB/releases/download/v{db_version}/database.db"
    database_file: str = "data/database.db"
    db_path = pathlib.Path(database_file)
    if overwrite:
        db_path.unlink(missing_ok=True)
    if not db_path.exists():
        print("Downloading the database file...")
        urllib.request.urlretrieve(remote_url, database_file)
    else:
        print("Database file already exists, skipping download.")
    if copy_enabled:
        destination_file: str = f"{database_file}_copy"
        shutil.copyfile(database_file, destination_file)
        print(f"Copied db to {destination_file}")

if __name__ == '__main__':
    main()
