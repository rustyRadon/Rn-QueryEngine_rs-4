# Rn-QueryEngine_rs-4
# Radon-V4: Vectorized Relational Hash-Join Engine

Radon-V4 is a high-performance OLAP (Online Analytical Processing) engine built in **Pure Rust** with **Zero Dependencies**. It demonstrates how modern databases handle millions of relational joins using memory-mapped binary data and vectorized execution.

##  Architecture & Design
Unlike row-based databases (PostgreSQL/MySQL), Radon-V4 uses a **Columnar Storage** model.

### 1. The Hash-Join Algorithm (Build & Probe)
Instead of a slow Nested Loop ($O(N^2)$), we use a Hash Join ($O(N)$):
* **Build Phase:**  `departments.json` is parsed into a `HashMap<i32, String>`.
* **Probe Phase:**  the `dept_id.bin` file is streamed. For every row, a constant-time $O(1)$ lookup in the Map is performed.

### 2. Zero-Copy Deserialization
used `unsafe` pointer casting to map `.bin` files directly into Rust `Vec<i32>` and `Vec<f64>`. This bypasses the overhead of traditional file parsing, achieving near-hardware-limit speeds.

### 3. BitMask Filtering
We use a custom bitset to track which rows pass the salary filter. This allows us to skip the "Join" operation for rows that don't match our criteria, saving millions of CPU cycles.

##  Performance
By utilizing `std::simd` (implicit) and avoiding heap allocations inside the hot loop, this engine can filter and join 1,000,000 rows in sub-millisecond time.