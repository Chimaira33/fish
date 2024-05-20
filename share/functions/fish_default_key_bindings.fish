function fish_default_key_bindings -d "emacs-like key binds"
  set -l bind
  if contains -- -h $argv
    or contains -- --help $argv
    echo "Sorry but this function doesn't support -h or --help"
    return 1
  end

  if not set -q argv[1]
    bind --erase --all --preset # clear earlier bindings, if any
    if test "$fish_key_bindings" != fish_default_key_bindings
      # Allow the user to set the variable universally
      set -l scope
      set -q fish_key_bindings
      or set scope -g
      true
      # We try to use `set --no-event`, but to avoid leaving the user without bindings
      # if they run this with an older version we fall back on setting the variable
      # with an event.
      if ! set --no-event $scope fish_key_bindings fish_default_key_bindings 2>/dev/null
        # This triggers the handler, which calls us again
        set $scope fish_key_bindings fish_default_key_bindings
        # unless the handler somehow doesn't exist, which would leave us without bindings.
        # this happens in no-config mode.
        functions -q __fish_reload_key_bindings
        and return
      else
        # (we need to set the bind mode to default)
        set --no-event fish_bind_mode default
      end
    end
  end

  # Silence warnings about unavailable keys. See #4431, 4188
  if not contains -- -s $argv
    set argv -s $argv
  end

  # These are shell-specific bindings that we share with vi mode.
  __fish_shared_key_bindings $argv
  or return # protect against invalid $argv

  bind --preset $argv ctrl-k kill-line

  bind --preset $argv right forward-char
  bind --preset $argv left backward-char

  bind --preset $argv delete delete-char
  bind --preset $argv backspace backward-delete-char
  bind --preset $argv shift-backspace backward-delete-char

  bind --preset $argv home beginning-of-line
  bind --preset $argv end end-of-line

  bind --preset $argv ctrl-z undo
  bind --preset $argv ctrl-Z redo

  bind --preset $argv alt-backspace backward-kill-word

  bind --preset $argv alt-\< beginning-of-buffer
  bind --preset $argv alt-\> end-of-buffer

  bind --preset $argv ctrl-r history-pager

  set -e -g fish_cursor_selection_mode
end
