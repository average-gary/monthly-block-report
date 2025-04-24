# Monthly Bitcoin Block Report Generator

A Rust-based tool that generates monthly reports of Bitcoin blocks, including detailed mining statistics and pool information. This tool queries a mempool.space instance to gather comprehensive block data and exports it to CSV format.

## Features

- Automatically fetches block data for the previous month
- Generates detailed CSV reports with block information including:
  - Block height
  - Timestamp
  - Mining reward
  - Total fees
  - Mining pool information
- Uses bulk block querying for efficient data retrieval
- Supports custom mempool.space instance configuration

## Prerequisites

- Rust (latest stable version)
- A mempool.space instance with bulk blocks enabled
- Environment variables (optional)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/average-gary/monthly-block-report.git
cd monthly-block-report
```

2. Build the project:
```bash
cargo build --release
```

## Configuration

The tool can be configured using environment variables or interactively:

1. Create a `.env` file in the project root (optional):
```bash
MEMPOOL_URL=your-mempool-instance-url
```

2. If no environment variable is set, the tool will prompt for the mempool URL at runtime.

## Usage

Run the tool:
```bash
cargo run --release
```

The tool will:
1. Determine the time range for the previous month
2. Query the mempool instance for block data
3. Generate a CSV report named `block_report_from_[start]_to_[end].csv`

## Output Format

The generated CSV file contains the following columns:
- `height`: Block height
- `timestamp`: Block timestamp
- `reward`: Mining reward
- `fees`: Total fees
- `pool`: Mining pool identifier

## Notes

- The mempool instance you're querying against must have bulk blocks enabled
- The tool uses the mempool.space API to fetch block data
- Reports are generated in CSV format for easy analysis
- The tool automatically handles timezone conversions and date calculations

## License

MIT License

Copyright (c) [year] [fullname]

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. 