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
                selected_file = Filepath(filepath)
                return selected_file
            else:
                print("No file selected.")
                return None
        if args[1] == "path":
            if Filepath is not None:
                print(f"Current file path: {Filepath}")
            else:
                print("No file path set.")