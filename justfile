# @ ------------------
# @ cargo fmt and lint
# @ ------------------

clean:
  cargo clean

fmt: clean
  rustfmt --config-path . ./src/main.rs

lint: fmt
  rustfmt --check ./src/main.rs

build: clean
  cargo build --release

strip: build
  strip ./target/release/walk-git

run: strip
  RUST_LOG=debug cargo run --release

install: strip
  cargo install --path .

# @ ----------------
# @ git quick commit
# @ ----------------

status:
  git status

add:status
  git add .

commit: add
  git commit -m "quick commit"

push: commit
  git push

graph: 
  git log --graph --oneline master
