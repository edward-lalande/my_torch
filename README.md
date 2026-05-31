# Chess State Predictor

A from-scratch PyTorch-like deep learning in Rust with the ultimate goal of detecting and analyzing chessboard states.

**Current Status**: V1 — Single-layer Perceptron learning basic logic gates (AND gate proof-of-concept)

---

## 📋 Overview

This project is a stepping stone toward a full ML library:
- **V1**: Simple Perceptron with the Perceptron Learning Rule
- Binary serialization for model persistence
- Proof-of-concept training on linearly separable data (AND gate)
- Foundation for scaling to MLPs with proper backpropagation (V2)

The codebase is organized into two binaries that work in tandem:
- **`my_torch_generator`** — Initializes and serializes network architecture
- **`my_torch_analyzer`** — Loads models, trains, and predicts

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

Creates a serialized binary file (`perceptron.nn`) from your JSON configuration.

**Example `config.json`:**
```json
{
  "inputs": 2,
  "hidden_layers": 2,
  "hidden_neurons": 50,
  "outputs": 1,
  "epochs": 30,
  "learning_rate": 0.01,
  "weights": [0.0, 0.0],
  "bias": 0.0
}
```

> **Note:** Always initialize `weights` and `bias` to `0.0`. The analyzer updates these dynamically during training.

### 2️⃣ Train & Predict

```bash
cd ../my_torch_analyzer
cargo build --release
./target/release/my_torch_analyzer --load path/to/model.nn
```

Loads your model, runs the training loop (`fit`), and outputs predictions.

---

## 📁 Project Structure

```
chess-state-predictor/
├── my_torch_generator/
│   ├── src/
│   │   └── main.rs          # Initializes & serializes networks
│   ├── config.json              # Network architecture config
│   └── Cargo.toml
├── my_torch_analyzer/
│   ├── src/
│   │   └── main.rs          # Training & inference engine
│   └── Cargo.toml
└── README.md
```

---

## ⚙️ Configuration Guide

| Field | Type | Purpose |
|-------|------|---------|
| `inputs` | int | Number of input features |
| `hidden_layers` | int | Number of hidden layers (V1: **not used yet**) |
| `hidden_neurons` | int | Neurons per hidden layer (V1: **not used yet**) |
| `outputs` | int | Number of output nodes |
| `epochs` | int | Training iterations |
| `learning_rate` | float | Gradient descent step size (default: 0.01) |
| `weights` | array | Initial weights (leave as `[0.0, ...]`) |
| `bias` | float | Initial bias (leave as `0.0`) |

---

## 🎯 Roadmap

| Version | Status | Features |
|---------|--------|----------|
| **V1** | ✅ Current | Simple Perceptron, Perceptron Learning Rule, AND gate training, binary serialization |
| **V2** | 📋 Planned | Full MLP, ReLU/Sigmoid activations, matrix math, backprop |
| **V3** | 🔮 Goal | Chess board state detector trained on real game data |

---

## 🔧 Technical Details

### Training Mechanism
- **Forward pass**: Linear combination (z = w·x + b) + step function activation
- **Weight updates**: Perceptron Learning Rule
  ```
  error = target - prediction
  weights += learning_rate × error × input
  bias += learning_rate × error
  ```
- No gradients or backpropagation (planned for V2)

### Serialization
- Binary format using `serde` + `bincode`
- Compact model persistence and fast loading
- Platform-independent `.nn` file format

### V1 Capabilities
- Perceptron algorithm for linearly separable problems
- Training on AND gate (proof of concept)
- Simple step function activation (z ≥ 0.0)
- Configurable epochs and learning rate

---

## 📊 Example: Training AND Gate

```json
{
  "inputs": 2,
  "hidden_layers": 1,
  "hidden_neurons": 4,
  "outputs": 1,
  "epochs": 100,
  "learning_rate": 0.5,
  "weights": [0.0, 0.0],
  "bias": 0.0
}
```

**Expected Output:**
```
Perceptron parameters: {{ The parametre of your perceptron config file }}
0 & 0 = 0
1 & 0 = 0
0 & 1 = 0
1 & 1 = 1
```

---

## 🐛 Troubleshooting

| Issue | Solution |
|-------|----------|
| File not found | Verify path is relative to binary location |
| Parse error | Check JSON syntax in `config.json` |

---

## 📚 References

- Backpropagation: https://en.wikipedia.org/wiki/Backpropagation
- Rust Serialization: https://serde.rs/
- PyTorch Docs: https://pytorch.org/docs/
