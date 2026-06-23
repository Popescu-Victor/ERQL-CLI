import pandas as pd


class DataFrameEditor:
    def __init__(self, df: pd.DataFrame):
        self.df = df

    def edit_cell(self, row_index: int, column_name: str, new_value):
        if row_index < 0 or row_index >= len(self.df):
            raise IndexError("Row index is out of bounds.")
        if column_name not in self.df.columns:
            raise KeyError(f"Column '{column_name}' does not exist in the DataFrame.")
        
        self.df.at[row_index, column_name] = new_value