# Just some simple tests to see how this works
whoami > whoami_1.txt
whoami > /home/codespace/whoami_2.txt

# Install neovim 
# sudo add-apt-repository ppa:neovim-ppa/stable
# sudo apt-get update
# sudo apt-get install neovim -Y

# Install: https://github.com/junegunn/vim-plug
# sh -c 'curl -fLo "${XDG_DATA_HOME:-$HOME/.local/share}"/nvim/site/autoload/plug.vim --create-dirs \
#     https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim'

#    13  nvim 
#    14  cd ~/
#    15  ls -ah
#    16  ls -lah
#    17  cd .vim
#    18  ls
#    19  cd autoload/
#    20  ls
#    21  cd ~
#    22  rm -rfv .vim 
#    23  nvim 
#    24  nvim +PlugInstall
#    25  nvim 
#    26  nvim ~/.config/nvim/lsp-config.lua 
#    27  cd ~/.config/
#    28  git status
#    29  git commit nvim/lsp-config.lua -m "Fixed deprecation warning lsp-config.lua" 
#    30  git push 
#    31  sudo git push 
#    32  gh
#    33  gh auth 
#    34  gh auth logn 
#    35  gh auth login 
#    36  echo ${GITHUB_TOKEN}
#    37  clear
#    38  history 
#    39  history >> /workspaces/advent-of-code-2021/.devcontainer/on_create_command.sh 
