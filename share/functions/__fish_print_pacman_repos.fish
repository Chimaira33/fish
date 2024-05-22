function __fish_print_pacman_repos --description "Print the repositories configured for arch's pacman package manager"
  string match -er "\[.*\]" </data/data/com.termux/files/usr/etc/pacman.conf | string match -r -v "^#|options" | string replace -ra "\[|\]" ""
end
