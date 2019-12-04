%: src/day%.rs data/%.in
	cargo run --release --bin=day$@ data/$@.in
