# ZAI Web Reader MCP Server for Zed

This extension integrates [z.ai Web Reader](https://z.ai) as a Model Context Protocol (MCP) server for Zed's Assistant, providing web content extraction capabilities directly in your prompts.

## What is z.ai Web Reader?

z.ai Web Reader fetches the complete content of any webpage, including text, links, and structured data such as title, main body, and metadata.

### ✅ With Web Reader

- Fetch full-page content from any URL
- Extract structured data (title, body, metadata, links)
- Use as reference for bug resolution, documentation reading, and knowledge extraction

## Installation

This extension can be installed from the Zed extension registry.

## Agent Mode Configuration

If you're using Zed's agent mode, you need to enable this context server for your assistant:

1. Open Zed's assistant settings
2. Enable the ZAI Web Reader MCP server. If you see that the status of the tool is a red dot, make sure you toggle it so that becomes green.
3. Enable the ZAI Web Reader MCP Server in the active assistant profile. In the chat section, click on the `Write | Ask` button, then click on `tools`, then enable the ZAI Web Reader MCP Server.

## API Key Configuration

You need a z.ai API key to use this extension.

Add your API key in the extension settings:

```json
{
  "context_server": {
    "mcp-server-zai-web-reader": {
      "source": "extension",
      "enabled": true,
      "settings": {
        "zai_api_key": "YOUR_ZAI_API_KEY"
      }
    }
  }
}
```

## Usage

Reference URLs in your prompt and the LLM will fetch and read the content:

- `Read the content of https://docs.z.ai/guides/overview/quick-start and summarize it`
- `Fetch the README from https://github.com/rust-lang/rust and extract key points`
- `What does https://example.com say about their API pricing?`

## Available Tools

The ZAI Web Reader MCP Server provides this tool to the LLM:

- **`webReader`** — Fetch webpage content for a specified URL. Returns the page title, main content, metadata, list of links, and more.

## How It Works

The extension uses [supergateway](https://github.com/supercorp-ai/supergateway) to bridge the remote Streamable HTTP MCP server at `https://api.z.ai/api/mcp/web_reader/mcp` to a local stdio connection that Zed can communicate with.

## Development

Clone the project and build:

```bash
cargo build
```

## License

MIT