#!/usr/bin/env python3

import os
from glob import glob
import pandas as pd
import shapefile


def load(path):
    def inner(p):
        base = os.path.basename(p)
        bare = os.path.splitext(base)[0]
        full = os.path.join(p, bare) + ".dbf"
        s = shapefile.Reader(full)
        return pd.DataFrame(s.records(), columns=[i[0] for i in s.fields[1:]])

    if isinstance(path, list):
        return pd.concat(inner(p) for p in path)

    return inner(path)


def load_places():
    df = load(glob("place/*.zip"))
    df["INTPTLAT"] = df["INTPTLAT"].apply(pd.to_numeric)
    df["INTPTLON"] = df["INTPTLON"].apply(pd.to_numeric)
    return df[["STATEFP", "INTPTLAT", "INTPTLON", "NAME"]]


def load_states():
    df = load("tl_2022_us_state.zip")
    df["STATE_NAME"] = df["NAME"]
    del df["NAME"]
    return df[["STATEFP", "STATE_NAME", "STUSPS"]]


def main():
    places = load_places()
    states = load_states()
    res = places.merge(states, on="STATEFP")
    del res["STATEFP"]
    res.to_csv("places.csv", index=False)


if __name__ == "__main__":
    main()
