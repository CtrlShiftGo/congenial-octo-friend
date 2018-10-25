# Congenial Octo Friend
A simple logging program written in Haskell for the purpose of preparing for my comparative programming midterm and also to help me remember when and where I met my friends.

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
./friend --add 'friend name'
'friend name' has been added on 2018-10-22
```

Adding new friend with postdated meeting date
```
TODO
```

Adding new friend with note
```
TODO
```

Adding new friend with note and postdated meeting date
```
TODO
```

Lookup friend name
```
./friend 'friend name'
'friend name'
First met on: 2018-10-22
Note: This is the note.
```

Lookup date
```
TODO
```
Lookup date range
```
TODO
```

## Storage Format
Friend logging is stored in a a simple CSV file.
| name | date | notes | date added |
|---|---|-|-|
| Hatsune Miku | 2018-08-01 | vocaloid; friends with Kagamine Rin | 2018-10-25 |
