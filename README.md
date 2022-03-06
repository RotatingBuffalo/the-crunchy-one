# What does this bot do?
It's for making bets and establishing dominance in the ruthless world of Cinnamon Toast Capitalism.
These bets are not integrated in any way into an external system to back them up, and work entirely on basis of honor on part of the person making the bet. ***This shouldn't really be used for anything more serious than a server where you goof off with friends.***

As I am a very lazy programmer, many of these commands will have syntax requirements from the end user that you may not expect from a discord bot. Command parameters may have to be sent enclosed in parenthesis, or quotation marks. You can use `\` as an escape character if you need to use quotes inside of a string.

`!makebet "quotation mark example" "I'm putting \"quotation marks\" inside of my quotation marks!"`

(Also a few commands I wanted to add purely for my own convenience.)
# Commands? Ayo?
These commands can be disabled or permission locked in commands.json
1. User Commands.
    - `!help {subcommand}` -> you'll never guess what this one does.
        - `RotatingBuffalo: !help`
        - `The Craziest Square: {list of commands}`
    - `!balance` -> returns your balance.
        - `RotatingBuffalo: !balance`
        - `The Crunchiest One: You have 1042 crazy squares.`
    - `!makebet {duration:minutes} {name} {description}` -> makes a bet that people can then use `!placebet` on to place their own wagers.
        - `RotatingBuffalo: !makebet 30 "RIPBOZO" "Oliver's going to die on Mithrix"`
    - `!placebet {for/against} {bet_name} {int amount}` -> Puts a wager on the result of a made bet.
        - `RotatingBuffalo: !placebet against RIPBOZO 200`
    - `!endbet {bet} {result (for/against)}`
        - `!endbet RIPBOZO for`
    - `!makevote {OPTIONS} {message_contents}` -> makes a message with the emotes in "OPTIONS" as message reacts to be used as vote choices.
        - `RotatingBuffalo: !makevote (✅, ❌) "Subject Conner to *the whimsy.*"`
    - `!color {HEXCODE}` -> makes a new role with the specified color and gives it to the user.
        - `RotatingBuffalo: !color "#FF00AE"`
2. Admin Commands
     - `!clearcolors {force}` -> removes color roles not in use, or if specified, all color roles.
        - `RotatingBuffalo: !clearcolors force`
        - `The Crunchiest One: All color roles successfully cleared.` 