# Just some simple tests to see how this works
whoami > whoami_1.txt
whoami > /home/codespace/whoami_2.txt

# Install neovim 
sudo add-apt-repository -y ppa:neovim-ppa/stable
sudo apt update -y
sudo apt install neovim -y

# Install: https://github.com/junegunn/vim-plug
sh -c 'curl -fLo "${XDG_DATA_HOME:-$HOME/.local/share}"/nvim/site/autoload/plug.vim --create-dirs \
    https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim'

# Install lsd 
cargo install lsd

# Install ripgrep 
cargo install ripgrep

# Install lazygit
LAZYGIT_VERSION=$(curl -s "https://api.github.com/repos/jesseduffield/lazygit/releases/latest" | grep '"tag_name":' |  sed -E 's/.*"v*([^"]+)".*/\1/')
curl -Lo lazygit.tar.gz "https://github.com/jesseduffield/lazygit/releases/latest/download/lazygit_${LAZYGIT_VERSION}_Linux_x86_64.tar.gz"
sudo tar xf lazygit.tar.gz -C /usr/local/bin lazygit
rm -fv lazygit.tar.gz
