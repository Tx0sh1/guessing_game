
# 🎯 Number Guessing Game (Rust)

A simple CLI game built in Rust where the player has to guess a randomly generated number between 1 and 100. You only get 5 lives—guess wrong too many times and it's game over!

This was built as part of my Rust learning journey, to practice user input, conditionals, loops, and handling basic game logic.

---

## 🕹️ How to Play

1. Run the game.
2. Enter a number between 1 and 100.
3. If your guess is too low or too high, you lose a life.
4. You have 5 lives total—guess correctly before they run out!

---

## 🛠 Built With

- [Rust](https://www.rust-lang.org/)
- [`rand`](https://crates.io/crates/rand) crate for random number generation
- Standard library for input/output

---

## 🚀 Getting Started

Make sure you have Rust installed. Then:

```bash
git clone https://github.com/Tx0sh1/guessing_game
cd guessing-game
cargo run
```

---

## 📚 What I Learned

- Handling user input with `io::stdin`
- Parsing strings to numbers safely
- Writing conditional logic with `if` and nested `if`
- Using mutable variables and thinking about ownership
- Not overcomplicating things that are actually simple 😅

---

## 💡 Next Steps

- Replace `if` chains with `match` expressions
- Refactor logic into functions
- Handle invalid input more gracefully
- Add replay option when game ends

---

## 👨‍💻 Author

Built by Karabo.  
Learning one project at a time. 🧠⚙️
