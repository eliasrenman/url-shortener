## Running

### Docker
To run this in docker please run:

```Bash
docker build -t url-shortener .
```
Copy and fill out a Rocket.toml from the Rocket.toml.example

Generate a secret for rocket
```Bash
openssl rand -base64 32
```
Start docker container with:
```bash
docker run -p 8000:8000 \
  -e DATABASE_URL=/app/sqlite.db \
  -e BASE_URL=https://url.renman.dev \
  -e JWT_SECRET=secret \
  -e ROCKET_SECRET_KEY={OUTPUT_OF_PREVIOUS_STEP} \
  -v $(pwd)/sqlite.db:/app/sqlite.db \
  -v $(pwd)/Rocket.toml:/app/Rocket.toml \
  url-shortener
```
