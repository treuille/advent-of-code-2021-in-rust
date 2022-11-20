#!/bin/sh

# Install the version of Bundler.
echo "/home/codespace/lil_test_2.txt"
# touch /home/codespace/lil_test_2.txt

# if [ -f Gemfile.lock ] && grep "BUNDLED WITH" Gemfile.lock > /dev/null; then
#     cat Gemfile.lock | tail -n 2 | grep -C2 "BUNDLED WITH" | tail -n 1 | xargs gem install bundler -v
# fi
# 
# # If there's a Gemfile, then run `bundle install`
# # It's assumed that the Gemfile will install Jekyll too
# if [ -f Gemfile ]; then
#     bundle install
# else
#     # If there's no Gemfile, install Jekyll
#     sudo gem install jekyll