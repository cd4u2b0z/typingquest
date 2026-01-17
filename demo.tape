# VHS Demo Tape for TypingQuest
# Run with: vhs demo.tape
# Requires: ttyd, ffmpeg (vhs will prompt to install)

Output demo.gif

Set Shell "zsh"
Set FontFamily "JetBrainsMono Nerd Font"
Set FontSize 16
Set Width 1200
Set Height 700
Set Padding 20
Set Theme "Dracula"
Set TypingSpeed 50ms

# Build and run the game
Type "cargo run --release -q"
Enter
Sleep 3s

# We're at title screen - select "New Game"
Enter
Sleep 1s

# Class selection - pick Wordsmith (first option)
Enter
Sleep 1s

# Tutorial starts - we'll skip through it fast
# Phase 1: Awakening - just press Enter
Enter
Sleep 1.5s

# Phase 2: First Strike - type "echo"
Type "echo"
Sleep 500ms
Enter
Sleep 1.5s

# Phase 3: Combo - type words to build combo
Type "word"
Sleep 300ms
Type " "
Type "power"
Sleep 300ms
Type " "
Type "flow"
Sleep 500ms
Enter
Sleep 1.5s

# Phase 4: Choice - press Enter
Enter
Sleep 1s

# Phase 5: Discovery - press Enter to finish tutorial
Enter
Sleep 2s

# Now we're in the dungeon! Show the map briefly
Sleep 2s

# Enter first room (combat)
Type "e"
Sleep 2s

# Combat! Type the words that appear
# We'll just type some generic combat words
Type "strike"
Sleep 200ms
Type " "
Type "swift"
Sleep 200ms
Type " "
Type "blade"
Sleep 200ms
Type " "
Type "attack"
Sleep 200ms
Type " "
Type "power"
Sleep 500ms

# Show the combo building
Type " "
Type "fierce"
Sleep 200ms
Type " "
Type "warrior"
Sleep 200ms
Type " "
Type "battle"
Sleep 200ms
Type " "
Type "honor"
Sleep 200ms
Type " "
Type "glory"
Sleep 1s

# Let it sit for a moment to show the combat UI
Sleep 2s

# Press ? to show help overlay
Type "?"
Sleep 2s

# Close help
Type "?"
Sleep 1s

# Escape to flee and show dungeon
Escape
Sleep 2s

# Show stats
Type "s"
Sleep 2s

# Back to dungeon
Escape
Sleep 1s

# Show inventory
Type "i"
Sleep 2s

# Back 
Escape
Sleep 1s

# Final pause on dungeon view
Sleep 2s

# Quit
Ctrl+C
Sleep 500ms
