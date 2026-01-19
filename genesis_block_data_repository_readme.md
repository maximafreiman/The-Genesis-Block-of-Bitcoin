# Bitcoin Genesis Block Data Repository

This repository contains **raw and structured data representations of the Bitcoin Genesis Block (Block 0)**, intended for direct inspection, verification, and reference using Bitcoin Core GUI and RPC Console. The focus is on **full block content and file formats**, not conceptual explanations.

---

## Repository Structure

```
/
├─ README.md
├─ data/
│  ├─ genesis_block.json
│  └─ genesis_block.raw
```

---

## Files

### `data/genesis_block.json`

- Format: **JSON**
- Source: Output from Bitcoin Core RPC
- Command used:

```
getblock <genesis_block_hash>
```

- Description: Structured, human-readable representation of the Genesis Block, including header fields, transaction list, and metadata.

---

### `data/genesis_block.raw`

- Format: **Raw hex**
- Source: Bitcoin Core RPC
- Command used:

```
getblock <genesis_block_hash> 0
```

- Description: Full raw block in hexadecimal format, suitable for low-level parsing and protocol-level inspection.

---

## RPC Verification (Bitcoin Core GUI)

Use the following steps to verify the data in this repository against your own node.

1. Open Bitcoin Core
2. Navigate to **Help > Debug Window > RPC Console**
3. Run:

```
getblockhash 0
```

4. Copy the returned hash
5. Run:

```
getblock <hash>
```

6. For raw hex output, run:

```
getblock <hash> 0
```

Compare the output with the contents of `genesis_block.json` and `genesis_block.raw`.

---

## Data Format Notes

- All structured output from Bitcoin Core RPC is returned in **JSON (JavaScript Object Notation)**.
- The raw block file contains a **byte-for-byte hexadecimal encoding** of the Genesis Block as transmitted over the Bitcoin P2P protocol.

---

## Compatibility

- Bitcoin Core GUI
- JSON-RPC interface
- Any JSON-compatible parser or hex parser

---

## License

This repository is provided for reference and documentation purposes. You may copy, modify, and redistribute it with proper attribution to the original source or repository.

