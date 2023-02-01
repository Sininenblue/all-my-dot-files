if status is-interactive
	fish_vi_key_bindings
	fish_add_path "$HOME/bin"
	fish_add_path "$HOME/.cargo/bin"

	set -gx EDITOR nvim

	alias lf "lfrun"
end
