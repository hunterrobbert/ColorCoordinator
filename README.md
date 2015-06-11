# ColorCoordinator
A file generator, written in rust, which generates dotfiles and firefox stylish css according to a supplied text file with 16 HEX color codes

## Description

Really just trying to make something useful while attempting to learing rust.  

ColorCoordinator uses dotfiles without color constants set, writes in the constants according to the chosen color scheme, and places the finished files
in the appropriate place. I made this on Arch for use themeing i3, urxvt, dmenu, vim and firefox.  It's quite specific to my individual setup and 
I haven't gotten a chance think of a way to make it more ... setup agnostic?

At the moment it generatees the following files:

.Xresources
.i3/config
.i3/i3status.config
.i3/dmenu_run_themed
.vim/colors/coordinated.vim
coordinated.css

