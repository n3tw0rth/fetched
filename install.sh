GIT_BRANCH=$(git symbolic-ref --short HEAD)

if [[ "$GIT_BRANCH" == "master" ]]; then
   PROGRAM=fetched
else
   PROGRAM=fetched-dev
fi

cargo build --release
sudo cp ./target/release/fetched /usr/local/bin/$PROGRAM

