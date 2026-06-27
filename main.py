from src import parse_args as pa
import pandas as pd
import sys
import os




while True:
    user_input = input("> ")
    if user_input.lower() == "exit":
        print("Exiting the program.")
        sys.exit(0)
    else:
        pa.main(user_input.split(" "))