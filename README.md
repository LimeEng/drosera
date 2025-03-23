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

## Table of Contents
- [Installation](#installation)
- [Usage](#usage)
- [Risks](#risks)

## Installation

Install drosera by either grabbing a [pre-built binary](https://github.com/LimeEng/drosera/releases) or by running one of these commands.

```sh
cargo install drosera
cargo install --git https://github.com/LimeEng/drosera
```

## Usage

To view the available options and usage details, execute the `drosera` binary. Below is an example of the output:

```sh
Tarpit SSH server

Usage: drosera [OPTIONS]

Options:
  -s, --socket_addr <socket_addr>
          The socket address to bind to [default: 127.0.0.1:22]
  -m, --max_connections <max_connections>
          The maximum number of connections maintained at once [default: 1024]
  -d, --delay <delay>
          Approximately wait this long before sending more data (in milliseconds) [default: 10000]
  -h, --help
          Print help
  -V, --version
          Print version
```

## Risks

Running tarpits servers at all might [not be a good idea](https://serverfault.com/questions/611063/does-tarpit-have-any-known-vulnerabilities-or-downsides). An adversary might even be able to exploit a vulnerability in drosera to gain access to the machine it's running on. Adversaries could also launch massive denial-of-service attacks, quickly consuming what little memory is available and crashing the server.
