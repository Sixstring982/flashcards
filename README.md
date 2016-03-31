# Flash Cards
A simple flash card runner written in Rust.

### Usage
`flashcards inputfilename`

Just run `flashcards` with your filename as input. It will shuffle all
input lines (described below), then show you the front of a card. Just
hit `Enter` to see the back of the card and get a new one. Press
`Ctrl+C` to quit.

### Input

The input file just needs to be line delimited (however your OS does
that).

Each line simply needs the term on the front of the flash card and the
term on the back, delimited by an equal sign. For instance, the
following is a legitimate flash card file:

<code>
Hitler = Really bad guy, killed lots of people.
</code>
<br>
<code>
Thomas Jefferson = Founding Father of the USA.
</code>

### Issues

This thing is really simple and should just work anywhere you can
compile it. Add any issues you find to the Issues tab above as you
would in any other repo.