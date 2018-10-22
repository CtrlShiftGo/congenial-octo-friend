## Options
* Add `--add`
    * Alias: `-a`
    * Adds parameter friend
* Note `--note`
    * Alias: `-n`
    * Adds a note to the friend. Will be shown on lookup.
    * Used on lookup will add an additional note.
* Date `--date`
    * Alias: `-d`
    * On add mode will add friend with new date entry.
    * On lookup mode will search for friends made on that day.
* Daterange `--daterange`
    * On lookup will search for friends made within (inclusive) the date range
## Example Behaviour
Adding new friend
```
py friend.py --add 'friend name'
'friend name' has been added on 2018-10-22
```

Adding new friend with postdated meeting date


Adding new friend with note
Adding new friend with note and postdated meeting date
Lookup friend name
```
py friend.py 'friend name'
'friend name'
First met on: 2018-10-22
Note: This is the note.
```

Lookup date
Lookup date range