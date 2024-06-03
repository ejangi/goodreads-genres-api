# Goodreads Genres API

I love looking up various [Goodreads](https://www.goodreads.com) Genre pages to see which books are *most read* each week.

However, I also really want to filter the list to only show high ranking and highly reviewed books.

This little rust app helps me to do that for the Genres I care about.

## Development

I've included a `docker compose` development environment. To run the tests, simply run:

```bash
docker compose build
docker compose up
```

## Production

To build the production docker image, you can run something like:

```bash
docker build -t goodreads-groups-api .
```

And test the build:

```bash
docker run -p8080:8080 goodreads-groups-api
```