print("Welcome to the ERQL CLI. Type 'exit' to quit.")


def main(args):

    if len(args) == 0:
        print("No arguments provided. Please provide a command.")
        return
    if args[0] == "file":
        if args[1] == "select":
            from src.args_file import Filepath
            from tkinter import filedialog
            filepath = filedialog.askopenfilename(title="Select a file", filetypes=[("CSV files", "*.csv")])
            if filepath:
                print(f"Selected file: {filepath}")
                global selected_file
                selected_file = Filepath(filepath)
                return selected_file
            else:
                print("No file selected.")
                return None
        if args[1] == "path":
            if selected_file is not None:
                print(f"Current file path: {selected_file}")
            else:
                print("No file path set.")

    if args[0] == "dataframe":
        if args[1] == "new":
            from src import dataframes
            col_list = input("Write the names of the columns separated by comma >>  ").split(',')
            global df
            df = dataframes.CLI_Dataframe(col_list)

        if args[1] == "data":
            user_input = input(">>")
            while user_input != "done":
                df.add_row(input(">>").split(','))
        if args[1] == "cols":
            print(df.head())