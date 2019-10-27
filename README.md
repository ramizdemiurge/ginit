## Install
You need cargo and rust compiler
```shell
sudo apt install cargo rustc -y
```

Then install:
```shell
git clone git@github.com:ramizdemiurge/ginit.git
cd ginit
cargo build
sudo cp target/debug/ginit /bin/ginit
sudo chmod +x /bin/ginit
```

## Usage
```shell
USAGE: ginit #initiates git
USAGE: ginit [NAME] [EMAIL] #initiates git and sets name and email

      --help     display this help and exit
      --version  output version information and exit

Example:
  ginit Linus torwalds@email
```
