function __fish_shared_key_bindings -d "Bindings shared between emacs and vi mode"
  set -l bind
  # These are some bindings that are supposed to be shared between vi mode and default mode.
  # They are supposed to be unrelated to text-editing (or movement).
  # This takes $argv so the vi-bindings can pass the mode they are valid in.

  if contains -- -h $argv
    or contains -- --help $argv
    echo "Sorry but this function doesn't support -h or --help" >&2
    return 1
  end

  bind --preset $argv ctrl-y yank
  or return # protect against invalid $argv
  bind --preset $argv alt-y yank-pop

  # Left/Right arrow
  bind --preset $argv right forward-char
  bind --preset $argv left backward-char

  # Ctrl-left/right - these also work in vim.
  bind --preset $argv ctrl-right forward-word
  bind --preset $argv ctrl-left backward-word

  # Interaction with the system clipboard.
  bind --preset $argv ctrl-x fish_clipboard_copy
  bind --preset $argv ctrl-v fish_clipboard_paste

  bind --preset $argv escape cancel
  bind --preset $argv tab complete
  # shift-tab does a tab complete followed by a search.
  bind --preset $argv shift-tab complete-and-search
  bind --preset $argv shift-delete history-pager-delete or backward-delete-char

  bind --preset $argv down down-or-search
  bind --preset $argv up up-or-search

  bind --preset $argv shift-right forward-bigword
  bind --preset $argv shift-left backward-bigword

  bind --preset $argv ctrl-l clear-screen
  bind --preset $argv ctrl-c cancel-commandline
  bind --preset $argv ctrl-u backward-kill-line
  bind --preset $argv ctrl-p backward-kill-path-component
  bind --preset $argv end end-of-line
  bind --preset $argv home beginning-of-line

  bind --preset $argv ctrl-d delete-or-exit

  # Make it easy to turn an unexecuted command into a comment in the shell history. Also,
  # remove the commenting chars so the command can be further edited then executed.
  bind --preset $argv alt-# __fish_toggle_comment_commandline

  bind --preset $argv alt-v edit_command_buffer
  if not set -l index (contains --index -- -M $argv)
    or test $argv[(math $index + 1)] = insert

    # This is the default binding, i.e. the one used if no other binding matches
    bind --preset $argv "" self-insert
    or exit # protect against invalid $argv

    # Space and other command terminators expands abbrs _and_ inserts itself.
    bind --preset $argv space self-insert expand-abbr
    bind --preset $argv ";" self-insert expand-abbr
    bind --preset $argv "|" self-insert expand-abbr
    bind --preset $argv "&" self-insert expand-abbr
    bind --preset $argv ">" self-insert expand-abbr
    bind --preset $argv "<" self-insert expand-abbr
    bind --preset $argv ")" self-insert expand-abbr # Closing a command substitution.
    bind --preset $argv alt-enter "commandline -i \n"
    bind --preset $argv enter execute
  end
end
