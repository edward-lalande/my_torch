# Chess State Predictor

A from-scratch PyTorch-like deep learning in Rust with the ultimate goal of detecting and analyzing chessboard states.

**Current Status**: V2 — Full MLP with backpropagation, LeakyReLU, and chess FEN classification

---

## 📋 Overview

This project is a stepping stone toward a full ML library:
- **V1**: Simple Perceptron with the Perceptron Learning Rule
- **V2**: Full MLP — layered architecture, backpropagation, LeakyReLU, MSE loss, trained on real chess data
- Binary serialization for model persistence
- Chess board state classification from FEN notation

The codebase is organized into two binaries that work in tandem:
- **`my_torch_generator`** — Initializes and serializes network architecture from a config file
- **`my_torch_analyzer`** — Loads models, trains, and runs predictions

---

## 🚀 Quick Start

### Prerequisites
- Rust 1.70+
- Cargo

### 1️⃣ Generate Model Architecture

```bash
cd my_torch_generator
cargo build --release
./target/release/my_torch_generator config.json
```

Creates a serialized binary file (`hyper_parameters.nn`) from your JSON configuration.

**Example `config.json`:**
```json
{
  "inputs": 64,
  "hidden_layers": 2,
  "hidden_neurons": 128,
  "outputs": 5,
  "epochs": 30,
  "learning_rate": 0.01
}
```

### 2️⃣ Train

```bash
cd ../my_torch_analyzer
cargo build --release
./target/release/my_torch --train --save model.nn hyper_parameters.nn dataset/check/10_pieces.txt
```

### 3️⃣ Predict

```bash
./target/release/my_torch --predict model.nn dataset/check/10_pieces.txt
```

---

## 📁 Project Structure

```
my_torch/
├── my_torch_generator/
│   ├── src/
│   │   └── main.rs
│   ├── config.json
│   └── Cargo.toml
├── my_torch_analyzer/
│   ├── src/
│   │   ├── main.rs
│   │   └── structs/
│   │       ├── mlp.rs                    # MLP orchestration (forward, backprop, fit, predict)
│   │       ├── layers.rs                 # Layer abstraction over neurons
│   │       ├── neuron.rs                 # Neuron: LeakyReLU, Xavier init, delta computation
│   │       ├── chess_fen_notation.rs     # FEN parser → Vec<f64>
│   │       ├── enum_chess_prediction.rs  # 5-class output enum
│   │       ├── hyper_parameters.rs       # Config deserialization
│   │       └── args.rs                   # CLI argument parsing
│   └── Cargo.toml
├── dataset/
│   ├── check/
│   ├── checkmate/
│   └── nothing/
└── README.md
```

---

## ⚙️ Configuration Guide

| Field | Type | Purpose |
|-------|------|---------|
| `inputs` | int | Number of input features (64 for a full chess board) |
| `hidden_layers` | int | Number of hidden layers |
| `hidden_neurons` | int | Neurons per hidden layer |
| `outputs` | int | Number of output classes (5 for chess state classification) |
| `epochs` | int | Training iterations |
| `learning_rate` | float | Gradient descent step size |

---

## 🎯 Roadmap

| Version | Status | Features |
|---------|--------|----------|
| **V1** | ✅ Done | Simple Perceptron, Perceptron Learning Rule, AND gate training, binary serialization |
| **V2** | ✅ Current | Full MLP, LeakyReLU, Xavier init, backpropagation, MSE loss, chess FEN classification |

---

## 🔧 Technical Details

### Architecture

Three-level hierarchy:

- **`Neuron`** — Core unit. Xavier weight initialization (`sqrt(6/n)`), LeakyReLU activation (slope 0.01 for x < 0), delta computation for both output and hidden layers.
- **`Layers`** — Aggregates neurons. Exposes `forward_layer`, `compute_output_deltas`, `compute_hidden_deltas` (upstream error via next layer weights), `update_layer_weights`.
- **`MLP`** — Orchestrates layers. Runs forward pass with full activation history, backpropagation cascade from output to input, weight updates from history, `fit` loop with MSE loss + accuracy logging, `predict`, and `save`/load via `bincode`.

### Training Mechanism

**Forward pass**
```
z = w·x + b
output = LeakyReLU(z)   →   x > 0 ? x : 0.01 * x
```

**Backpropagation**
```
# Output layer
delta = (target - output) * LeakyReLU'(output)

# Hidden layers (upstream error)
upstream_error = Σ (next_weight[i] * next_delta)
delta = upstream_error * LeakyReLU'(output)

# Weight update
weight += learning_rate * delta * input
bias   += learning_rate * delta
```

**Loss** — MSE averaged per sample, logged each epoch alongside accuracy.

### Chess Input Encoding

FEN board strings are parsed into a 64-element `Vec<f64>`:

| Piece | Value |
|-------|-------|
| White King `K` | -1.0 |
| White Queen `Q` | -0.9 |
| White Rook `R` | -0.5 |
| White Bishop `B` | -0.35 |
| White Knight `N` | -0.3 |
| White Pawn `P` | -0.1 |
| Empty | 0.0 |
| Black Pawn `p` | 0.1 |
| Black Knight `n` | 0.3 |
| Black Bishop `b` | 0.35 |
| Black Rook `r` | 0.5 |
| Black Queen `q` | 0.9 |
| Black King `k` | 1.0 |

### Output Classes

5-class one-hot classification:

| Index | Class |
|-------|-------|
| 0 | Nothing |
| 1 | Check White |
| 2 | Check Black |
| 3 | Checkmate White |
| 4 | Checkmate Black |

### Serialization
- Binary format using `serde` + `bincode`
- The generator produces either a bare `HyperParameters` file or a full `MLP` — the analyzer distinguishes them on load and handles both.

---

## 📊 Example Training Output

```
> ./target/release/my_torch --train --save model.nn hyper_parameters.nn dataset/check/10_pieces.txt
-> Initialization of a new MLP.
-> Epoch [01/30] | Loss: 0.3821 | Accuracy: 41.20%
-> Epoch [02/30] | Loss: 0.2914 | Accuracy: 58.75%
...
-> Epoch [30/30] | Loss: 0.0423 | Accuracy: 94.10%
-> Progress saved in: (model.nn).
```

---

## 🐛 Troubleshooting

| Issue | Solution |
|-------|----------|
| File not found | Verify path is relative to binary location |
| Parse error | Check JSON syntax in `config.json` |
| Model/param mismatch | Ensure the `.nn` file was generated with matching architecture |

---

## 📚 References

- Backpropagation: https://en.wikipedia.org/wiki/Backpropagation
- Xavier Initialization: https://proceedings.mlr.press/v9/glorot10a.html
- Rust Serialization: https://serde.rs/
- PyTorch Docs: https://pytorch.org/docs/
