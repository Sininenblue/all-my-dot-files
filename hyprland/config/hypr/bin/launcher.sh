#!/bin/fish

# set program $(bash -c 'compgen -c | grep -v fzf | sort -u | fzf --layout=reverse')
# command $program > /dev/null 2>&1

# ls /usr/share/applications/ | grep -v fzf | sort -u | fzf --layout=reverse

set program $(complete -C ""| grep -v fzf | sort -u | fzf --layout=reverse | awk '{print $1}')
gtk-launch $program
