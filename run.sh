trap "kill -- -$$" INT EXIT

./websocketd --port="8080" ./target/snippets \
& google-chrome index.html
