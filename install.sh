set -e

OS=''
case `uname` in
  Darwin*)  OS="macos" ;;
  Linux*)   OS="linux" ;;
  *)        echo "unknown os: $OSTYPE" && exit 1 ;;
esac

ARCH=`uname -m`
case "$ARCH" in
  ix86*|x86_64*)    ARCH="x86_64" ;;
  arm64*|aarch64*)  ARCH="aarch64" ;;
  *)                echo "unknown arch: $ARCH" && exit 1 ;;
esac

export TAG=v0.1.0
export BINARYEN_TAG="version_116"

curl -L -o /tmp/extism-py.tar.gz "https://github.com/extism/python-pdk/releases/download/$TAG/extism-py-$ARCH-$OS-$TAG.tar.gz"

tar xzf /tmp/extism-py.tar.gz
mkdir -p $HOME/.local/bin $HOME/.local/share/extism-py
mv extism-py/bin/extism-py $HOME/.local/bin/extism-py
rm -rf $HOME/.local/share/extism-py
mv extism-py/share/extism-py $HOME/.local/share
chmod +x $HOME/.local/bin/extism-py
rm -r /tmp/extism-py.tar.gz extism-py

echo "extism-py installed to $HOME/.local/bin/extism-py"

if ! which "wasm-merge" > /dev/null || ! which "wasm-opt" > /dev/null; then
  echo 'Missing binaryen tool(s)'

  # binaryen use arm64 instead where as extism-py uses aarch64 for release file naming
  case "$ARCH" in
    aarch64*)  ARCH="arm64" ;;
  esac

  # matches the case where the user installs extism-pdk in a Linux-based Docker image running on mac m1
  # binaryen didn't have arm64 release file for linux 
  if [ $ARCH = "arm64" ] && [ $OS = "linux" ]; then
    ARCH="x86_64"
  fi

  if [ $OS = "macos" ]; then
    echo "Installing binaryen and wasm-merge using homebrew"
    brew install binaryen
  else
    if [ ! -e "binaryen-$BINARYEN_TAG-$ARCH-$OS.tar.gz" ]; then
      echo 'Downloading binaryen...'
      curl -L -O "https://github.com/WebAssembly/binaryen/releases/download/$BINARYEN_TAG/binaryen-$BINARYEN_TAG-$ARCH-$OS.tar.gz"
    fi
    rm -rf 'binaryen' "binaryen-$BINARYEN_TAG"
    tar xf "binaryen-$BINARYEN_TAG-$ARCH-$OS.tar.gz"
    mv "binaryen-$BINARYEN_TAG"/ ./binaryen
    if ! which 'wasm-merge' > /dev/null; then
      echo "Installing wasm-merge..."
      mv binaryen/bin/wasm-merge ~/.local/bin/wasm-merge
    else
      echo "wasm-merge is already installed"
    fi
    if ! which 'wasm-opt' > /dev/null; then
      echo "Installing wasm-opt..."
      mv binaryen/bin/wasm-opt ~/.local/bin/wasm-opt
    else
      echo "wasm-opt is already installed"
    fi
  fi
  rm -rf ./binaryen
else
  echo "wasm-merge and wasm-opt are already installed"
fi
