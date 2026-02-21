[![CI status](https://github.com/LimeEng/drosera/actions/workflows/ci.yaml/badge.svg)](https://github.com/LimeEng/drosera/actions/workflows/ci.yaml)
[![Latest version](https://img.shields.io/crates/v/drosera?color=blue)](https://crates.io/crates/drosera)

# Drosera

Drosera is a SSH tarpit server. Inspired by [endlessh](https://nullprogram.com/blog/2019/03/22/).

When initiating a SSH session, the server first sends a version string to the client before communication begins. However, tarpits like drosera can exploit the following paragraph in the SSH specification, found in [RFC 4253](https://tools.ietf.org/html/rfc4253#page-4):
```
The server MAY send other lines of data before sending the version
string.  Each line SHOULD be terminated by a Carriage Return and Line
Feed.  Such lines MUST NOT begin with "SSH-", and SHOULD be encoded
in ISO-10646 UTF-8 [RFC3629] (language is not specified).  Clients
MUST be able to process such lines.
```

It is thus incredibly easy to trap ill-configured clients in a tarpit by simply never sending the version string. To keep the connection alive, some data should be sent periodically.

Drosera, like its [beautiful namesake](https://en.wikipedia.org/wiki/Drosera), is designed to thrive in memory-constrained environments while feeding on the numerous blood-sucking drones seeking prey on the Internet.

## Installation

Install drosera from [crates.io](https://crates.io/crates/drosera) with `cargo install drosera` or by grabbing a [pre-built binary](https://github.com/LimeEng/drosera/releases).
