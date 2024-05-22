FROM alpine/curl

COPY . .

ENTRYPOINT ["curl", "http://llm:11434/api/pull", "-d", "@llm/req.json", "--header", "Content-Type: application/json"]
