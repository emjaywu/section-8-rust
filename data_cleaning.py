## DS 210 Supplement - Data Cleaning
#  Will use relative path to import data, then parse out rows with missing OwnerType variable
#  Link to dataset: https://catalog.data.gov/dataset/subsidized-housing-six-metro-areas-2017-18653

from pathlib import Path 
import pandas as pd

# Instantiate constants (yes, I used ChatGPT to help me navigate this)
BASE = Path(__file__).parent
DATA_DIR = BASE / "data"
RAW_CSV = DATA_DIR / "Subsidized_Housing_-_Six_Metro_Areas_-_2017.csv"
CLEAN_CSV = DATA_DIR / "cleaned_subsidized_housing.csv"

# Load and clean relevant data (again, used ChatGPT here)
cols = ["TotalUnits", "ActiveSubs", "Latitude", "Longitude", "OwnerType"] # using 5/17 variables
df = pd.read_csv(RAW_CSV, na_values = [''], keep_default_na = True)
df5 = df[cols].copy() # making a new DataFrame

# Convert blank/white-space entries to NaN for filtering
for c in cols:
	if df5[c].dtype == object:
		df5[c] = (df5[c].astype(str).str.strip().replace('', pd.NA))
df_clean = df5.dropna().reset_index(drop = True)

# Return it to the data folder (and here^^)
df_clean.to_csv(CLEAN_CSV, index = False)
print(f"Wrote {df_clean.shape[0]}x{df_clean.shape[1]} to {CLEAN_CSV}")

'''
print(df_clean.shape, "\n")
print(df_clean.isna().sum(), "\n")
print(df_clean.head(20), "\n")
print(df_clean.tail(20), "\n")

# provide output path
outpath_path = "C://"
'''

# turn blank or whitespace-only OwnerType into real Nan
# df['OwnerType'] = (df['OwnerType'].astype(str).str.strip().replace('', pd.NA))

# see how many missing values each of the five columns has
# print(df[cols].isna().sum())

# peek at the rows that have any missing among those five
# print(df[df[cols].isna().any(axis = 1)])

# drop every row with missing in any of the five
# df_clean = df.dropna(subset = cols).reset_index(drop = True)

# verify
# print("Before:", df.shape, "→ After:", df_clean.shape)