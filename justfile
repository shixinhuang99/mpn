alias rp := release-pr
alias pt := push-tag

default:
	just --list --unsorted

toolchain-vers:
	rustup -V
	rustc -V
	cargo -V
	cargo fmt --version
	cargo clippy -V

fmt:
	cargo fmt
	taplo fmt

lint: fmt
	cargo clippy --all-features

check:
	cargo fmt --check
	taplo fmt --check
	cargo clippy --all-features -- -D warnings

release-pr tag:
	git checkout -b "release-{{tag}}"
	git cliff --tag {{tag}} -o CHANGELOG.md
	cargo set-version {{tag}}
	git commit -am "chore(release): {{tag}}"
	git push --set-upstream origin release-{{tag}}

push-tag tag:
	git tag {{tag}}
	git push origin {{tag}}

rslab:
	cargo run -p lab

jslab:
	node ./lab/lab.js

test *args:
	cargo test {{args}}

test-config:
	cargo insta test --review -p mpn_config
