function __fish_complete_user_ids --description "Complete user IDs with user name as description"
  if command -sq getent
    getent passwd | string replace -f -r '^([[:alpha:]_][^:]*):[^:]*:(\d+).*' '$2\t$1'
  else if test -r /data/data/com.termux/files/usr/etc/passwd
    string replace -f -r '^([[:alpha:]_][^:]*):[^:]*:(\d+).*' '$2\t$1' </data/data/com.termux/files/usr/etc/passwd
  end
end
