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
docker build -t goodreads-genres-api .
```

And test the build:

```bash
docker run -p8080:8080 goodreads-genres-api
```

## Usage

Once deployed you need to supply a comma-separated list of genre slugs:

```
/?genres=science-fiction,young-adult-science-fiction
```

By default the returned list will be sorted by `rating_count` in descending order. However, if you want to sort by `rating_avg` in descending order you can specify the `sort_by` parameter:

```
/?genres=science-fiction&sort_by=avg
```

You can also filter the list to only include Books with a `min_avg` and `min_count`:

```
/?genres=science-fiction&min_avg=4.2&min_count=5000
```