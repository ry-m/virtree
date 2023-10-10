# Quick virtual file trees in ASCII 
#### (🚨 Extremely niche project alert!)

Virtree is a command-line tool for creating virtual file systems on the fly and generating an ASCII representation of the tree.

## Example input

```
virtree empty/ folder1/[file1.txt file2.txt] folder2/file3.txt
```
**Output:**
```
.
├── empty
├── folder1
│   ├── file1.txt
│   └── file2.txt
└── folder2
    └── file3.txt
```
## Usage 
Virtree can be used with command line arguments, like the example shown above. But for larger and more complex virtual directory structures, the [builder mode](#builder-mode) is recommended. 

### Input 
#### Creating virtual folders
Folders are created with the folder name followed by a forward slash. Back slashes are also accepted. 

**Example**:
```
my_folder/ 
```
#### Creating virtual subfolders
Subfolders are created by appending the path of the parent folder with the folder name followed by a forward slash. 

**Example**:
```
my_folder/my_subfolder/
```
#### Creating virtual files
Files are created in the same way as folders, but the forward slash at the end is removed, and typically includes a file extension. 

**Example**:
```
file.txt
my_folder/file.txt
my_folder/my_subfolder/run.sh
```

### Builder mode 
Builder mode is useful for prototyping large virtual directory structures in real time through an the interactive shell. The tree is updated in real time and commands can be used to modify the tree state. 

Run `virtree` with no arguments to start builder mode. 

#### Builder mode UI
# (Coming Soon)