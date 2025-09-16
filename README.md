# Cardea Web Search MCP Server

The Cardea Web Search MCP Server is a component of the Cardea project that provides web search capabilities through a Multi-Channel Protocol (MCP) server. It leverages the Tavily API to fetch and return web search results.

## Tools

- **search**
  - Perform a web search for the given query
  - Input parameters:
    - `query`: The query to search for
  - Output parameters:
    - `query`: The original query
    - `response_time`: The time taken to get the response
    - `results`: A list of search results, each containing:
      - `content`: The main content of the search result
      - `raw_content`: The raw content of the search result
      - `score`: The relevance score of the search result
      - `title`: The title of the search result
      - `url`: The URL of the search result

## Build

Let's build the mcp server and client by running the following commands:

```bash
# build mcp server
cargo build --package cardea-web-search-mcp-server --release
```

## Run

> [!IMPORTANT]
>
> Before running the mcp server, you need to set the Tavily API key. You can obtain a key from [Tavily](https://app.tavily.com/).
>
> ```bash
> export TAVILY_API_KEY=<your-api-key>
> ```

The CLI options of the mcp server are as follows:

```bash
Usage: cardea-web-search-mcp-server [OPTIONS]

Options:
  -s, --socket-addr <SOCKET_ADDR>  Socket address to bind to [default: 127.0.0.1:8010]
  -t, --transport <TRANSPORT>      Transport type to use (sse or stream-http) [default: stream-http] [possible values: sse, stream-http]
  -m, --max-results <MAX_RESULTS>  Max results to return [default: 5]
  -h, --help                       Print help
  -V, --version                    Print version
```

Now, let's start the mcp server by running the following command:

```bash
# run mcp server (stream-http)
./target/release/cardea-web-search-mcp-server --transport stream-http

# run mcp server (sse)
./target/release/cardea-web-search-mcp-server --transport sse
```

If started successfully, you will see the following output:

```bash
Starting Cardea Web Search MCP server on 127.0.0.1:8010
```
