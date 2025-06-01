# Trading System

This is a simple trading system built using Rust and Axum. It provides APIs to add trading data in batches and retrieve statistical information about the trading data.

## Features

- Add trading data in batches.
- Retrieve statistical information such as minimum, maximum, average, variance, and the last value for a given symbol.

## Project Structure

```
src/
├── src/handlers.rs   # Contains API handlers for adding batches and retrieving stats.
├── src/main.rs       # Entry point of the application.
├── src/models.rs     # Defines data models for requests and responses.
├── src/state.rs      # Defines the application state.
```

## Dependencies

- Axum: Web framework for building APIs.
- Tokio: Asynchronous runtime for Rust.
- Serde: Framework for serializing and deserializing Rust data structures.
- Serde JSON: JSON serialization and deserialization.

## API Endpoints

### Add Batch

**Endpoint:** POST /add_batch  
**Description:** Adds a batch of trading data for a specific symbol in time complexity O(n).  
**Request Body:**
```json
{
  "symbol": "AAPL",
  "values": [123.45, 124.56, 125.67]
}
```
**Response:**
```json
{
  "message": "batch is added successfully"
}
```

### Get Stats

**Endpoint:** GET /stats?symbol={}&k={}  
**Description:** Retrieves statistical information for a specific symbol and batch size in time complexity O(1).  
**Query Parameters:**
- symbol: The trading symbol (e.g., AAPL).
- k: The batch size exponent (e.g., k=1 for 10^1 data points).

**Response:**
```json
{
  "min": 123.45,
  "max": 125.67,
  "last": 125.67,
  "avg": 124.56,
  "var": 0.49
}
```

## How to Run

1. Clone the repository:
   ```sh
   git clone https://github.com/AhmedM00/trading_system.git
   cd trading_system
   ```

2. Build the project:
   ```sh
   cargo build
   ```

3. Run the project:
   ```sh
   cargo run
   ```

4. The server will start on http://0.0.0.0:3000.

## License

This project is licensed under the MIT License.
