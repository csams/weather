# Weather CLI

This is a weather utility for the CLI that provides hourly and 7 day forecasts for the United States.

It first uses the [Geocoding Services API][geocoding] of [census.gov][census] to convert free text street
addresses to latitudes and longitudes. It then uses those coordinates with
`https://api.weather.gov/points/{lat},{lon}` to get the corresponding forecast grid. Finally, it passes the
grid info to [api.weather.gov][api] to retrieve the forecast.

You can pass the address query as a string argument or set the environment variable `WX_DEFAULT_ADDRESS` as a
fallback.

```bash
# week forecast
weather "4600 Silver Hill Rd, Washington, DC 20233"

# hourly forecast
weather --hourly "4600 Silver Hill Rd, Washington, DC 20233"
```

This is a simple project for me to learn rust. It doesn't cache anything between invocations, and error
reporting isn't very good.


[api]: https://weather-gov.github.io/api/general-faqs
[geocoding]: https://geocoding.geo.census.gov/geocoder/Geocoding_Services_API.html
[census]: https://census.gov
