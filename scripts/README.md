These scripts download and prepare data files containing the latitude and longitude of cities within the
United States. The files are provided by the [census bureau][data-files].

`download-place.sh` and `download-state.sh` download the files.

`merge.py` merges the state data with the place data.

`run.sh` both downloads the files and merges them.


[data-files]: https://www2.census.gov/geo/tiger/TIGER2022/
