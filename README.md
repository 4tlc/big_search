# Big Search

Big Search is a command-line tool that searches all text in a given folder/file for a given target phrase, printing all files that contain the phrase.

### How to Use From Command Line
##### Basic
```
bs path target
```
Read as search *this* for *this*
**Path:** Either a folder or file. If it is a folder all subdirectories will be searched (depth is unlimited).
**Target:** Target phrase that is searched for. Use *\n* to indicate new line. The *\* and *n* can be seperate characters, *bs* will condense it to the new line character.

##### Possible Flags
**-n:** Do not calculate the size of the area to be searched. This doesn't change the results. The size of the area is only used to show how close to completion the algorithm is.

### ToDo
* [ ] Implement replace functionality
* [ ] Add flags to ignore folders
* [ ] Add flags to specify depth
* [ ] Rework errors so printed errors have no extra info from rust
* [x] Status tracker, shows searched and total
* [x] Add flags to not compute size
* [x] Add flags to not compute size
* [x] Start benching against grep
