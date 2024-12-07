# setup.py
import os
import urllib.request
import getopt, sys
from typing import Optional

# Remove 1st argument from the
# list of command line arguments
argumentList = sys.argv[1:]

# Options
options = "hd:"

# Long options
long_options = ["help", "db_version="]

def handle_command_line_arguments() -> Optional[str]:
    try:
        # Parsing argument
        arguments, values = getopt.getopt(argumentList, options, long_options)

        # checking each argument
        for currentArgument, currentValue in arguments:

            if currentArgument in ("-h", "--help"):
                print("This script downloads or creates necessary file to build BYBE. \n"
                      "Should be executed the first time the project is built in the machine or when resetting the database \n"
                      "Pass the --db_version or -d argument to input a specific BYBE-DB version to download (>= 2.3.0)")
            elif currentArgument in ("-d", "--db_version"):
                return currentValue
    except getopt.error:
        pass

def main():
    # Check if the file already exists or needs downloading
    db_version: str = handle_command_line_arguments()
    print(f"Using DB version: {db_version}") or "2.3.0" # Oldest DB version publicly available
    remote_url: str = f"https://github.com/RakuJa/BYBE-DB/releases/download/v{db_version}/database.db"
    destination_file: str = "database.db"
    if not os.path.exists(destination_file):
        print("Downloading the database file...")
        urllib.request.urlretrieve(remote_url, destination_file)
    else:
        print("Database file already exists, skipping download.")

if __name__ == '__main__':
    main()
